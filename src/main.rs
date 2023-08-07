use tungstenite::{connect, Message};
use url::Url;
use serde_json::json;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn main() {
    let mut pings: Vec<u128> = vec![];
    let (mut socket, _response) = connect(
        Url::parse("wss://ws-api.binance.com:443/ws-api/v3").unwrap()
    ).expect("Can't connect");

    
    for i in 0..100 {

        socket.write_message(Message::Text(json!({
            "id": 123,
            "method": "time"
        }).to_string())).unwrap();

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // println!("{:?}", timestamp.as_micros());
        // println!("{:?}", timestamp.as_millis());

        let message = socket.read_message().unwrap();
        let timestamp_new = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // println!("{:?}", timestamp_new.as_micros());
        pings.push(timestamp_new.as_micros() - timestamp.as_micros());
    }

    let mut sum: u128 = 0;

    println!("{:?}", pings);
    for i in pings {
        sum += i;
    }
    println!("{}", sum / 100);

}
