use std::{
    io::{Read, Write},
    net::TcpStream,
};

use addressbook::{carddav::Config, tcp};
use native_tls::{TlsConnector, TlsStream};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    TlsError(#[from] native_tls::Error),
    #[error(transparent)]
    TlsHandshakeError(#[from] native_tls::HandshakeError<TcpStream>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Connector {
    stream: TlsStream<TcpStream>,
}

impl Connector {
    pub fn connect(config: &Config) -> Result<Self> {
        let addr = (config.hostname.as_str(), config.port);
        let conn = TlsConnector::new()?;
        let sock = TcpStream::connect(addr)?;
        let stream = conn.connect(&config.hostname, sock)?;

        Ok(Self { stream })
    }

    pub fn read<F: tcp::Read>(&mut self, flow: &mut F) -> Result<()> {
        let buffer = flow.get_buffer_mut();
        let read_bytes_count = self.stream.read(buffer)?;
        flow.set_read_bytes_count(read_bytes_count);
        Ok(())
    }

    pub fn write<F: tcp::Write>(&mut self, flow: &mut F) -> Result<()> {
        let buffer = flow.get_buffer();
        let wrote_bytes_count = self.stream.write(buffer)?;
        flow.set_wrote_bytes_count(wrote_bytes_count);
        Ok(())
    }
}
