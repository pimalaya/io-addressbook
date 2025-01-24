#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

use std::{
    io::{Read, Result, Write},
    net::TcpStream,
};

use addressbook::tcp;

#[derive(Debug)]
pub struct Connector {
    stream: TcpStream,
}

impl Connector {
    pub fn connect(hostname: impl AsRef<str>, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((hostname.as_ref(), port))?;
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
