use async_graphql::{Context, EmptySubscription, Object};
use std::sync::Mutex;

type Users = Mutex<Vec<String>>;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> Schema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(Users::default())
        .finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn get_users<'a>(&self, ctx: &Context<'a>) -> Vec<String> {
        ctx.data_unchecked::<Users>().lock().unwrap().clone()
    }

    async fn do_test(&self) -> String {
        build_schema().sdl()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>) -> String {
        let mut users = ctx.data_unchecked::<Users>().lock().unwrap();
        let new_user = format!("User#{id}", id = users.len());
        users.push(new_user.clone());
        new_user
    }
}
