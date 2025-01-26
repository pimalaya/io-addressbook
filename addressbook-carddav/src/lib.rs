#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

use std::{
    io::{Read, Result, Write},
    net::TcpStream,
};

use addressbook::carddav::tcp;
use tracing::{instrument, trace};

#[derive(Debug)]
pub struct Connector {
    stream: TcpStream,
}

impl Connector {
    pub fn connect(hostname: impl AsRef<str>, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((hostname.as_ref(), port))?;
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
