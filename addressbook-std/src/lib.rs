use std::{
    io::{Read, Result, Write},
    net::TcpStream,
};

use addressbook::{carddav::Config, tcp};

#[derive(Debug)]
pub struct Connector {
    stream: TcpStream,
}

impl Connector {
    pub fn connect(config: &Config) -> Result<Self> {
        let addr = (config.hostname.as_str(), config.port);
        let stream = TcpStream::connect(addr)?;
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
