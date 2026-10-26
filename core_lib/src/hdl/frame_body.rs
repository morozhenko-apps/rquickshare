use tokio::io::{AsyncRead, AsyncReadExt};

use super::frame_header::FrameHeader;

/// Retains consumed body bytes when an independent socket write wins a select.
#[derive(Debug, Default)]
pub(super) struct FrameBody {
    bytes: Vec<u8>,
    filled: usize,
    has_header: bool,
}

impl FrameBody {
    pub(super) async fn read<R: AsyncRead + Unpin>(
        &mut self,
        header: &mut FrameHeader,
        stream: &mut R,
    ) -> std::io::Result<Vec<u8>> {
        if !self.has_header {
            let size = u32::from_be_bytes(header.read(stream).await?) as usize;
            if size == 0 || size > crate::protocol::SANE_FRAME_LENGTH as usize {
                return Err(std::io::ErrorKind::InvalidData.into());
            }
            self.bytes.resize(size, 0);
            self.has_header = true;
        }
        while self.filled < self.bytes.len() {
            let count = stream.read(&mut self.bytes[self.filled..]).await?;
            if count == 0 {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            self.filled += count;
        }
        self.filled = 0;
        self.has_header = false;
        Ok(std::mem::take(&mut self.bytes))
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncWriteExt;

    use super::*;

    #[tokio::test]
    async fn given_fragmented_body_when_reads_are_interrupted_then_every_byte_and_next_frame_survives(
    ) {
        for split in 0..8 {
            // Arrange
            let (mut socket, mut peer) = tokio::io::duplex(32);
            let mut header = FrameHeader::default();
            let mut body = FrameBody::default();
            let bytes = [4_u32.to_be_bytes().as_slice(), b"data"].concat();
            peer.write_all(&bytes[..split]).await.unwrap();
            // Act
            {
                let read = body.read(&mut header, &mut socket);
                tokio::pin!(read);
                assert!(futures::poll!(read).is_pending());
            }
            peer.write_all(&bytes[split..]).await.unwrap();
            // Assert
            assert_eq!(body.read(&mut header, &mut socket).await.unwrap(), b"data");
            peer.write_all(&bytes).await.unwrap();
            assert_eq!(body.read(&mut header, &mut socket).await.unwrap(), b"data");
        }
    }

    #[tokio::test]
    async fn given_invalid_size_or_truncated_body_when_read_then_framing_error_is_terminal() {
        for size in [
            0,
            crate::protocol::SANE_FRAME_LENGTH as u32 + 1,
            u32::MAX,
            2,
        ] {
            // Arrange
            let bytes = size.to_be_bytes().to_vec();
            let mut stream = std::io::Cursor::new(bytes);
            // Act / Assert
            let error = FrameBody::default()
                .read(&mut FrameHeader::default(), &mut stream)
                .await
                .unwrap_err();
            assert_eq!(
                error.kind(),
                if size == 2 {
                    std::io::ErrorKind::UnexpectedEof
                } else {
                    std::io::ErrorKind::InvalidData
                }
            );
        }
    }
}
