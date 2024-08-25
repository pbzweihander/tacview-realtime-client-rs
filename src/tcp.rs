use crc::{Crc, CRC_32_ISO_HDLC};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream},
    net::{TcpStream, ToSocketAddrs},
};

use crate::error::{Error, Result};

fn hash_password(password: &str) -> String {
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let password_utf16 = password.encode_utf16();
    let mut password_bytes = Vec::<u8>::with_capacity(password.len() * 2);
    for c in password_utf16 {
        password_bytes.push((c >> 0) as u8);
        password_bytes.push((c >> 8) as u8);
    }
    let checksum = CRC.checksum(&password_bytes);
    format!("{checksum:x}")
}

pub async fn connect<A>(addr: A, username: &str, password: &str) -> Result<BufStream<TcpStream>>
where
    A: ToSocketAddrs,
{
    let tcp_stream = TcpStream::connect(addr).await.map_err(Error::TcpConnect)?;
    let tcp_stream = BufStream::new(tcp_stream);
    from_tcp_stream(tcp_stream, username, password).await
}

pub async fn from_tcp_stream(
    mut tcp_stream: BufStream<TcpStream>,
    username: &str,
    password: &str,
) -> Result<BufStream<TcpStream>> {
    let mut buf = String::new();

    // protocol header
    tcp_stream
        .read_line(&mut buf)
        .await
        .map_err(Error::TcpRead)?;
    if buf != "XtraLib.Stream.0\n" {
        return Err(Error::TcpHeaderProtocol(buf));
    }
    buf.clear();

    // version header
    tcp_stream
        .read_line(&mut buf)
        .await
        .map_err(Error::TcpRead)?;
    if buf != "Tacview.RealTimeTelemetry.0\n" {
        return Err(Error::TcpHeaderVersion(buf));
    }
    buf.clear();

    // hostname
    tcp_stream
        .read_line(&mut buf)
        .await
        .map_err(Error::TcpRead)?;
    tracing::debug!(hostname = %buf, "server hostname");

    let eoh = tcp_stream.read_u8().await.map_err(Error::TcpRead)?;
    if eoh != 0 {
        return Err(Error::TcpEndOfHeader(eoh));
    }

    tcp_stream
        .write_all(b"XtraLib.Stream.0\n")
        .await
        .map_err(Error::TcpWrite)?;
    tcp_stream
        .write_all(b"Tacview.RealTimeTelemetry.0\n")
        .await
        .map_err(Error::TcpWrite)?;
    tcp_stream
        .write_all(format!("{username}\n").as_bytes())
        .await
        .map_err(Error::TcpWrite)?;
    tcp_stream
        .write_all(format!("{}\x00", hash_password(password)).as_bytes())
        .await
        .map_err(Error::TcpWrite)?;

    tcp_stream.flush().await.map_err(Error::TcpWrite)?;

    Ok(tcp_stream)
}
