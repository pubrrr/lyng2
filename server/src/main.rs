use std::convert::Infallible;
use std::fs::File;

use async_graphql::http::GraphiQLSource;
use async_graphql::Request;
use async_graphql_warp::GraphQLResponse;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger, ThreadLogMode, WriteLogger};
use warp::http::Response;
use warp::ws::{Message, WebSocket, Ws};
use warp::Filter;

use crate::application::Application;
use crate::chat::{build_schema, Schema};

mod application;
mod ast;
mod chat;

#[tokio::main]
async fn main() {
    setup_logger();

    let math_websocket_route = warp::path("math")
        .and(warp::ws())
        .map(|handshake: Ws| handshake.on_upgrade(handle_connection));

    let chat_routes = async_graphql_warp::graphql(build_schema()).and_then(
        |(schema, request): (Schema, Request)| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("http://localhost:8080")
                    .finish(),
            )
    });

    let routes = math_websocket_route.or(chat_routes).or(graphiql);

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
