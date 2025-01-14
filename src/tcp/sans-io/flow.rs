pub trait Flow: Iterator {}

pub trait WroteBytes: Flow {
    fn set_wrote_bytes_count(&mut self, count: usize);
}

pub trait ReadBytes: Flow {
    fn read_buffer(&mut self) -> &[u8];
    fn read_buffer_mut(&mut self) -> &mut [u8];
    fn set_read_bytes_count(&mut self, count: usize);
}
