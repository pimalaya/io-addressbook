use std::{
    io::{Read, Write},
    net::TcpStream,
};

use addressbook::tcp;
use native_tls::{TlsConnector, TlsStream};
use thiserror::Error;

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
