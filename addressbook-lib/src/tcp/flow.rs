pub trait Read {
    fn get_buffer_mut(&mut self) -> &mut [u8];
    fn set_read_bytes_count(&mut self, count: usize);
}

pub trait Write {
    fn get_buffer(&mut self) -> &[u8];
    fn set_wrote_bytes_count(&mut self, count: usize);
}
