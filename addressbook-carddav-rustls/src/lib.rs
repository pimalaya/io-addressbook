#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use addressbook::carddav::tcp;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use thiserror::Error;
use tracing::{instrument, trace};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RustlsError(#[from] rustls::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum CryptoProvider {
    Default,
    #[cfg(feature = "aws-lc")]
    AwsLc,
    #[cfg(feature = "ring")]
    Ring,
}

#[derive(Debug)]
pub struct Connector {
    stream: StreamOwned<ClientConnection, TcpStream>,
}

impl Connector {
    pub fn connect(hostname: impl AsRef<str>, port: u16, crypto: &CryptoProvider) -> Result<Self> {
        let hostname = hostname.as_ref();

        let client_config = match crypto {
            CryptoProvider::Default => {
                use rustls_platform_verifier::ConfigVerifierExt;
                ClientConfig::with_platform_verifier()
            }
            #[cfg(feature = "aws-lc")]
            CryptoProvider::AwsLc => {
                use rustls::crypto::aws_lc_rs::default_provider;
                use rustls_platform_verifier::BuilderVerifierExt;

                ClientConfig::builder_with_provider(Arc::new(default_provider()))
                    .with_safe_default_protocol_versions()?
                    .with_platform_verifier()
                    .with_no_client_auth()
            }
            #[cfg(feature = "ring")]
            CryptoProvider::Ring => {
                use rustls::crypto::ring::default_provider;
                use rustls_platform_verifier::BuilderVerifierExt;

                ClientConfig::builder_with_provider(Arc::new(default_provider()))
                    .with_safe_default_protocol_versions()?
                    .with_platform_verifier()
                    .with_no_client_auth()
            }
        };

        let server_name = hostname.to_owned().try_into().unwrap();
        let addr = (hostname, port);
        let conn = ClientConnection::new(Arc::new(client_config), server_name)?;
        let sock = TcpStream::connect(addr)?;
        let stream = StreamOwned::new(conn, sock);

        Ok(Self { stream })
    }

    #[instrument(skip_all)]
    pub fn execute<F: AsMut<tcp::State>>(&mut self, flow: &mut F, io: tcp::Io) -> Result<()> {
        let state = flow.as_mut();

        match io {
            tcp::Io::Read => self.read(state),
            tcp::Io::Write => self.write(state),
        }
    }

    #[instrument(skip_all)]
    fn read(&mut self, state: &mut tcp::State) -> Result<()> {
        let buffer = state.get_buffer_mut();
        let read_bytes_count = self.stream.read(buffer)?;
        trace!("read bytes {read_bytes_count}/{}", buffer.len());
        state.set_read_bytes_count(read_bytes_count);
        Ok(())
    }

    #[instrument(skip_all)]
    fn write(&mut self, state: &mut tcp::State) -> Result<()> {
        let buffer = state.get_buffer();
        let wrote_bytes_count = self.stream.write(buffer)?;
        trace!("wrote bytes {wrote_bytes_count}/{}", buffer.len());
        state.set_wrote_bytes_count(wrote_bytes_count);
        Ok(())
    }
}
