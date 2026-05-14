use std::pin::Pin;

use bitsync_storage::async_file_read::AsyncFileRead;
use tokio::io::DuplexStream;

pub fn user_root_directory_name(user_name: &str) -> String {
    if user_name.ends_with('s') {
        format!("{user_name}' Storage")
    } else {
        format!("{user_name}'s Storage")
    }
}

pub enum AsyncStorageItemRead {
    File(AsyncFileRead),
    Directory(DuplexStream),
}

impl tokio::io::AsyncRead for AsyncStorageItemRead {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let self_mut = self.get_mut();

        match self_mut {
            AsyncStorageItemRead::File(inner) => {
                let pinned_inner = Pin::new(inner);

                pinned_inner.poll_read(cx, buf)
            }
            AsyncStorageItemRead::Directory(inner) => {
                let pinned_inner = Pin::new(inner);

                pinned_inner.poll_read(cx, buf)
            }
        }
    }
}
