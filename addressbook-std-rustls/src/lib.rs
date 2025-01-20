use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    sync::Arc,
};

use addressbook::{carddav::Config, tcp};
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;

#[derive(Debug)]
pub struct Connector {
    stream: StreamOwned<ClientConnection, TcpStream>,
}

impl Connector {
    pub fn connect(config: &Config) -> Result<Self> {
        let client_config = Arc::new(ClientConfig::with_platform_verifier());
        let server_name = config.hostname.clone().try_into().unwrap();
        let addr = (config.hostname.as_str(), config.port);
        let conn = ClientConnection::new(client_config, server_name).unwrap();
        let sock = TcpStream::connect(addr)?;
        let stream = StreamOwned::new(conn, sock);

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
