use tokio::io::{AsyncRead, AsyncReadExt};

/// Retains a partially read prefix when a resumable control branch wins a select.
#[derive(Debug, Default)]
pub(super) struct FrameHeader {
    bytes: [u8; 4],
    filled: usize,
}

impl FrameHeader {
    pub(super) async fn read<R: AsyncRead + Unpin>(
        &mut self,
        stream: &mut R,
    ) -> std::io::Result<[u8; 4]> {
        while self.filled < self.bytes.len() {
            // A single read is cancellation-safe. Commit its progress before the
            // next await, so dropping this future never discards consumed bytes.
            let count = stream.read(&mut self.bytes[self.filled..]).await?;
            if count == 0 {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            self.filled += count;
        }
        self.filled = 0;
        Ok(self.bytes)
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncWriteExt;

    use super::*;

    #[tokio::test]
    async fn given_partial_header_when_read_is_cancelled_then_resuming_retains_every_byte() {
        for split in 0..4 {
            // Arrange
            let expected = [0x12, 0x34, 0x56, 0x78];
            let (mut stream, mut peer) = tokio::io::duplex(16);
            let mut header = FrameHeader::default();
            peer.write_all(&expected[..split]).await.unwrap();
            // Act: polling consumes exactly the available bytes, without sleeps.
            {
                let read = header.read(&mut stream);
                tokio::pin!(read);
                assert!(futures::poll!(read.as_mut()).is_pending());
            }
            peer.write_all(&expected[split..]).await.unwrap();
            // Assert
            assert_eq!(header.read(&mut stream).await.unwrap(), expected);
        }
    }

    #[tokio::test]
    async fn given_repeated_interruptions_when_read_resumes_then_progress_accumulates() {
        // Arrange
        let expected = [0x12, 0x34, 0x56, 0x78];
        let (mut stream, mut peer) = tokio::io::duplex(16);
        let mut header = FrameHeader::default();
        // Act
        for byte in &expected[..3] {
            peer.write_all(&[*byte]).await.unwrap();
            let read = header.read(&mut stream);
            tokio::pin!(read);
            assert!(futures::poll!(read.as_mut()).is_pending());
        }
        peer.write_all(&expected[3..]).await.unwrap();
        // Assert
        assert_eq!(header.read(&mut stream).await.unwrap(), expected);
    }

    #[tokio::test]
    async fn given_coalesced_headers_when_read_consecutively_then_prefix_state_resets() {
        // Arrange
        let first = [0x12, 0x34, 0x56, 0x78];
        let second = [0x87, 0x65, 0x43, 0x21];
        let mut stream = std::io::Cursor::new([first, second].concat());
        let mut header = FrameHeader::default();
        // Act / Assert
        assert_eq!(header.read(&mut stream).await.unwrap(), first);
        assert_eq!(header.read(&mut stream).await.unwrap(), second);
    }

    #[tokio::test]
    async fn given_truncated_header_when_peer_closes_then_unexpected_eof_is_reported() {
        for count in 0..4 {
            // Arrange
            let mut stream = std::io::Cursor::new(vec![0x12; count]);
            let mut header = FrameHeader::default();
            // Act / Assert
            assert_eq!(
                header.read(&mut stream).await.unwrap_err().kind(),
                std::io::ErrorKind::UnexpectedEof
            );
        }
    }

    #[tokio::test]
    async fn given_two_fragmented_frames_when_each_read_is_interrupted_then_headers_never_splice() {
        // Exercise all nonempty split boundaries across consecutive frames.
        let first = [0x10, 0x20, 0x30, 0x40];
        let second = [0xa0, 0xb0, 0xc0, 0xd0];
        for first_split in 1..4 {
            for second_split in 1..4 {
                let (mut stream, mut peer) = tokio::io::duplex(16);
                let mut header = FrameHeader::default();

                peer.write_all(&first[..first_split]).await.unwrap();
                {
                    let pending = header.read(&mut stream);
                    tokio::pin!(pending);
                    assert!(futures::poll!(pending.as_mut()).is_pending());
                }
                peer.write_all(&first[first_split..]).await.unwrap();
                peer.write_all(&second[..second_split]).await.unwrap();
                assert_eq!(header.read(&mut stream).await.unwrap(), first);

                {
                    let pending = header.read(&mut stream);
                    tokio::pin!(pending);
                    assert!(futures::poll!(pending.as_mut()).is_pending());
                }
                peer.write_all(&second[second_split..]).await.unwrap();
                assert_eq!(header.read(&mut stream).await.unwrap(), second);
            }
        }
    }

    #[tokio::test]
    async fn given_io_failure_after_partial_prefix_when_retried_then_partial_bytes_survive() {
        let first = [0x11, 0x22, 0x33, 0x44];
        let next = [0x55, 0x66, 0x77, 0x88];
        for consumed in 1..4 {
            let mut header = FrameHeader::default();
            let mut interrupted =
                std::io::Cursor::new(first[..consumed].to_vec()).chain(FailedRead);
            assert_eq!(
                header.read(&mut interrupted).await.unwrap_err().kind(),
                std::io::ErrorKind::ConnectionReset
            );
            let mut recovered =
                std::io::Cursor::new([first[consumed..].to_vec(), next.to_vec()].concat());
            assert_eq!(header.read(&mut recovered).await.unwrap(), first);
            assert_eq!(header.read(&mut recovered).await.unwrap(), next);
        }
    }

    struct FailedRead;

    impl AsyncRead for FailedRead {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _context: &mut std::task::Context<'_>,
            _buffer: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Err(std::io::ErrorKind::ConnectionReset.into()))
        }
    }

    #[tokio::test]
    async fn given_read_failure_when_prefix_is_incomplete_then_original_error_is_preserved() {
        for count in 0..4 {
            // Arrange: only the external I/O failure is simulated.
            let mut stream = std::io::Cursor::new(vec![0x12; count]).chain(FailedRead);
            let mut header = FrameHeader::default();
            // Act / Assert
            assert_eq!(
                header.read(&mut stream).await.unwrap_err().kind(),
                std::io::ErrorKind::ConnectionReset
            );
        }
    }
}
