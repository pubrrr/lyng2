use async_graphql::{EmptyMutation, EmptySubscription, Object};

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self) -> String {
        String::from("test")
    }

    async fn do_test(&self) -> String {
        build_schema().sdl()
    }
}
