#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use addressbook::carddav::tcp;
use native_tls::{TlsConnector, TlsStream};
use thiserror::Error;
use tracing::{instrument, trace};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    NativeTlsError(#[from] native_tls::Error),
    #[error(transparent)]
    NativeTlsHandshakeError(#[from] native_tls::HandshakeError<TcpStream>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Connector {
    stream: TlsStream<TcpStream>,
}

impl Connector {
    pub fn connect(hostname: impl AsRef<str>, port: u16) -> Result<Self> {
        let conn = TlsConnector::new()?;
        let sock = TcpStream::connect((hostname.as_ref(), port))?;
        let stream = conn.connect(hostname.as_ref(), sock)?;

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
