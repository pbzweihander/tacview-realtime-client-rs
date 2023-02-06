use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::parse_object_id;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum ObjectProperty {
    /// Object coordinates
    T(Coords),

    // Text Properties
    /// The object name should use the most common notation for each object. It
    /// is strongly recommended to use
    /// [ICAO](https://www.icao.int/publications/DOC8643/Pages/Search.aspx) or
    /// [NATO](https://en.wikipedia.org/wiki/NATO_reporting_name) names like:
    /// `C172` or `F/A-18C`. This will help Tacview to associate each object
    /// with the corresponding entry in its database. `Type` and `Name` are the
    /// only properties which *CANNOT* be predefined in Tacview
    /// [database](https://www.tacview.net/documentation/database/en/).  
    /// `Name=F-16C-52`
    Name(String),
    /// Object types are built using tags. This makes object management much
    /// more powerful and transparent than with the previous exclusive types.
    /// (see below for the list of supported types). `Type` and `Name` are the
    /// only properties which *CANNOT* be predefined in Tacview
    /// [database](https://www.tacview.net/documentation/database/en/).  
    /// `Type=Air+FixedWing`
    Type(HashSet<Tag>),
    /// Parent hexadecimal object id. Useful to associate for example a missile
    /// (child object) and its launcher aircraft (parent object).  
    /// `Parent=2D50A7`
    Parent(u64),
    /// Hexadecimal id of the following object. Typically used to link waypoints
    /// together.  
    /// `Next=40F1`
    Next(u64),
    /// The call sign will be displayed in priority over the object name and
    /// sometimes pilot name, especially in the 3D view and selection boxes.
    /// This is handy for mission debriefings where call signs are more
    /// informative than aircraft names.  
    /// `CallSign=Jester`
    Callsign(String),
    /// Aircraft registration (aka tail number)  
    /// `Registration=N594EX`
    Registration(String),
    /// Current transponder code. Any code is possible, there is no limitation
    /// like with the old 4 digit transponders.  
    /// `Squawk=1200`
    Squawk(String),
    /// Mode S equipped aircraft uniquely assigned ICAO 24-bit address.  
    /// `ICAO24=A72EC8`
    Icao24(String),
    /// Aircraft pilot in command name.  
    /// `Pilot=Iceman`
    Pilot(String),
    /// Group the object belongs to. Used to group objects together. For
    /// example, a formation of F-16 flying a CAP together.  
    /// `Group=Springfield`
    Group(String),
    /// ISO 3166-1 alpha-2 country code.  
    /// `Country=us`
    Country(String),
    /// Coalition  
    /// `Coalition=Allies`
    Coalition(String),
    /// Can be one of the following: `Red`, `Orange`, `Yellow` (Tacview 1.8.8),
    /// `Green`, `Cyan` (Tacview 1.8.8), `Blue`, `Violet`. Colors are predefined
    /// to ensure a clear display of the whole battlefield in all conditions.  
    /// `Color=Blue`
    Color(Color),
    /// Filename of the 3D model which will be used to represent the object in
    /// the 3D view. 3D models must be in
    /// [Wavefront .obj file format](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
    /// and stored in either `%ProgramData%\Tacview\Data\Meshes\` or
    /// `%APPDATA%\Tacview\Data\Meshes\`.  
    /// Learn more about 3D models by reading the
    /// [dedicated documentation](https://www.tacview.net/documentation/3dobjects/en/)  
    /// `Shape=Rotorcraft.Bell 206.obj`
    Shape(String),
    /// Debug text visible in the 3D view when Tacview is launched with the /Debug:on command line argument.
    /// `Debug=ObjectHandle:0x237CB9`
    Debug(String),
    /// Free real-time text displayable in the 3D view and telemetry windows (to
    /// provide miscellaneous info to the end-user)  
    /// `Label=Lead aircraft`
    Label(String),
    /// Target currently focused by the object (typically used to designate
    /// laser beam target object, can also be used to show what the pilot is
    /// currently focused on)  
    /// `FocusedTarget=3001200`
    FocusedTarget(u64),
    /// Primary target hexadecimal id (could be locked using any device, like
    /// radar, IR, NVG, ...)  
    /// `LockedTarget2=3001200`
    LockedTarget(u64),
    LockedTarget2(u64),
    LockedTarget3(u64),
    LockedTarget4(u64),
    LockedTarget5(u64),
    LockedTarget6(u64),
    LockedTarget7(u64),
    LockedTarget8(u64),
    LockedTarget9(u64),

    // Numeric Properties
    /// The higher the ratio, the more important is the object is (e.g. locally
    /// simulated aircraft could be 1.0 importance factor)  
    /// Unit: ratio  
    /// `Importance=1`
    Importance(u64),
    /// Plane position in its Group (the lowest is the leader)  
    /// Unit: index  
    /// `Slot=0`
    Slot(u64),
    /// Specifies that an object is disabled (typically out-of-combat) without
    /// being destroyed yet. This is particularly useful for combat training and
    /// shotlogs.
    /// Unit: boolean
    /// `Disabled=1`
    Disabled(bool),
    /// This property is useful to hide specific objects from the 3D view. Can
    /// be used for a fog-of-war effect, or to prevent virtual objects from
    /// being displayed. When set to 1, the object is fully visible. When set to
    /// 0, the object is invisible and may be omitted from objects lists.  
    /// Unit: ratio
    /// `Visible=0.333`
    Visible(f64),
    /// Use this attribute to record the current health status of an object. The
    /// ratio is equal to 1.0 when the object is brand new, and 0.0 whenever the
    /// object is out of combat/dead/destroyed. This attribute as currently no
    /// effect on the events, you still need to remove the object manually
    /// whenever it is destroyed.  
    /// Unit: ratio  
    /// Health=0.84
    Health(f64),
    /// Object length. Especially useful when displaying buildings.  
    /// Unit: m  
    /// `Length=20.5`
    Length(f64),
    /// Object width. Especially useful when displaying buildings.  
    /// Unit: m  
    /// `Width=10.27`
    Width(f64),
    /// Object bounding sphere radius. Object bounding sphere radius. Can be
    /// used to define custom explosion, smoke/grenade radius. Can be
    /// animated.  
    /// Unit: m  
    /// `Radius=82`
    Radius(f64),
    /// Indicated airspeed  
    /// Unit: m/s  
    /// `IAS=69.4444`
    Ias(f64),
    /// Calibrated airspeed  
    /// Unit: m/s
    /// `CAS=250`
    Cas(f64),
    /// True airspeed  
    /// Unit: m/s  
    /// `TAS=75`
    Tas(f64),
    /// Mach number  
    /// Unit: ratio  
    /// `Mach=0.75`
    Mach(f64),
    /// Angle of attack  
    /// Unit: deg  
    /// `AOA=15.7`
    Aoa(f64),
    /// Sideslip angle, also called angle of sideslip  
    /// Unit: deg  
    /// `AOS=5.2`
    Aos(f64),
    /// Object altitude above ground level  
    /// Unit: m  
    /// `AGL=1501.2`
    Agl(f64),
    /// Aircraft heading. When there is no roll and pitch data available, this
    /// property can be used to specify the yaw while keeping full rotation
    /// emulation in the 3D view.  
    /// Unit: deg  
    /// `HDG=185.3`
    Hdg(f64),
    /// Aircraft magnetic heading. Heading relative to local magnetic north.  
    /// Unit: deg  
    /// HDM=187.3
    Hdm(f64),
    /// Main/engine #1 throttle handle position (could be >1 for Afterburner and
    /// <0 for reverse)  
    /// Unit: ratio  
    /// `Throttle=0.75`
    Throttle(f64),
    /// Main/engine #1 afterburner status  
    /// Unit: ratio  
    /// `Afterburner=1`
    Afterburner(f64),
    /// Air brakes status  
    /// Unit: ratio  
    /// `AirBrakes=0`
    AirBrakes(f64),
    /// Flaps position  
    /// Unit: ratio  
    /// `Flaps=0.4`
    Flaps(f64),
    /// Landing gear status  
    /// Unit: ratio  
    /// `LandingGear=1`
    LandingGear(f64),
    /// Landing gear handle position  
    /// Unit: ratio  
    /// `LandingGearHandle=0`
    LandingGearHandle(f64),
    /// Arresting hook status  
    /// Unit: ratio  
    /// `Tailhook=1`
    Tailhook(f64),
    /// Parachute status (not to be mistaken for DragChute)  
    /// Unit: ratio  
    /// `Parachute=0`
    Parachute(f64),
    /// Drogue/Drag Parachute status  
    /// Unit: ratio  
    /// `DragChute=1`
    DragChute(f64),
    /// Fuel quantity currently available in each tanks (up to 10 tanks
    /// supported).  
    /// Unit: kg  
    /// `FuelWeight4=8750`
    FuelWeight(f64),
    FuelWeight2(f64),
    FuelWeight3(f64),
    FuelWeight4(f64),
    FuelWeight5(f64),
    FuelWeight6(f64),
    FuelWeight7(f64),
    FuelWeight8(f64),
    FuelWeight9(f64),
    /// Fuel quantity currently available in each tanks (up to 10 tanks
    /// supported).  
    /// Unit: l  
    /// `FuelVolume=75`
    FuelVolume(f64),
    FuelVolume2(f64),
    FuelVolume3(f64),
    FuelVolume4(f64),
    FuelVolume5(f64),
    FuelVolume6(f64),
    FuelVolume7(f64),
    FuelVolume8(f64),
    FuelVolume9(f64),
    /// Fuel flow for each engine (up to 8 engines supported).  
    /// Unit: kg/hour  
    /// `FuelFlowWeight2=38.08`
    FuelFlowWeight(f64),
    FuelFlowWeight2(f64),
    FuelFlowWeight3(f64),
    FuelFlowWeight4(f64),
    FuelFlowWeight5(f64),
    FuelFlowWeight6(f64),
    FuelFlowWeight7(f64),
    /// Fuel flow for each engine (up to 8 engines supported).  
    /// Unit: l/hour  
    /// `FuelFlowVolume2=53.2`
    FuelFlowVolume(f64),
    FuelFlowVolume2(f64),
    FuelFlowVolume3(f64),
    FuelFlowVolume4(f64),
    FuelFlowVolume5(f64),
    FuelFlowVolume6(f64),
    FuelFlowVolume7(f64),
    /// Radar mode (0 = off)  
    /// Unit: number  
    /// `RadarMode=1`
    RadarMode(u64),
    /// Radar azimuth (heading) relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarAzimuth=-20`
    RadarAzimuth(f64),
    /// Radar elevation relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarElevation=15`
    RadarElevation(f64),
    /// Radar roll angle relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarRoll=-45`
    RadarRoll(f64),
    /// Radar scan range  
    /// Unit: m
    /// `RadarRange=296320`
    RadarRange(f64),
    /// Radar beamwidth in azimuth  
    /// Unit: deg  
    /// `RadarHorizontalBeamwidth=40`
    RadarHorizontalBeamwidth(f64),
    /// Radar beamwidth in elevation  
    /// Unit: deg  
    /// `RadarVerticalBeamwidth=12`
    RadarVerticalBeamwidth(f64),
    /// Radar Range Gate azimuth (heading) relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarRangeGateAzimuth=-20`
    RadarRangeGateAzimuth(f64),
    /// Radar Range Gate elevation relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarRangeGateElevation=15`
    RadarRangeGateElevation(f64),
    /// Radar Range Gate roll angle relative to aircraft orientation  
    /// Unit: deg  
    /// `RadarRangeGateRoll=-45`
    RadarRangeGateRoll(f64),
    /// Defines the beginning of the range currently focused on by the radar
    /// (not to be confused with RadarRange).  
    /// Unit: m  
    /// `RadarRangeGateMin=37040`
    RadarRangeGateMin(f64),
    /// Defines the end of the range currently focused on by the radar (not to
    /// be confused with RadarRange).  
    /// Unit: m  
    /// `RadarRangeGateMax=74080`
    RadarRangeGateMax(f64),
    /// Radar Range Gate beamwidth in azimuth  
    /// Unit: deg  
    /// `RadarRangeGateHorizontalBeamwidth=40`
    RadarRangeGateHorizontalBeamwidth(f64),
    /// Radar Range Gate beamwidth in elevation  
    /// Unit: deg  
    /// `RadarRangeGateVerticalBeamwidth=12`
    RadarRangeGateVerticalBeamwidth(f64),
    /// Primary target lock mode (0 = no lock/no target)  
    /// Unit: number  
    /// `LockedTargetMode=1`
    LockedTargetMode(u64),
    /// Primary target azimuth (heading) relative to aircraft orientation  
    /// Unit: deg  
    /// `LockedTargetAzimuth=14.5`
    LockedTargetAzimuth(f64),
    /// Primary target elevation relative to aircraft orientation  
    /// Unit: deg  
    /// `LockedTargetElevation=0.9`
    LockedTargetElevation(f64),
    /// Primary target distance to aircraft  
    /// Unit: m  
    /// `LockedTargetRange=17303`
    LockedTargetRange(f64),
    /// Enable/disable engagement range (such as when a SAM site turns off its
    /// radar) (0 = off)  
    /// Unit: number  
    /// `EngagementMode=1`
    EngagementMode(u64),
    EngagementMode2(u64),
    /// Engagement range for anti-aircraft units. This is the radius of the
    /// sphere which will be displayed in the 3D view. Typically used for SAM
    /// and AAA units, but this can be also relevant to warships.  
    /// Unit: m  
    /// `EngagementRange=2500`
    ///
    /// You can optionally specify the vertical engagement range to draw an
    /// ovoid engagement bubble.  
    /// `VerticalEngagementRange=1800`
    EngagementRange(f64),
    EngagementRange2(f64),
    VerticalEngagementRange(f64),
    VerticalEngagementRange2(f64),
    /// Raw player HOTAS/Yoke position in real-life (flight sim input device)  
    /// Unit: ratio  
    /// `PitchControlInput=0.41`
    RollControlInput(f64),
    PitchControlInput(f64),
    YawControlInput(f64),
    /// HOTAS/Yoke position in simulated (with response curves) or real-life
    /// cockpit  
    /// Unit: ratio  
    /// `PitchControlPosition=0.3`
    RollControlPosition(f64),
    PitchControlPosition(f64),
    YawControlPosition(f64),
    /// Trim position for each axis  
    /// Unit: ratio  
    /// `PitchTrimTab=-0.15`
    RollTrimTab(f64),
    PitchTrimTab(f64),
    YawTrimTab(f64),
    /// Control surfaces position on the aircraft  
    /// Unit: ratio  
    /// `Elevator=0.15`
    AileronLeft(f64),
    AileronRight(f64),
    Elevator(f64),
    Rudder(f64),
    /// Pilot head orientation in the cockpit relative to the aircraft
    /// orientation  
    /// Unit: deg  
    /// `PilotHeadPitch=12`
    PilotHeadRoll(f64),
    PilotHeadPitch(f64),
    PilotHeadYaw(f64),
    /// Gravitational force equivalent of the acceleration in each axis relative
    /// to the aircraft orientation  
    /// Unit: g  
    /// `VerticalGForce=3.4`
    VerticalGForce(f64),
    LongitudinalGForce(f64),
    LateralGForce(f64),
    /// Position of the main weapon trigger position. Set to 1.0 when the
    /// trigger is being fully pressed. All other values (such as 0.0) are
    /// considered as released. You could use continuous values from 0.0 to 1.0
    /// to display the course of the trigger during time.  
    /// Unit: boolean  
    /// `TriggerPressed=1`
    TriggerPressed(bool),
    /// Ratio between 0 and 1 describing the current Environmental Noise Level
    /// measured by the flight recorder. Typically used by gliders to detect
    /// engine use. This is the equivalent of the ENL field which can be found
    /// in IGC files.  
    /// Unit: ratio  
    /// `ENL=0.02`
    Enl(f64),
    /// Heart rate in beats per minute.  
    /// Unit: number  
    /// `HeartRate=72`
    HeartRate(u64),
    /// Blood oxygen saturation (SpO2) is the percentage of blood that is
    /// saturated with oxygen.  
    /// Unit: ratio  
    /// `SpO2=0.95`
    SpO2(f64),

    /// Unknown property. `(name, value)`
    Unknown(String, String),
}

impl FromStr for ObjectProperty {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(value) = s.strip_prefix("T=") {
            let coords = Coords::from_str(value)?;
            Ok(Self::T(coords))
        } else if let Some(value) = s.strip_prefix("Name=") {
            Ok(Self::Name(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Type=") {
            let tags = value.split('+').map(Tag::from_str).try_collect()?;
            Ok(Self::Type(tags))
        } else if let Some(value) = s.strip_prefix("Parent=") {
            let id = parse_object_id(value)?;
            Ok(Self::Parent(id))
        } else if let Some(value) = s.strip_prefix("Next=") {
            let id = parse_object_id(value)?;
            Ok(Self::Next(id))
        } else if let Some(value) = s.strip_prefix("Callsign=") {
            Ok(Self::Callsign(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Registration=") {
            Ok(Self::Registration(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Squawk=") {
            Ok(Self::Squawk(value.to_string()))
        } else if let Some(value) = s.strip_prefix("ICAO24=") {
            Ok(Self::Icao24(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Pilot=") {
            Ok(Self::Pilot(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Group=") {
            Ok(Self::Group(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Country=") {
            Ok(Self::Country(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Coalition=") {
            Ok(Self::Coalition(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Color=") {
            let color = Color::from_str(value)?;
            Ok(Self::Color(color))
        } else if let Some(value) = s.strip_prefix("Shape=") {
            Ok(Self::Shape(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Debug=") {
            Ok(Self::Debug(value.to_string()))
        } else if let Some(value) = s.strip_prefix("Label=") {
            Ok(Self::Label(value.to_string()))
        } else if let Some(value) = s.strip_prefix("FocusedTarget=") {
            let id = parse_object_id(value)?;
            Ok(Self::FocusedTarget(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget2=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget2(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget3=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget3(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget4=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget4(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget5=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget5(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget6=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget6(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget7=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget7(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget8=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget8(id))
        } else if let Some(value) = s.strip_prefix("LockedTarget9=") {
            let id = parse_object_id(value)?;
            Ok(Self::LockedTarget9(id))
        } else if let Some(value) = s.strip_prefix("Importance=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::Importance(value))
        } else if let Some(value) = s.strip_prefix("Slot=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::Slot(value))
        } else if let Some(value) = s.strip_prefix("Disabled=") {
            let value = value == "1";
            Ok(Self::Disabled(value))
        } else if let Some(value) = s.strip_prefix("Visible=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Visible(value))
        } else if let Some(value) = s.strip_prefix("Health=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Health(value))
        } else if let Some(value) = s.strip_prefix("Length=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Length(value))
        } else if let Some(value) = s.strip_prefix("Width=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Width(value))
        } else if let Some(value) = s.strip_prefix("Radius=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Radius(value))
        } else if let Some(value) = s.strip_prefix("IAS=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Ias(value))
        } else if let Some(value) = s.strip_prefix("CAS=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Cas(value))
        } else if let Some(value) = s.strip_prefix("TAS=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Tas(value))
        } else if let Some(value) = s.strip_prefix("Mach=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Mach(value))
        } else if let Some(value) = s.strip_prefix("AOA=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Aoa(value))
        } else if let Some(value) = s.strip_prefix("AOS=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Aos(value))
        } else if let Some(value) = s.strip_prefix("AGL=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Agl(value))
        } else if let Some(value) = s.strip_prefix("HDG=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Hdg(value))
        } else if let Some(value) = s.strip_prefix("HDM=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Hdm(value))
        } else if let Some(value) = s.strip_prefix("Throttle=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Throttle(value))
        } else if let Some(value) = s.strip_prefix("Afterburner=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Afterburner(value))
        } else if let Some(value) = s.strip_prefix("AirBrakes=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::AirBrakes(value))
        } else if let Some(value) = s.strip_prefix("Flaps=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Flaps(value))
        } else if let Some(value) = s.strip_prefix("LandingGear=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LandingGear(value))
        } else if let Some(value) = s.strip_prefix("LandingGearHandle=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LandingGearHandle(value))
        } else if let Some(value) = s.strip_prefix("Tailhook=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Tailhook(value))
        } else if let Some(value) = s.strip_prefix("Parachute=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Parachute(value))
        } else if let Some(value) = s.strip_prefix("DragChute=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::DragChute(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight2(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight3=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight3(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight4=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight4(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight5=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight5(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight6=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight6(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight7=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight7(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight8=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight8(value))
        } else if let Some(value) = s.strip_prefix("FuelWeight9=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelWeight9(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume2(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume3=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume3(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume4=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume4(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume5=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume5(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume6=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume6(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume7=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume7(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume8=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume8(value))
        } else if let Some(value) = s.strip_prefix("FuelVolume9=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelVolume9(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight2(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight3=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight3(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight4=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight4(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight5=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight5(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight6=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight6(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowWeight7=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowWeight7(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume2(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume3=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume3(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume4=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume4(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume5=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume5(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume6=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume6(value))
        } else if let Some(value) = s.strip_prefix("FuelFlowVolume7=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::FuelFlowVolume7(value))
        } else if let Some(value) = s.strip_prefix("RadarMode=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::RadarMode(value))
        } else if let Some(value) = s.strip_prefix("RadarAzimuth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarAzimuth(value))
        } else if let Some(value) = s.strip_prefix("RadarElevation=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarElevation(value))
        } else if let Some(value) = s.strip_prefix("RadarRoll=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRoll(value))
        } else if let Some(value) = s.strip_prefix("RadarRange=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRange(value))
        } else if let Some(value) = s.strip_prefix("RadarHorizontalBeamwidth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarHorizontalBeamwidth(value))
        } else if let Some(value) = s.strip_prefix("RadarVerticalBeamwidth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarVerticalBeamwidth(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateAzimuth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateAzimuth(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateElevation=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateElevation(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateRoll=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateRoll(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateMin=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateMin(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateMax=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateMax(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateHorizontalBeamwidth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateHorizontalBeamwidth(value))
        } else if let Some(value) = s.strip_prefix("RadarRangeGateVerticalBeamwidth=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RadarRangeGateVerticalBeamwidth(value))
        } else if let Some(value) = s.strip_prefix("LockedTargetMode=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::LockedTargetMode(value))
        } else if let Some(value) = s.strip_prefix("LockedTargetElevation=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LockedTargetElevation(value))
        } else if let Some(value) = s.strip_prefix("LockedTargetRange=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LockedTargetRange(value))
        } else if let Some(value) = s.strip_prefix("EngagementMode=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::EngagementMode(value))
        } else if let Some(value) = s.strip_prefix("EngagementMode2=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::EngagementMode2(value))
        } else if let Some(value) = s.strip_prefix("EngagementRange=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::EngagementRange(value))
        } else if let Some(value) = s.strip_prefix("EngagementRange2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::EngagementRange2(value))
        } else if let Some(value) = s.strip_prefix("VerticalEngagementRange=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::VerticalEngagementRange(value))
        } else if let Some(value) = s.strip_prefix("VerticalEngagementRange2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::VerticalEngagementRange2(value))
        } else if let Some(value) = s.strip_prefix("RollControlInput=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RollControlInput(value))
        } else if let Some(value) = s.strip_prefix("PitchControlInput=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PitchControlInput(value))
        } else if let Some(value) = s.strip_prefix("YawControlInput=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::YawControlInput(value))
        } else if let Some(value) = s.strip_prefix("RollControlPosition=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RollControlPosition(value))
        } else if let Some(value) = s.strip_prefix("PitchControlPosition=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PitchControlPosition(value))
        } else if let Some(value) = s.strip_prefix("YawControlPosition=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::YawControlPosition(value))
        } else if let Some(value) = s.strip_prefix("RollTrimTab=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::RollTrimTab(value))
        } else if let Some(value) = s.strip_prefix("PitchTrimTab=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PitchTrimTab(value))
        } else if let Some(value) = s.strip_prefix("YawTrimTab=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::YawTrimTab(value))
        } else if let Some(value) = s.strip_prefix("AileronLeft=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::AileronLeft(value))
        } else if let Some(value) = s.strip_prefix("AileronRight=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::AileronRight(value))
        } else if let Some(value) = s.strip_prefix("Elevator=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Elevator(value))
        } else if let Some(value) = s.strip_prefix("Rudder=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Rudder(value))
        } else if let Some(value) = s.strip_prefix("PilotHeadRoll=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PilotHeadRoll(value))
        } else if let Some(value) = s.strip_prefix("PilotHeadPitch=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PilotHeadPitch(value))
        } else if let Some(value) = s.strip_prefix("PilotHeadYaw=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::PilotHeadYaw(value))
        } else if let Some(value) = s.strip_prefix("VerticalGForce=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::VerticalGForce(value))
        } else if let Some(value) = s.strip_prefix("LongitudinalGForce=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LongitudinalGForce(value))
        } else if let Some(value) = s.strip_prefix("LateralGForce=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::LateralGForce(value))
        } else if let Some(value) = s.strip_prefix("TriggerPressed=") {
            let value = value == "1" || value == "1.0";
            Ok(Self::TriggerPressed(value))
        } else if let Some(value) = s.strip_prefix("ENL=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::Enl(value))
        } else if let Some(value) = s.strip_prefix("HeartRate=") {
            let value = u64::from_str(value).map_err(Error::ParseInt)?;
            Ok(Self::HeartRate(value))
        } else if let Some(value) = s.strip_prefix("SpO2=") {
            let value = f64::from_str(value).map_err(Error::ParseFloat)?;
            Ok(Self::SpO2(value))
        } else {
            let (name, value) = s
                .split_once('=')
                .ok_or_else(|| Error::MalformedObjectProperty(s.to_string()))?;
            Ok(Self::Unknown(name.to_string(), value.to_string()))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Coords {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub altitude: Option<f64>,
    pub roll: Option<f64>,
    pub pitch: Option<f64>,
    pub yaw: Option<f64>,
    pub u: Option<f64>,
    pub v: Option<f64>,
    pub heading: Option<f64>,
}

impl Coords {
    pub fn update(&mut self, other: &Self) {
        if let Some(longitude) = other.longitude {
            self.longitude = Some(longitude);
        }
        if let Some(latitude) = other.latitude {
            self.latitude = Some(latitude);
        }
        if let Some(altitude) = other.altitude {
            self.altitude = Some(altitude);
        }
        if let Some(roll) = other.roll {
            self.roll = Some(roll);
        }
        if let Some(pitch) = other.pitch {
            self.pitch = Some(pitch);
        }
        if let Some(yaw) = other.yaw {
            self.yaw = Some(yaw);
        }
        if let Some(u) = other.u {
            self.u = Some(u);
        }
        if let Some(v) = other.v {
            self.v = Some(v);
        }
        if let Some(heading) = other.heading {
            self.heading = Some(heading);
        }
    }
}

impl FromStr for Coords {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('|');

        let longitude = tokens
            .next()
            .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
        let longitude = if longitude.is_empty() {
            None
        } else {
            Some(longitude.parse().map_err(Error::ParseFloat)?)
        };
        let latitude = tokens
            .next()
            .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
        let latitude = if latitude.is_empty() {
            None
        } else {
            Some(latitude.parse().map_err(Error::ParseFloat)?)
        };
        let altitude = tokens
            .next()
            .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
        let altitude = if altitude.is_empty() {
            None
        } else {
            Some(altitude.parse().map_err(Error::ParseFloat)?)
        };

        let v4 = tokens.next();
        if let Some(v4) = v4 {
            let v4 = if v4.is_empty() {
                None
            } else {
                Some(v4.parse().map_err(Error::ParseFloat)?)
            };
            let v5 = tokens
                .next()
                .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
            let v5 = if v5.is_empty() {
                None
            } else {
                Some(v5.parse().map_err(Error::ParseFloat)?)
            };

            let v6 = tokens.next();
            if let Some(v6) = v6 {
                let v6 = if v6.is_empty() {
                    None
                } else {
                    Some(v6.parse().map_err(Error::ParseFloat)?)
                };

                let v7 = tokens.next();
                if let Some(v7) = v7 {
                    let v7 = if v7.is_empty() {
                        None
                    } else {
                        Some(v7.parse().map_err(Error::ParseFloat)?)
                    };
                    let v8 = tokens
                        .next()
                        .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
                    let v8 = if v8.is_empty() {
                        None
                    } else {
                        Some(v8.parse().map_err(Error::ParseFloat)?)
                    };
                    let v9 = tokens
                        .next()
                        .ok_or_else(|| Error::MalformedCoords(s.to_string()))?;
                    let v9 = if v9.is_empty() {
                        None
                    } else {
                        Some(v9.parse().map_err(Error::ParseFloat)?)
                    };

                    Ok(Self {
                        longitude,
                        latitude,
                        altitude,
                        roll: v4,
                        pitch: v5,
                        yaw: v6,
                        u: v7,
                        v: v8,
                        heading: v9,
                    })
                } else {
                    Ok(Self {
                        longitude,
                        latitude,
                        altitude,
                        roll: v4,
                        pitch: v5,
                        yaw: v6,
                        u: None,
                        v: None,
                        heading: None,
                    })
                }
            } else {
                Ok(Self {
                    longitude,
                    latitude,
                    altitude,
                    roll: None,
                    pitch: None,
                    yaw: None,
                    u: v4,
                    v: v5,
                    heading: None,
                })
            }
        } else {
            Ok(Self {
                longitude,
                latitude,
                altitude,
                roll: None,
                pitch: None,
                yaw: None,
                u: None,
                v: None,
                heading: None,
            })
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    // Class
    Air,
    Ground,
    Sea,
    Weapon,
    Sensor,
    Navaid,
    Misc,

    // Attributes
    Static,
    Heavy,
    Medium,
    Light,
    Minor,

    // Basic Types
    FixedWing,
    Rotorcraft,
    Armor,
    AntiAircraft,
    Vehicle,
    Watercraft,
    Human,
    Biologic,
    Missile,
    Rocket,
    Bomb,
    Torpedo,
    Projectile,
    Beam,
    Decoy,
    Building,
    Bullseye,
    Waypoint,

    // Specific Types
    Tank,
    Warship,
    AircraftCarrier,
    Submarine,
    Infantry,
    Parachutist,
    Shell,
    Bullet,
    Grenade,
    Flare,
    Chaff,
    SmokeGrenade,
    Aerodrome,
    Container,
    Shrapnel,
    Explosion,

    #[serde(rename = "other")]
    Other(String),
}

impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Air" => Ok(Self::Air),
            "Ground" => Ok(Self::Ground),
            "Sea" => Ok(Self::Sea),
            "Weapon" => Ok(Self::Weapon),
            "Sensor" => Ok(Self::Sensor),
            "Navaid" => Ok(Self::Navaid),
            "Misc" => Ok(Self::Misc),
            "Static" => Ok(Self::Static),
            "Heavy" => Ok(Self::Heavy),
            "Medium" => Ok(Self::Medium),
            "Light" => Ok(Self::Light),
            "Minor" => Ok(Self::Minor),
            "FixedWing" => Ok(Self::FixedWing),
            "Rotorcraft" => Ok(Self::Rotorcraft),
            "Armor" => Ok(Self::Armor),
            "AntiAircraft" => Ok(Self::AntiAircraft),
            "Vehicle" => Ok(Self::Vehicle),
            "Watercraft" => Ok(Self::Watercraft),
            "Human" => Ok(Self::Human),
            "Biologic" => Ok(Self::Biologic),
            "Missile" => Ok(Self::Missile),
            "Rocket" => Ok(Self::Rocket),
            "Bomb" => Ok(Self::Bomb),
            "Torpedo" => Ok(Self::Torpedo),
            "Projectile" => Ok(Self::Projectile),
            "Beam" => Ok(Self::Beam),
            "Decoy" => Ok(Self::Decoy),
            "Building" => Ok(Self::Building),
            "Bullseye" => Ok(Self::Bullseye),
            "Waypoint" => Ok(Self::Waypoint),
            "Tank" => Ok(Self::Tank),
            "Warship" => Ok(Self::Warship),
            "AircraftCarrier" => Ok(Self::AircraftCarrier),
            "Submarine" => Ok(Self::Submarine),
            "Infantry" => Ok(Self::Infantry),
            "Parachutist" => Ok(Self::Parachutist),
            "Shell" => Ok(Self::Shell),
            "Bullet" => Ok(Self::Bullet),
            "Grenade" => Ok(Self::Grenade),
            "Flare" => Ok(Self::Flare),
            "Chaff" => Ok(Self::Chaff),
            "SmokeGrenade" => Ok(Self::SmokeGrenade),
            "Aerodrome" => Ok(Self::Aerodrome),
            "Container" => Ok(Self::Container),
            "Shrapnel" => Ok(Self::Shrapnel),
            "Explosion" => Ok(Self::Explosion),
            _ => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Violet,

    #[serde(rename = "other")]
    Other(String),
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Self::Red),
            "Orange" => Ok(Self::Orange),
            "Yellow" => Ok(Self::Yellow),
            "Green" => Ok(Self::Green),
            "Cyan" => Ok(Self::Cyan),
            "Blue" => Ok(Self::Blue),
            "Violet" => Ok(Self::Violet),
            color => Ok(Self::Other(color.to_string())),
        }
    }
}
