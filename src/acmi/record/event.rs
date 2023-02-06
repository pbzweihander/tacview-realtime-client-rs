use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

use super::parse_object_id;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Event {
    /// Generic event.  
    /// `Event=Message|705|Maverick has violated ATC directives`
    Message(u64, String),
    /// Bookmarks are highlighted in the time line and in the event log. They
    /// are easy to spot and handy to highlight parts of the flight, like a
    /// bombing run, or when the trainee was in her final approach for
    /// landing.  
    /// `Event=Bookmark|Starting precautionary landing practice`
    Bookmark(String),
    /// Debug events are highlighted and easy to spot in the timeline and event
    /// log. Because they must be used for development purposes, they are
    /// displayed only when launching Tacview with the command line argument
    /// `/Debug:on`  
    /// `Event=Debug|327 active planes`
    Debug(String),
    /// This event is useful to specify when an aircraft (or any object) is
    /// cleanly removed from the battlefield (not destroyed). This prevents
    /// Tacview from generating a `Destroyed` event by error.  
    /// `Event=LeftArea|507|`
    LeftArea(u64),
    /// When an object has been officially destroyed.  
    /// `Event=Destroyed|6A56|`
    Destroyed(u64),
    /// Because Tacview may not always properly auto-detect take-off events, it
    /// can be useful to manually inject this event in the flight recording.  
    /// `Event=TakenOff|2723|Col. Sinclair has taken off from Camarillo Airport`
    TakenOff(u64, String),
    /// Because Tacview may not always properly auto-detect landing events, it
    /// can be useful to manually inject this event in the flight recording.
    /// `Event=Landed|705|Maverick has landed on the USS Ranger`
    Landed(u64, String),
    /// Mainly used for real-life training debriefing to specify when a weapon
    /// (typically a missile) reaches or misses its target. Tacview will report
    /// in the shot log as well as in the 3D view the result of the shot. Most
    /// parameters are optional. `SourceId` designates the object which has
    /// fired the weapon, while `TargetId` designates the target. Even if the
    /// displayed result may be in nautical miles, bullseye coordinates must be
    /// specified in meters. The target must be explicitly (manually) destroyed
    /// or disabled using the appropriate properties independently from this
    /// event.  
    /// `Event=Timeout|SourceId:507|AmmoType:FOX2|AmmoCount:1|Bullseye:50/15000/2500|TargetId:201|IntendedTarget:Leader|Outcome:Kill`
    Timeout(TimeoutEvent),

    /// Unknown event. `(type, message)`
    Unknown(String, String),
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('|');
        let event_type = tokens
            .next()
            .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
        match event_type {
            "Event=Message" => {
                let object_id = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                let object_id = parse_object_id(object_id)?;
                let message = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?
                    .to_string();
                Ok(Self::Message(object_id, message))
            }
            "Event=Bookmark" => {
                let message = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?
                    .to_string();
                Ok(Self::Bookmark(message))
            }
            "Event=Debug" => {
                let message = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?
                    .to_string();
                Ok(Self::Debug(message))
            }
            "Event=LeftArea" => {
                let object_id = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                let object_id = parse_object_id(object_id)?;
                Ok(Self::LeftArea(object_id))
            }
            "Event=Destroyed" => {
                let object_id = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                let object_id = parse_object_id(object_id)?;
                Ok(Self::Destroyed(object_id))
            }
            "Event=TakenOff" => {
                let object_id = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                let object_id = parse_object_id(object_id)?;
                let message = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?
                    .to_string();
                Ok(Self::TakenOff(object_id, message))
            }
            "Event=Landed" => {
                let object_id = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                let object_id = parse_object_id(object_id)?;
                let message = tokens
                    .next()
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?
                    .to_string();
                Ok(Self::Landed(object_id, message))
            }
            "Event=Timeout" => {
                let timeout = TimeoutEvent::from_tokens_iter(tokens)?;
                Ok(Self::Timeout(timeout))
            }
            _ => {
                let (ty, message) = s.split_once('|').unwrap_or((s, ""));
                let (_, ty) = ty
                    .split_once('=')
                    .ok_or_else(|| Error::MalformedEvent(s.to_string()))?;
                Ok(Self::Unknown(ty.to_string(), message.to_string()))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimeoutEvent {
    pub source_id: Option<String>,
    pub ammo_type: Option<String>,
    pub ammo_count: Option<String>,
    pub bullseye: Option<String>,
    pub target_id: Option<String>,
    pub intended_target: Option<String>,
    pub outcome: Option<String>,
}

impl TimeoutEvent {
    fn from_tokens_iter<'a, I>(iter: I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut source_id = None;
        let mut ammo_type = None;
        let mut ammo_count = None;
        let mut bullseye = None;
        let mut target_id = None;
        let mut intended_target = None;
        let mut outcome = None;
        for token in iter {
            if let Some(token) = token.strip_prefix("SourceId:") {
                source_id = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("AmmoType:") {
                ammo_type = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("AmmoCount:") {
                ammo_count = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("Bullseye:") {
                bullseye = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("TargetId:") {
                target_id = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("IntendedTarget:") {
                intended_target = Some(token.to_string());
            } else if let Some(token) = token.strip_prefix("Outcome:") {
                outcome = Some(token.to_string());
            }
        }
        Ok(Self {
            source_id,
            ammo_type,
            ammo_count,
            bullseye,
            target_id,
            intended_target,
            outcome,
        })
    }
}
