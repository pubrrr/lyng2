use std::convert::Infallible;
use std::fs::File;
use std::net::SocketAddr;
use std::str::FromStr;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql::{Data, Request};
use async_graphql_warp::{graphql_protocol, GraphQLResponse, GraphQLWebSocket};
use log::{info, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger, ThreadLogMode, WriteLogger};
use warp::http::Response;
use warp::ws::Ws;
use warp::{Filter, Rejection, Reply};
#[cfg(feature = "watch_mode")]
use warp_reverse_proxy::reverse_proxy_filter;

use lyng2::chat::auth::{with_auth, AuthUser};
use lyng2::chat::{build_schema, Schema};
use lyng2::math::handle_websocket_connection;

#[tokio::main]
async fn main() {
    setup_logger();

    #[cfg(feature = "watch_mode")]
    let routes = api_routes().or(reverse_proxy_filter(
        "".to_string(),
        "http://localhost:3000".to_string(),
    ));
    #[cfg(not(feature = "watch_mode"))]
    let routes = api_routes().or(static_files_route());
    let server = warp::serve(routes.with(warp::log("lyng::api")));

    if let Ok(cert_path) = std::env::var("CERT_PATH") {
        info!("starting TLS server - using certificates from {cert_path}");
        server
            .tls()
            .cert_path(format!("{cert_path}/cert.pem"))
            .key_path(format!("{cert_path}/key.rsa"))
            .run(address())
            .await;
    } else {
        server.run(address()).await;
    }
}

fn address() -> SocketAddr {
    std::env::var("ADDRESS")
        .ok()
        .map(|address| SocketAddr::from_str(&address).unwrap())
        .or_else(|| {
            std::env::args().nth(1).map(|address| {
                SocketAddr::from_str(&address)
                    .unwrap_or_else(|_| panic!("{address} was no valid socket address"))
            })
        })
        .unwrap_or_else(|| ([127, 0, 0, 1], 8080).into())
}

fn api_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let schema = build_schema();

    let routes = lyng2_route()
        .or(chat_subscription_route(schema.clone()))
        .or(chat_route(schema))
        .or(playground_route())
        .or(graphiql_route());

    warp::path("api").and(routes)
}

fn chat_subscription_route(
    schema: Schema,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::ws()
        .and(warp::path("chat"))
        .and(graphql_protocol())
        .and(with_auth())
        .map(move |ws: Ws, protocol, auth_token| {
            let schema = schema.clone();

            let reply = ws.on_upgrade(move |socket| {
                GraphQLWebSocket::new(socket, schema, protocol)
                    .with_data(data_with(auth_token))
                    .serve()
            });

            warp::reply::with_header(
                reply,
                "Sec-WebSocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        })
}

fn data_with(auth_token: Option<AuthUser>) -> Data {
    let mut data = Data::default();
    if let Some(auth_token) = auth_token {
        data.insert(auth_token);
    }
    data
}

fn lyng2_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("math")
        .and(warp::ws())
        .map(|handshake: Ws| handshake.on_upgrade(handle_websocket_connection))
}

fn chat_route(schema: Schema) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    async_graphql_warp::graphql(schema)
        .and(warp::path("chat"))
        .and(with_auth())
        .and_then(
            |(schema, request): (Schema, Request), auth_token| async move {
                let request = match auth_token {
                    None => request,
                    Some(auth_user) => request.data(auth_user),
                };
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        )
}

fn graphiql_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("graphiql").and(warp::get()).map(|| {
        let subscription_endpoint = subscription_endpoint();
        Response::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("api/chat/")
                    .subscription_endpoint(&subscription_endpoint)
                    .finish(),
            )
    })
}

fn playground_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("playground").and(warp::get()).map(|| {
        let subscription_endpoint = subscription_endpoint();
        let config =
            GraphQLPlaygroundConfig::new("api/chat/").subscription_endpoint(&subscription_endpoint);
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(config))
    })
}

fn subscription_endpoint() -> String {
    format!("ws://{host}/api/chat", host = address())
}

#[cfg(not(feature = "watch_mode"))]
fn static_files_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and(warp::fs::dir("../client/build"))
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
            File::create(format!("logs/log-{current_date_time}.txt")).unwrap(),
        ),
        SimpleLogger::new(LevelFilter::Debug, config),
    ])
    .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::address;

    #[test]
    fn test_address() {
        assert_eq!("127.0.0.1:8080", format!("{}", address()));
    }
}
