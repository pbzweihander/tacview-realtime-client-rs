pub mod acmi;
pub mod error;
pub mod tcp;

use tokio::{
    io::BufStream,
    net::{TcpStream, ToSocketAddrs},
};

use crate::{acmi::RealTimeReader, error::Result};

pub type TcpRealTimeReader = RealTimeReader<BufStream<TcpStream>>;

pub async fn connect<A>(
    addr: A,
    username: &str,
    password: &str,
) -> Result<RealTimeReader<BufStream<TcpStream>>>
where
    A: ToSocketAddrs,
{
    let tcp_stream = crate::tcp::connect(addr, username, password).await?;
    RealTimeReader::try_from_reader(tcp_stream).await
}
