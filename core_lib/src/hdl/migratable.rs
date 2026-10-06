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
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
        peer.read_exact(&mut outbound).await.unwrap();
        assert_eq!(&outbound, b"linux-to-phone");
    }
}
