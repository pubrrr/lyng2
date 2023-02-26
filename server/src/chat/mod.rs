use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use tokio::sync::Mutex;

use async_graphql::async_stream::stream;
use async_graphql::extensions::Logger;
use async_graphql::{Context, Object, SimpleObject, Subscription};
use chrono::{DateTime, Local};
use futures_util::Stream;
use log::{debug, info};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use warp::http::header::SET_COOKIE;

use crate::chat::auth::{create_auth_token, AuthExtensionFactory, AuthUser, AUTH_COOKIE_NAME};
use crate::chat::repository::ChatRepository;

pub mod auth;
pub mod repository;
#[cfg(test)]
mod test;

type Streams<T> = Mutex<HashMap<User, UnboundedSender<T>>>;

pub type Schema<Repository> =
    async_graphql::Schema<Query<Repository>, Mutation<Repository>, Subscription<Repository>>;

#[derive(Clone, Debug, SimpleObject, Hash, Eq, PartialEq, Deserialize)]
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

pub fn build_schema<Repository: ChatRepository + 'static>() -> Schema<Repository> {
    Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(Repository::default())
    .data(Streams::<User>::default())
    .data(Streams::<Message>::default())
    .extension(Logger)
    .extension(AuthExtensionFactory)
    .finish()
}

#[derive(Default)]
pub struct Query<Repository> {
    phantom: PhantomData<Repository>,
}

#[Object]
impl<Repository: ChatRepository + 'static> Query<Repository> {
    async fn get_users<'a>(&self, ctx: &Context<'a>) -> Vec<User> {
        info!("calling get_users");
        ctx.data_unchecked::<Repository>().get_users().await
    }

    async fn logged_in_user<'a>(&self, ctx: &Context<'a>) -> Option<User> {
        let auth_user = ctx.data_opt::<AuthUser>()?;
        info!("calling logged_in_user");
        let user = ctx
            .data_unchecked::<Repository>()
            .get_user(&auth_user.id)
            .await;
        info!("found logged in user: {user:?}");
        user
    }
}

#[derive(Default)]
pub struct Mutation<Repository> {
    phantom: PhantomData<Repository>,
}

#[Object]
impl<Repository: ChatRepository + 'static> Mutation<Repository> {
    async fn register(&self, ctx: &Context<'_>, name: String) -> User {
        let new_user = ctx
            .data_unchecked::<Repository>()
            .register_new_user(name)
            .await;

        let mut subscribers = ctx.data_unchecked::<Streams<User>>().lock().await;
        notify_subscribers(new_user.clone(), &new_user, &mut subscribers);
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
        let mut subscribers = ctx.data_unchecked::<Streams<Message>>().lock().await;

        let user = get_user::<Repository>(ctx).await;
        info!("from user: {user:?}");

        let message = Message {
            user: user.clone(),
            message,
            date: Local::now(),
        };
        notify_subscribers(message.clone(), &user, &mut subscribers);
        message
    }
}

async fn get_user<Repository: ChatRepository + 'static>(ctx: &Context<'_>) -> User {
    let auth_user = ctx.data_unchecked::<AuthUser>();
    ctx.data_unchecked::<Repository>()
        .get_user(&auth_user.id)
        .await
        .unwrap()
}

fn notify_subscribers<T: Clone + Debug>(
    message: T,
    acting_user: &User,
    subscribers: &mut HashMap<User, UnboundedSender<T>>,
) {
    let disconnected_users: Vec<_> = subscribers
        .iter()
        .filter(|(stream_user, _)| *stream_user != acting_user)
        .filter_map(|(stream_user, stream)| {
            send_message_and_check_for_disconnect(message.clone(), stream_user, stream)
        })
        .cloned()
        .collect();

    subscribers.retain(|stream_user, _| !disconnected_users.contains(stream_user))
}

fn send_message_and_check_for_disconnect<'a, T: Clone + Debug>(
    message: T,
    stream_user: &'a User,
    stream: &'a UnboundedSender<T>,
) -> Option<&'a User> {
    debug!("notifying: {message:?}");
    match stream.send(message) {
        Err(_) if stream.is_closed() => {
            debug!("stream disconnected - removing it");
            Some(stream_user)
        }
        _ => None,
    }
}

#[derive(Default)]
pub struct Subscription<Repository> {
    phantom: PhantomData<Repository>,
}

#[Subscription]
impl<Repository: ChatRepository + 'static> Subscription<Repository> {
    async fn get_new_users(&self, ctx: &Context<'_>) -> impl Stream<Item = User> {
        info!("new subscription for users");

        let (sender, mut receiver) = mpsc::unbounded_channel::<User>();

        let mut streams = ctx.data_unchecked::<Streams<User>>().lock().await;
        streams.insert(get_user::<Repository>(ctx).await, sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }

    async fn get_new_messages(&self, ctx: &Context<'_>) -> impl Stream<Item = Message> {
        info!("new subscription for messages");

        let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();

        let mut streams = ctx.data_unchecked::<Streams<Message>>().lock().await;
        streams.insert(get_user::<Repository>(ctx).await, sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }
}
