use core::result::Result;
use embassy_net::raw::{RawSocket, RecvError};

#[cfg_attr(test, autospy::autospy)]
#[allow(async_fn_in_trait)]
pub trait Socket<'a> {
    async fn recv(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], RecvError>;
    async fn send(&self, data: &[u8]);
}

impl<'a> Socket<'a> for RawSocket<'_> {
    async fn recv(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], RecvError> {
        let size = self.recv(buffer).await?;
        Ok(&buffer[..size])
    }

    async fn send(&self, data: &[u8]) {
        self.send(data).await
    }
}
