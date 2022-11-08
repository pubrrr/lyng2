mod application;
mod ast;

use crate::math::application::Application;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info};
use warp::ws::{Message, WebSocket};

pub async fn handle_websocket_connection(websocket: WebSocket) {
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
