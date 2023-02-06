pub mod event;
pub mod global_property;
pub mod object_property;

use std::str::FromStr;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

use self::{event::Event, global_property::GlobalProperty, object_property::ObjectProperty};

fn parse_object_id(id: &str) -> Result<u64> {
    u64::from_str_radix(id, 16).map_err(Error::ParseInt)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Record {
    Remove(u64),
    Frame(f64),
    Event(Event),
    GlobalProperties(Vec<GlobalProperty>),
    Update(u64, Vec<ObjectProperty>),
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // remove
        if let Some(line) = s.strip_prefix('-') {
            let id = parse_object_id(line)?;
            return Ok(Record::Remove(id));
        }

        // time frame
        if let Some(line) = s.strip_prefix('#') {
            let timeframe = f64::from_str(line).map_err(Error::ParseFloat)?;
            return Ok(Record::Frame(timeframe));
        }

        let (id, rest) = s.split_once(',').ok_or(Error::AcmiReaderEol)?;

        if id == "0" {
            if rest.starts_with("Event=") {
                let event = Event::from_str(rest)?;
                Ok(Self::Event(event))
            } else {
                let global_properties = parse_comma(rest)
                    .into_iter()
                    .map(|token| GlobalProperty::from_str(&token))
                    .try_collect()?;
                Ok(Self::GlobalProperties(global_properties))
            }
        } else {
            let id = parse_object_id(id)?;
            let object_properties = parse_comma(rest)
                .into_iter()
                .map(|token| ObjectProperty::from_str(&token))
                .try_collect()?;
            Ok(Self::Update(id, object_properties))
        }
    }
}

fn parse_comma(line: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut buf = String::new();
    let tokens = line.split(',');
    for token in tokens {
        buf.push_str(token);

        if buf.ends_with('\\') {
            buf.pop();
            buf.push(',');
        } else {
            output.push(buf.clone());
            buf.clear();
        }
    }
    if !buf.is_empty() {
        output.push(buf);
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_comman() {
        let line = "a=1,b=2,c=3,d=4";
        let expected = vec![
            "a=1".to_string(),
            "b=2".to_string(),
            "c=3".to_string(),
            "d=4".to_string(),
        ];
        assert_eq!(parse_comma(line), expected);

        let line = "a=1,b=2\\,c=3,d=4";
        let expected = vec!["a=1".to_string(), "b=2,c=3".to_string(), "d=4".to_string()];
        assert_eq!(parse_comma(line), expected);

        let line = "a=1,b=2,\\,\\c=3,d=4";
        let expected = vec![
            "a=1".to_string(),
            "b=2".to_string(),
            ",\\c=3".to_string(),
            "d=4".to_string(),
        ];
        assert_eq!(parse_comma(line), expected);
    }
}
