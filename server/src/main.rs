use std::fs::File;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error, info, warn, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger, ThreadLogMode, WriteLogger};
use websocket::sync::Server;
use websocket::{CloseData, OwnedMessage, WebSocketError, WebSocketResult};

use crate::application::Application;

mod application;
mod ast;

fn main() {
    setup_logger();

    let mut server = Server::bind("127.0.0.1:8080").unwrap();

    while let Ok(request) = server.accept() {
        let client = request.accept().unwrap();

        let ip = client.peer_addr().unwrap();
        info!("{} connected", ip);

        let (mut reader, mut writer) = client.split().unwrap();

        thread::Builder::new()
            .name(format!("WebSocketHandler-{ip}"))
            .spawn(move || {
                let mut connection_closed = false;
                while !connection_closed {
                    for message in reader.incoming_messages() {
                        info!("got message: {message:?}");
                        let process_result = process(message);

                        if let Some(response) = process_result {
                            writer.send_message(&response).unwrap();

                            if let OwnedMessage::Close(data) = response {
                                info!("{} disconnected: {:?}", ip, data);
                                connection_closed = true;
                                break;
                            }
                        }
                    }

                    sleep(Duration::from_millis(100));
                }
                info!("shutting down thread for ip: {:?}", ip);
            })
            .unwrap();
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
            _ => {
                warn!("{:?}", message);
                unimplemented!("{:?}", message)
            }
        },
        Err(WebSocketError::NoDataAvailable) => None,
        Err(err) => {
            error!("{}", err);
            Some(OwnedMessage::Close(Some(CloseData::new(
                1001,
                err.to_string(),
            ))))
        }
    }
}

fn setup_logger() {
    let config = ConfigBuilder::default()
        .set_thread_level(LevelFilter::Error)
        .set_thread_mode(ThreadLogMode::Both)
        .build();

    let current_date_time = chrono::Local::now().format("%Y-%m-%d_%H-%M");
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            config.clone(),
            File::create(format!("logs/log-{}.txt", current_date_time)).unwrap(),
        ),
        SimpleLogger::new(LevelFilter::Debug, config),
    ])
    .unwrap();
}
