use super::{Flow, Io, ReadBytes};

pub struct ReadFlow {
    buffer: Vec<u8>,
    count: Option<usize>,
}

impl ReadFlow {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; 1024],
            count: None,
        }
    }
}

impl Iterator for ReadFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count.is_some() {
            None
        } else {
            Some(Io::Read)
        }
    }
}

impl Flow for ReadFlow {}

impl ReadBytes for ReadFlow {
    fn read_buffer(&mut self) -> &[u8] {
        &self.buffer
    }

    fn read_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.count.replace(count);
    }
}
