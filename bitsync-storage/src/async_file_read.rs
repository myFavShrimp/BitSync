use std::pin::Pin;

pub struct AsyncFileRead(pub(crate) tokio::fs::File);

impl tokio::io::AsyncRead for AsyncFileRead {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let self_mut = self.get_mut();
        let inner = Pin::new(&mut self_mut.0);

        inner.poll_read(cx, buf)
    }
}
