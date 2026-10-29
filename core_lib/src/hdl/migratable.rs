use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, DuplexStream, ReadBuf};
use tokio::net::TcpStream;

/// Transport used by an inbound Quick Share session.
///
/// Modern Android receivers may start over the BLE weave socket and later
/// migrate the same encrypted session to Wi-Fi LAN. Keeping the transport
/// behind one AsyncRead/AsyncWrite type lets the existing hardened state
/// machine preserve its crypto keys and sequence counters across that swap.
#[derive(Debug)]
pub enum MigratableStream {
    Ble(DuplexStream),
    Tcp(TcpStream),
}

impl AsyncRead for MigratableStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Ble(stream) => Pin::new(stream).poll_read(cx, buf),
            Self::Tcp(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for MigratableStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            Self::Ble(stream) => Pin::new(stream).poll_write(cx, buf),
            Self::Tcp(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Ble(stream) => Pin::new(stream).poll_flush(cx),
            Self::Tcp(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Ble(stream) => Pin::new(stream).poll_shutdown(cx),
            Self::Tcp(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    #[tokio::test]
    async fn ble_transport_round_trip() {
        let (mut peer, transport) = tokio::io::duplex(128);
        let mut stream = MigratableStream::Ble(transport);

        peer.write_all(b"phone-to-linux").await.unwrap();

        let mut inbound = [0_u8; 14];
        stream.read_exact(&mut inbound).await.unwrap();
        assert_eq!(&inbound, b"phone-to-linux");

        stream.write_all(b"linux-to-phone").await.unwrap();
        stream.flush().await.unwrap();

        let mut outbound = [0_u8; 14];
        tokio::time::timeout(
            std::time::Duration::from_millis(250),
            peer.read_exact(&mut outbound),
        )
        .await
        .expect("write was not forwarded to the underlying BLE stream")
        .unwrap();
        assert_eq!(&outbound, b"linux-to-phone");
    }

    #[tokio::test]
    async fn ble_shutdown_is_forwarded_to_the_underlying_stream() {
        let (mut peer, transport) = tokio::io::duplex(128);
        let mut stream = MigratableStream::Ble(transport);

        stream.shutdown().await.unwrap();

        let mut byte = [0_u8; 1];
        let read =
            tokio::time::timeout(std::time::Duration::from_millis(250), peer.read(&mut byte))
                .await
                .expect("shutdown was not forwarded to the underlying BLE stream")
                .unwrap();

        assert_eq!(read, 0);
    }
    async fn tcp_pair() -> (tokio::net::TcpStream, tokio::net::TcpStream) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let client = tokio::net::TcpStream::connect(address).await.unwrap();
        let (server, _) = listener.accept().await.unwrap();
        (client, server)
    }

    #[tokio::test]
    async fn ble_half_close_preserves_inbound_read_after_outbound_eof() {
        // Arrange: the application has sent its final outbound control bytes.
        let (mut peer, transport) = tokio::io::duplex(128);
        let mut stream = MigratableStream::Ble(transport);

        // Act: close only our write side and let the peer drain the message.
        stream.write_all(b"goodbye").await.unwrap();
        stream.flush().await.unwrap();
        stream.shutdown().await.unwrap();

        // Assert: the peer sees the complete message and then EOF.
        let mut message = [0_u8; 7];
        peer.read_exact(&mut message).await.unwrap();
        assert_eq!(&message, b"goodbye");
        assert_eq!(peer.read(&mut message).await.unwrap(), 0);

        // The peer can still send a final control response after local EOF.
        peer.write_all(b"ack").await.unwrap();
        let mut response = [0_u8; 3];
        stream.read_exact(&mut response).await.unwrap();
        assert_eq!(&response, b"ack");
    }

    #[tokio::test]
    async fn tcp_half_close_preserves_inbound_read_after_outbound_eof() {
        // Arrange: a local loopback pair exercises the real TCP variant.
        let (mut peer, transport) = tcp_pair().await;
        let mut stream = MigratableStream::Tcp(transport);

        // Act: flushing and closing the write half must not close the read half.
        stream.write_all(b"goodbye").await.unwrap();
        stream.flush().await.unwrap();
        stream.shutdown().await.unwrap();

        // Assert: the peer drains all sent data before observing EOF.
        let mut message = [0_u8; 7];
        peer.read_exact(&mut message).await.unwrap();
        assert_eq!(&message, b"goodbye");
        assert_eq!(peer.read(&mut message).await.unwrap(), 0);

        peer.write_all(b"ack").await.unwrap();
        let mut response = [0_u8; 3];
        stream.read_exact(&mut response).await.unwrap();
        assert_eq!(&response, b"ack");
    }

    #[tokio::test]
    async fn tcp_transport_round_trip() {
        let (mut peer, transport) = tcp_pair().await;
        let mut stream = MigratableStream::Tcp(transport);

        peer.write_all(b"phone-to-linux").await.unwrap();

        let mut inbound = [0_u8; 14];
        stream.read_exact(&mut inbound).await.unwrap();
        assert_eq!(&inbound, b"phone-to-linux");

        stream.write_all(b"linux-to-phone").await.unwrap();
        stream.flush().await.unwrap();

        let mut outbound = [0_u8; 14];
        peer.read_exact(&mut outbound).await.unwrap();
        assert_eq!(&outbound, b"linux-to-phone");
    }

    #[tokio::test]
    async fn tcp_shutdown_is_forwarded_to_the_underlying_stream() {
        let (mut peer, transport) = tcp_pair().await;
        let mut stream = MigratableStream::Tcp(transport);

        stream.shutdown().await.unwrap();

        let mut byte = [0_u8; 1];
        let read = peer.read(&mut byte).await.unwrap();
        assert_eq!(read, 0);
    }
}
