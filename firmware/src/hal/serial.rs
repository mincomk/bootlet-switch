pub trait Serial {
    async fn wait_connection(&mut self);
    async fn write(&mut self, data: &[u8]) -> crate::Result<()>;
    async fn read(&mut self, buffer: &mut [u8]) -> crate::Result<usize>;
}
