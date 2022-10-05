use std::fs::File;

use log::{debug, error, info, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger, ThreadLogMode, WriteLogger};
use warp::ws::{Message, WebSocket, Ws};
use warp::Filter;

use futures_util::{SinkExt, StreamExt, TryFutureExt};

use crate::application::Application;

mod application;
mod ast;

#[tokio::main]
async fn main() {
    setup_logger();

    let routes = warp::path("math")
        .and(warp::ws())
        .map(|handshake: Ws| handshake.on_upgrade(handle_connection));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_connection(websocket: WebSocket) {
    info!("New websocket connection");

    let (mut writer, mut reader) = websocket.split();

    tokio::task::spawn(async move {
        while let Some(incoming) = reader.next().await {
            let message: Message = match incoming {
                Ok(msg) => msg,
                Err(e) => {
                    error!("websocket error: {}", e);
                    break;
                }
            };

            let response = process(message);

            writer
                .send(response)
                .unwrap_or_else(|e| {
                    error!("websocket send error: {}", e);
                })
                .await;
        }
    });
}

fn process(message: Message) -> Message {
    let response = match message.to_str() {
        Ok(input) => {
            debug!("text: {input}");
            let result = Application::create().run(input.to_string());

            let result = ron::to_string(&result).unwrap();
            debug!("result: {result}");
            result
        }
        Err(_) => format!("Can't deal with message: {message:?}"),
    };

    Message::text(response)
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
