use std::{
    io::{Read, Write},
    net::TcpStream,
};

use thiserror::Error;

use super::sans_io::{ReadBytes, WroteBytes};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct IoConnector {
    stream: TcpStream,
}

impl IoConnector {
    pub fn connect(host: impl AsRef<str>, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((host.as_ref(), port))?;
        Ok(Self { stream })
    }

    pub fn read<'a, F: ReadBytes>(&mut self, flow: &'a mut F) -> Result<&'a [u8]> {
        let count = self.stream.read(flow.read_buffer_mut())?;
        flow.set_read_bytes_count(count);
        Ok(&flow.read_buffer()[..count])
    }

    pub fn write<'a, F: WroteBytes>(&mut self, flow: &mut F, bytes: &'a [u8]) -> Result<&'a [u8]> {
        let count = self.stream.write(bytes)?;
        flow.set_wrote_bytes_count(count);
        Ok(&bytes[..count])
    }
}
