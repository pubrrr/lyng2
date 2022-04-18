use std::thread::sleep;
use std::time::Duration;

use websocket::sync::Server;
use websocket::{CloseData, OwnedMessage, WebSocketError, WebSocketResult};

fn main() {
    let mut server = Server::bind("127.0.0.1:8080").unwrap();

    if let Ok(request) = server.accept() {
        let client = request.accept().unwrap();

        let ip = client.peer_addr().unwrap();
        println!("{} connected", ip);

        let (mut reader, mut writer) = client.split().unwrap();

        'listener: loop {
            for message in reader.incoming_messages() {
                let process_result = process(message);

                if let Some(response) = process_result {
                    writer.send_message(&response).unwrap();

                    if let OwnedMessage::Close(data) = response {
                        println!("{} disconnected: {:?}", ip, data);
                        break 'listener;
                    }
                }
            }

            sleep(Duration::from_millis(100));
        }
    }
}

fn process(message: WebSocketResult<OwnedMessage>) -> Option<OwnedMessage> {
    match message {
        Ok(message) => match message {
            OwnedMessage::Text(text) => {
                println!("text: {text}");
                Some(OwnedMessage::Text(text))
            }
            OwnedMessage::Close(data) => Some(OwnedMessage::Close(data)),
            OwnedMessage::Ping(ping) => Some(OwnedMessage::Pong(ping)),
            _ => unimplemented!("{:?}", message),
        },
        Err(WebSocketError::NoDataAvailable) => None,
        Err(err) => Some(OwnedMessage::Close(Some(CloseData::new(
            1001,
            err.to_string(),
        )))),
    }
}
