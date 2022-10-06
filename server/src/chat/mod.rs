use async_graphql::async_stream::stream;
use async_graphql::{Context, EmptySubscription, Object, Subscription};
use futures_util::Stream;
use log::{debug, info};
use std::ops::Deref;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

type Users = Mutex<Vec<String>>;
type Streams = Mutex<Vec<UnboundedSender<String>>>;

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

pub fn build_schema() -> Schema {
    Schema::build(Query, Mutation, Subscription)
        .data(Users::default())
        .data(Streams::default())
        .finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn get_users<'a>(&self, ctx: &Context<'a>) -> Vec<String> {
        info!("getting all users");
        ctx.data_unchecked::<Users>().lock().unwrap().clone()
    }

    async fn do_test(&self) -> String {
        info!("getting schema");

        build_schema().sdl()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>) -> String {
        info!("registering");
        let mut users = ctx.data_unchecked::<Users>().lock().unwrap();
        let new_user = format!("User#{id}", id = users.len());
        users.push(new_user.clone());

        let mut streams = ctx.data_unchecked::<Streams>().lock().unwrap();
        for stream in streams.deref() {
            debug!("sending new user");
            stream.send(new_user.clone()).unwrap();
        }
        info!("new user registered: {new_user}");

        new_user
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn get_new_users(&self, ctx: &Context<'_>) -> impl Stream<Item = String> {
        info!("new subscription");

        let (sender, mut receiver) = mpsc::unbounded_channel::<String>();

        let mut streams = ctx.data_unchecked::<Streams>().lock().unwrap();
        streams.push(sender);

        stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        }
    }
}
