use std::fmt::Debug;
use std::sync::Mutex;

use async_graphql::async_stream::stream;
use async_graphql::extensions::Logger;
use async_graphql::{Context, Object, SimpleObject, Subscription};
use chrono::{DateTime, Local};
use futures_util::Stream;
use log::{debug, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use warp::http::header::SET_COOKIE;

use crate::chat::auth::{create_auth_token, AuthExtensionFactory, AuthUser, AUTH_COOKIE_NAME};

pub mod auth;
#[cfg(test)]
mod test;

type Users = Mutex<Vec<User>>;
type Streams<T> = Mutex<Vec<UnboundedSender<T>>>;

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    id: String,
    name: String,
}

#[derive(Clone, Debug, SimpleObject)]
pub struct Message {
    user: User,
    message: String,
    date: DateTime<Local>,
}

pub fn build_schema() -> Schema {
    Schema::build(Query, Mutation, Subscription)
        .data(Users::default())
        .data(Streams::<User>::default())
        .data(Streams::<Message>::default())
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

    async fn logged_in_user<'a>(&self, ctx: &Context<'a>) -> Option<User> {
        let users = ctx.data_unchecked::<Users>().lock().unwrap();
        let auth_user = ctx.data_opt::<AuthUser>()?;
        users.iter().find(|user| user.id == auth_user.id).cloned()
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

        let mut subscribers = ctx.data_unchecked::<Streams<User>>().lock().unwrap();
        notify_subscribers(new_user.clone(), &mut subscribers);
        info!("new user registered: {new_user:?}");

        let auth_cookie = create_auth_token(&new_user);
        ctx.insert_http_header(
            SET_COOKIE,
            format!("{AUTH_COOKIE_NAME}={auth_cookie}; SameSite=Strict; Secure"),
        );

        new_user
    }

    async fn send_message(&self, ctx: &Context<'_>, message: String) -> Message {
        info!("new message received: {message}");
        println!("new message received: {message}");
        let mut subscribers = ctx.data_unchecked::<Streams<Message>>().lock().unwrap();

        let user = get_user(ctx);
        info!("from user: {user:?}");
        println!("from user: {user:?}");

        let message = Message {
            user,
            message,
            date: Local::now(),
        };
        notify_subscribers(message.clone(), &mut subscribers);
        message
    }
}

fn get_user(ctx: &Context) -> User {
    let auth_user = ctx.data_unchecked::<AuthUser>();
    let users = ctx.data_unchecked::<Users>().lock().unwrap();
    users
        .iter()
        .find(|user| user.id == auth_user.id)
        .unwrap()
        .clone()
}

fn notify_subscribers<T: Clone + Debug>(message: T, subscribers: &mut Vec<UnboundedSender<T>>) {
    for (i, stream) in subscribers.clone().iter().enumerate() {
        debug!("notifying: {message:?}");
        match stream.send(message.clone()) {
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
        info!("new subscription for users");

        let (sender, mut receiver) = mpsc::unbounded_channel::<User>();

        let mut streams = ctx.data_unchecked::<Streams<User>>().lock().unwrap();
        streams.push(sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }

    async fn get_new_messages(&self, ctx: &Context<'_>) -> impl Stream<Item = Message> {
        info!("new subscription for messages");
        println!("new subscription for messages");

        let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();

        let mut streams = ctx.data_unchecked::<Streams<Message>>().lock().unwrap();
        streams.push(sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }
}
