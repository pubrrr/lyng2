use std::sync::Mutex;

use async_graphql::async_stream::stream;
use async_graphql::extensions::Logger;
use async_graphql::{Context, Object, SimpleObject, Subscription};
use futures_util::Stream;
use log::{debug, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use warp::http::header::SET_COOKIE;

use crate::chat::auth::{create_auth_token, AuthExtensionFactory, AUTH_COOKIE_NAME};

pub mod auth;

type Users = Mutex<Vec<User>>;
type Streams = Mutex<Vec<UnboundedSender<User>>>;

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    id: String,
    name: String,
}

pub fn build_schema() -> Schema {
    Schema::build(Query, Mutation, Subscription)
        .data(Users::default())
        .data(Streams::default())
        .extension(Logger)
        .extension(AuthExtensionFactory)
        .finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn get_users<'a>(&self, ctx: &Context<'a>) -> Vec<User> {
        ctx.data_unchecked::<Users>().lock().unwrap().clone()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>, name: String) -> User {
        let mut users = ctx.data_unchecked::<Users>().lock().unwrap();
        let new_user = User {
            id: format!("User#{id}", id = users.len()),
            name,
        };
        users.push(new_user.clone());

        let mut subscribers = ctx.data_unchecked::<Streams>().lock().unwrap();
        notify_subscribers(new_user.clone(), &mut subscribers);
        info!("new user registered: {new_user:?}");

        let auth_cookie = create_auth_token(&new_user);
        ctx.insert_http_header(
            SET_COOKIE,
            format!("{AUTH_COOKIE_NAME}={auth_cookie}; SameSite=Strict; Secure"),
        );

        new_user
    }
}

fn notify_subscribers(new_user: User, subscribers: &mut Vec<UnboundedSender<User>>) {
    for (i, stream) in subscribers.clone().iter().enumerate() {
        debug!("sending new user");
        match stream.send(new_user.clone()) {
            Err(_) if stream.is_closed() => {
                debug!("stream disconnected - removing it");
                subscribers.remove(i);
            }
            _ => {}
        };
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn get_new_users(&self, ctx: &Context<'_>) -> impl Stream<Item = User> {
        info!("new subscription");

        let (sender, mut receiver) = mpsc::unbounded_channel::<User>();

        let mut streams = ctx.data_unchecked::<Streams>().lock().unwrap();
        streams.push(sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }
}
