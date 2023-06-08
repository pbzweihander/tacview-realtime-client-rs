use std::str::FromStr;

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

    println!("{:#?}", reader.header);

    loop {
        let record = reader.next().await.expect("failed to read next record");

        println!("{record:?}");
    }
}
