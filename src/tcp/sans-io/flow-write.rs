use super::{Flow, Io, WroteBytes};

#[derive(Debug)]
pub struct WriteFlow {
    count: Option<usize>,
}

impl WriteFlow {
    pub fn new() -> Self {
        Self { count: None }
    }
}

impl Iterator for WriteFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count.is_some() {
            None
        } else {
            Some(Io::Write)
        }
    }
}

impl Flow for WriteFlow {}

impl WroteBytes for WriteFlow {
    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.count.replace(count);
    }
}
