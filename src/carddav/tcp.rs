#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    Read,
    Write,
}

#[derive(Debug)]
pub struct State {
    pub(crate) read_buffer: Vec<u8>,
    pub(crate) read_bytes_count: usize,
    pub(crate) write_buffer: Vec<u8>,
    pub(crate) wrote_bytes_count: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            read_buffer: vec![0; 512],
            read_bytes_count: 0,
            write_buffer: vec![],
            wrote_bytes_count: 0,
        }
    }

    pub fn get_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.read_buffer
    }

    pub fn set_read_bytes_count(&mut self, count: usize) {
        self.read_bytes_count = count;
    }

    pub fn get_buffer(&mut self) -> &[u8] {
        &self.write_buffer
    }

    pub fn set_wrote_bytes_count(&mut self, count: usize) {
        self.wrote_bytes_count = count;
    }
}
