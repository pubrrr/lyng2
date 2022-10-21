use std::convert::Infallible;
use std::fs::File;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql::Request;
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use log::LevelFilter;
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger, ThreadLogMode, WriteLogger};
use warp::http::Response;
use warp::ws::Ws;
use warp::{Filter, Rejection, Reply};

use lyng2::chat::auth::with_auth;
use lyng2::chat::{build_schema, Schema};
use lyng2::math::handle_websocket_connection;

#[tokio::main]
async fn main() {
    setup_logger();

    let routes = api_routes()
        .or(static_files_route())
        .or(catch_all_index_html_route())
        .with(warp::log("lyng::api"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn api_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let schema = build_schema();

    let routes = lyng2_route()
        .or(graphql_subscription(schema.clone()).and(warp::path("chat")))
        .or(chat_route(schema))
        .or(playground_route())
        .or(graphiql_route());

    warp::path("api").and(routes)
}

fn lyng2_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("math")
        .and(warp::ws())
        .map(|handshake: Ws| handshake.on_upgrade(handle_websocket_connection))
}

fn chat_route(schema: Schema) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async_graphql_warp::graphql(schema)
        .and(warp::path("chat"))
        .and(with_auth())
        .and_then(
            |(schema, request): (Schema, Request), auth_token| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(
                    schema.execute(request.data(auth_token)).await,
                ))
            },
        )
}

fn graphiql_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("graphiql").and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("chat/")
                    .subscription_endpoint("ws://localhost:8080/api/chat")
                    .finish(),
            )
    })
}

fn playground_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("playground").and(warp::get()).map(|| {
        let config = GraphQLPlaygroundConfig::new("chat/")
            .subscription_endpoint("ws://localhost:8080/api/chat/");
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(config))
    })
}

fn static_files_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(warp::fs::dir("../react-client/build"))
}

fn catch_all_index_html_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(warp::fs::file("../react-client/build/index.html"))
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
