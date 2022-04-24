use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use websocket::sync::{Client, Server};
use websocket::{CloseData, OwnedMessage, WebSocketError, WebSocketResult};

use crate::application::{Application, CommandResult, Context};

mod application;
mod ast;

fn main() {
    let mut server = Server::bind("127.0.0.1:8080").unwrap();

    let clients: HashMap<SocketAddr, (Client<TcpStream>, Context)> = HashMap::new();

    if let Ok(request) = server.accept() {
        let client = request.accept().unwrap();

        let ip = client.peer_addr().unwrap();
        println!("{} connected", ip);

        let (mut reader, mut writer) = client.split().unwrap();

        let mut connection_closed = false;

        while !connection_closed {
            for message in reader.incoming_messages() {
                let process_result = process(message);

                if let Some(response) = process_result {
                    writer.send_message(&response).unwrap();

                    if let OwnedMessage::Close(data) = response {
                        println!("{} disconnected: {:?}", ip, data);
                        connection_closed = true;
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

                let mut application = Application::create();
                let result = application.run(text.clone());

                let result = ron::to_string(&result).unwrap();
                println!("result: {result}");

                Some(OwnedMessage::Text(result))
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
