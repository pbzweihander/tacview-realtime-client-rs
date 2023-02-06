pub mod record;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

use crate::error::{Error, Result};

use self::record::Record;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub file_type: String,
    pub file_version: String,
}

#[derive(Debug)]
pub struct RealTimeReader<R> {
    pub header: Header,
    reader: R,
}

impl<R> RealTimeReader<R>
where
    R: AsyncBufRead + Unpin,
{
    pub async fn try_from_reader(mut reader: R) -> Result<Self> {
        let header = parse_header(&mut reader).await?;
        Ok(Self { header, reader })
    }

    pub async fn next(&mut self) -> Result<Record> {
        let mut line = String::new();
        loop {
            self.reader
                .read_line(&mut line)
                .await
                .map_err(Error::AcmiReaderRead)?;

            line = line.strip_suffix('\n').unwrap_or(&line).to_string();

            // comment
            if line.starts_with("//") {
                line.clear();
                continue;
            }

            // multiline
            if line.ends_with('\\') {
                line.pop();
                line.push('\n');
                continue;
            }

            break;
        }

        tracing::debug!(line, "parsing ACMI line");
        Record::from_str(&line)
    }
}

async fn parse_header<R>(reader: &mut R) -> Result<Header>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = String::new();

    // file type
    reader
        .read_line(&mut buf)
        .await
        .map_err(Error::AcmiReaderRead)?;
    if buf != "FileType=text/acmi/tacview\n" {
        return Err(Error::BadAcmiFileType(buf));
    }
    let file_type = buf
        .strip_prefix("FileType=")
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string();
    buf.clear();

    // file version
    reader
        .read_line(&mut buf)
        .await
        .map_err(Error::AcmiReaderRead)?;
    if !buf.starts_with("FileVersion=2.2") {
        return Err(Error::BadAcmiFileVersion(buf));
    }
    let file_version = buf
        .strip_prefix("FileVersion=")
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string();
    buf.clear();

    Ok(Header {
        file_type,
        file_version,
    })
}
