use tungstenite::{connect, Message};
use url::Url;
use serde_json::json;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn main() {
    let (mut socket, _response) = connect(
        Url::parse("wss://ws-api.binance.com:443/ws-api/v3").unwrap()
    ).expect("Can't connect");

    socket.write_message(Message::Text(json!({
        "id": 123,
        "method": "time"
    }).to_string())).unwrap();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", timestamp.as_micros());
    println!("{:?}", timestamp.as_millis());

    let message = socket.read_message().unwrap();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", timestamp.as_micros());
    println!("{:?}", message);
}
