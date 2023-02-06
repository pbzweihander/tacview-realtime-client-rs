use std::str::FromStr;

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum GlobalProperty {
    // Text Properties
    /// Source simulator, control station or file format.  
    /// `DataSource=DCS 2.0.0.48763`  
    /// `DataSource=GPX File`
    DataSource(String),
    /// Software or hardware used to record the data.  
    /// `DataRecorder=Tacview 1.5`  
    /// `DataRecorder=Falcon 4.0`
    DataRecorder(String),
    /// Base time (UTC) for the current mission. This time is combined with each
    /// frame offset (in seconds) to get the final absolute UTC time for each
    /// data sample.  
    /// `ReferenceTime=2011-06-02T05:00:00Z`
    #[serde(with = "time::serde::rfc3339")]
    ReferenceTime(OffsetDateTime),
    /// Recording (file) creation (UTC) time.  
    /// `RecordingTime=2016-02-18T16:44:12Z`
    #[serde(with = "time::serde::rfc3339")]
    RecordingTime(OffsetDateTime),
    /// Author or operator who has created this recording.  
    /// `Author=Lt. Cmdr. Rick 'Jester' Heatherly`
    Author(String),
    /// Mission/flight title or designation.  
    /// `Title=Counter Attack`
    Title(String),
    /// Category of the flight/mission.  
    /// `Category=Close air support`
    Category(String),
    /// Free text containing the briefing of the flight/mission.  
    /// `Briefing=Destroy all SCUD launchers`
    Briefing(String),
    /// Free text containing the debriefing.  
    /// `Debriefing=Managed to stay ahead of the airplane.`
    Debriefing(String),
    /// Free comments about the flight. Do not forget to escape any end-of-line
    /// character you want to inject into the comments.  
    /// `Comments=Part of the recording is missing because of technical difficulties.`
    Comments(String),

    // Numeric Properties
    /// These properties are used to reduce the file size by centering
    /// coordinates around a median point. They will be added to each object
    /// Longitude and Latitude to get the final coordinates.  
    /// `ReferenceLongitude=-129`
    /// `ReferenceLatitude=43`
    ReferenceLongitude(f64),
    ReferenceLatitude(f64),

    /// Unknown global property. `(name, value)`
    Unknown(String, String),
}

impl FromStr for GlobalProperty {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(value) = s.strip_prefix("DataSource=") {
            Ok(Self::DataSource(value.to_string()))
        } else if let Some(value) = s.strip_prefix("DataRecorder=") {
            Ok(Self::DataRecorder(value.to_string()))
        } else if let Some(value) = s.strip_prefix("ReferenceTime=") {
            Ok(Self::ReferenceTime(
                OffsetDateTime::parse(value, &Rfc3339).map_err(Error::ParseDateTime)?,
            ))
        } else if let Some(value) = s.strip_prefix("RecordingTime=") {
            Ok(Self::RecordingTime(
                OffsetDateTime::parse(value, &Rfc3339).map_err(Error::ParseDateTime)?,
            ))
        } else if let Some(value) = s.strip_prefix("Author=") {
            Ok(Self::Author(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Title=") {
            Ok(Self::Title(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Category=") {
            Ok(Self::Category(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Briefing=") {
            Ok(Self::Briefing(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Debriefing=") {
            Ok(Self::Debriefing(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Comments=") {
            Ok(Self::Comments(value.to_string()))
        } else if let Some(value) = s.strip_prefix("ReferenceLongitude=") {
            Ok(Self::ReferenceLongitude(
                f64::from_str(value).map_err(Error::ParseFloat)?,
            ))
        } else if let Some(value) = s.strip_prefix("ReferenceLatitude=") {
            Ok(Self::ReferenceLatitude(
                f64::from_str(value).map_err(Error::ParseFloat)?,
            ))
        } else {
            let (name, value) = s
                .split_once('=')
                .ok_or_else(|| Error::MalformedGlobalProperty(s.to_string()))?;
            Ok(Self::Unknown(name.to_string(), value.to_string()))
        }
    }
}
