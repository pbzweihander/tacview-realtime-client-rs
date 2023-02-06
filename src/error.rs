use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to server with TCP: {0}")]
    TcpConnect(#[source] std::io::Error),
    #[error("failed to read from server with TCP: {0}")]
    TcpRead(#[source] std::io::Error),
    #[error("failed to write to server with TCP: {0}")]
    TcpWrite(#[source] std::io::Error),
    #[error("bad TCP header protocol, found: {0}")]
    TcpHeaderProtocol(String),
    #[error("bad TCP header version, found: {0}")]
    TcpHeaderVersion(String),
    #[error("bad TCP end-of-header")]
    TcpEndOfHeader(u8),
    #[error("failed to read from ACMI reader: {0}")]
    AcmiReaderRead(#[source] std::io::Error),
    #[error("bad ACMI file type header, found: {0}")]
    BadAcmiFileType(String),
    #[error("bad ACMI file version header, found: {0}")]
    BadAcmiFileVersion(String),
    #[error("unexpected end-of-line from ACMI reader")]
    AcmiReaderEol,
    #[error("failed to parse integer: {0}")]
    ParseInt(#[source] ParseIntError),
    #[error("failed to parse datetime: {0}")]
    ParseDateTime(#[source] time::error::Parse),
    #[error("failed to parse float: {0}")]
    ParseFloat(#[source] ParseFloatError),
    #[error("malformed event, found: {0}")]
    MalformedEvent(String),
    #[error("malformed global property, found: {0}")]
    MalformedGlobalProperty(String),
    #[error("malformed object property, found: {0}")]
    MalformedObjectProperty(String),
    #[error("malformed coordinates, found: {0}")]
    MalformedCoords(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
