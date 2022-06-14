use std::collections::HashMap;
use std::fs::File;
use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use log::{debug, info, LevelFilter};
use simplelog::{CombinedLogger, Config, SimpleLogger, WriteLogger};
use websocket::sync::{Client, Server};
use websocket::{CloseData, OwnedMessage, WebSocketError, WebSocketResult};

use crate::application::{Application, Context};

mod application;
mod ast;

fn main() {
    setup_logger();

    let mut server = Server::bind("127.0.0.1:8080").unwrap();

    let _clients: HashMap<SocketAddr, (Client<TcpStream>, Context)> = HashMap::new();

    if let Ok(request) = server.accept() {
        let client = request.accept().unwrap();

        let ip = client.peer_addr().unwrap();
        info!("{} connected", ip);

        let (mut reader, mut writer) = client.split().unwrap();

        let mut connection_closed = false;

        while !connection_closed {
            for message in reader.incoming_messages() {
                let process_result = process(message);

                if let Some(response) = process_result {
                    writer.send_message(&response).unwrap();

                    if let OwnedMessage::Close(data) = response {
                        info!("{} disconnected: {:?}", ip, data);
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
                debug!("text: {text}");

                let mut application = Application::create();
                let result = application.run(text);

                let result = ron::to_string(&result).unwrap();
                debug!("result: {result}");

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

fn setup_logger() {
    let current_date_time = chrono::Local::now().format("%Y-%m-%d_%H-%M");
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(format!("logs/log-{}.txt", current_date_time)).unwrap(),
        ),
        SimpleLogger::new(LevelFilter::Debug, Config::default()),
    ])
    .unwrap();
}
