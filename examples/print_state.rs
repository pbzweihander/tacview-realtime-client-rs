use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
    str::FromStr,
};

use tacview_realtime_client::acmi::{
    record::{global_property::GlobalProperty, object_property::ObjectProperty, Record},
    Header,
};

#[derive(Debug)]
struct State {
    #[allow(dead_code)]
    acmi_header: Header,
    global_properties: HashMap<Discriminant<GlobalProperty>, GlobalProperty>,
    objects: HashMap<u64, HashMap<Discriminant<ObjectProperty>, ObjectProperty>>,
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let cmd = args.next().unwrap();
    macro_rules! next_arg {
        () => {
            args.next().unwrap_or_else(|| {
                panic!("USAGE: {} <HOSTNAME> <PORT> <USERNAME> [<PASSWORD>]", cmd)
            })
        };
    }
    let host = next_arg!();
    let port = u16::from_str(&next_arg!()).expect("bad port");
    let username = next_arg!();
    let password = args.next().unwrap_or_default();

    let mut reader = tacview_realtime_client::connect((host, port), &username, &password)
        .await
        .expect("failed to connect");

    let mut state = State {
        acmi_header: reader.header.clone(),
        global_properties: HashMap::new(),
        objects: HashMap::new(),
    };

    loop {
        let record = reader.next().await.expect("failed to read next record");

        match record {
            Record::Remove(id) => {
                state.objects.remove(&id);
            }
            Record::Frame(timeframe) => {
                println!("new timeframe: {timeframe}");
            }
            Record::Event(event) => {
                println!("new event: {event:?}");
            }
            Record::GlobalProperties(global_properties) => {
                for global_property in global_properties {
                    state
                        .global_properties
                        .insert(discriminant(&global_property), global_property);
                }
            }
            Record::Update(id, object_properties) => {
                let entry = state.objects.entry(id).or_default();
                for object_property in object_properties {
                    entry.insert(discriminant(&object_property), object_property);
                }
            }
        }

        println!("header: {:?}", state.acmi_header);
        println!("global_properties: {:?}", state.global_properties);
        for (id, properties) in &state.objects {
            println!("object {}: {:?}", id, properties);
        }
        println!();
    }
}
