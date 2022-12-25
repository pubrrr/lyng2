use std::fmt::Debug;

use async_graphql::Value::Null;
use async_graphql::{Request, Response, ServerError, Value};
use futures_lite::future::poll_once;
use futures_util::{Stream, StreamExt};

use crate::chat::auth::AuthUser;
use crate::chat::{build_schema, Schema};
use crate::from_json;

#[tokio::test]
async fn test_register_user() {
    let schema = build_schema();

    let response = register_user(&schema).await;

    assert_no_error(&response);
    assert_eq!(
        from_json!({"register": {"id": "User#0", "name": "user name"}}),
        response.data,
    );
}

#[tokio::test]
async fn get_users_without_authentication_fails() {
    let schema = build_schema();

    let response = schema.execute("{ getUsers { id } }").await;

    assert_missing_authentication(response);
}

#[tokio::test]
async fn get_users_with_authentication_returns_registered_users() {
    let schema = build_schema();
    register_user(&schema).await;
    register_user(&schema).await;

    let request = Request::from("{ getUsers { id } }").data(some_auth_user());
    let response = schema.execute(request).await;

    assert_no_error(&response);
    assert_eq!(
        from_json!({"getUsers": [{"id": "User#0"}, {"id": "User#1"}]}),
        response.data,
    );
}

#[tokio::test]
async fn get_logged_in_user_with_authentication_returns_user() {
    let schema = build_schema();
    register_user(&schema).await;

    let request = Request::from("{ loggedInUser { id } }").data(some_auth_user());
    let response: Response = schema.execute(request).await;

    assert_no_error(&response);
    assert_eq!(
        from_json!({"loggedInUser": {"id": "User#0"}}),
        response.data,
    );
}

#[tokio::test]
async fn get_logged_in_user_without_authentication_returns_no_data() {
    let schema = build_schema();
    register_user(&schema).await;

    let request = Request::from("{ loggedInUser { id } }");
    let response: Response = schema.execute(request).await;

    assert_no_error(&response);
    assert_eq!(from_json!({ "loggedInUser": null }), response.data,);
}

mod subscribe_to_new_users {
    use super::*;

    #[tokio::test]
    async fn get_new_users_with_authentication() {
        let schema = build_schema();
        register_user(&schema).await;

        let request =
            Request::from("subscription { getNewUsers { id, name } }").data(some_auth_user());
        let mut stream = schema.execute_stream(request);
        poll_once_to_make_stream_perform_work(&mut stream).await;

        let response = register_user(&schema).await;
        assert_no_error(&response);

        let response: Response = stream.next().await.unwrap();

        assert_no_error(&response);
        assert_eq!(
            from_json!({ "getNewUsers": {"id": "User#1", "name": "user name"} }),
            response.data,
        );
    }

    #[tokio::test]
    async fn get_new_users_without_authentication_denies_access() {
        let schema = build_schema();

        let mut stream = schema.execute_stream("subscription { getNewUsers { id, name } }");
        let response = stream.next().await.unwrap();

        assert_missing_authentication(response);
    }
}

mod send_messages {
    use std::str::FromStr;

    use async_graphql::Value::{Object, String};
    use chrono::{DateTime, Local};

    use super::*;

    #[tokio::test]
    async fn send_message_without_authentication_fails() {
        let schema = build_schema();

        let response = schema
            .execute("mutation { sendMessage(message: \"test message\") }")
            .await;

        assert_missing_authentication(response);
    }

    #[tokio::test]
    async fn subscribe_to_messages_without_authentication_fails() {
        let schema = build_schema();

        let mut stream = schema.execute_stream("subscription { getNewMessages }");
        let response = stream.next().await.unwrap();

        assert_missing_authentication(response);
    }

    #[tokio::test]
    async fn subscribe_to_messages_receives_sent_messages() {
        let schema = build_schema();
        register_user(&schema).await;
        register_user(&schema).await;

        let x = send_message_as_user(schema, "User#1").await;
        let response: Response = x.unwrap();

        assert_no_error(&response);
        assert_eq!(
            from_json!({ "getNewMessages": {
                "user": {"id": "User#1", "name": "user name"},
                "message": "test message"
            }}),
            response.data,
        );
    }

    #[tokio::test]
    async fn subscribe_to_messages_does_not_receive_sent_messages_from_itself() {
        let schema = build_schema();
        register_user(&schema).await;

        let response = send_message_as_user(schema, "User#0").await;

        assert_eq!(None, response);
    }

    async fn send_message_as_user(schema: Schema, user: &str) -> Option<Response> {
        let request =
            Request::from("subscription { getNewMessages { message,  user { id, name } } }")
                .data(auth_user("User#0".to_string()));
        let mut stream = schema.execute_stream(request);
        poll_once_to_make_stream_perform_work(&mut stream).await;

        let request =
            Request::from("mutation { sendMessage(message: \"test message\") { message } }")
                .data(auth_user(user.to_string()));
        let sent_message = schema.execute(request).await;
        assert_no_error(&sent_message);

        match poll_once(stream.next()).await {
            Some(Some(o)) => Some(o),
            _ => None,
        }
    }

    #[tokio::test]
    async fn date_of_sent_message() {
        let schema = build_schema();
        register_user(&schema).await;

        let request = Request::from("mutation { sendMessage(message: \"test message\") { date } }")
            .data(some_auth_user());
        let sent_message = schema.execute(request).await;
        assert_no_error(&sent_message);
        match &sent_message.data {
            Object(send_message) => match &send_message["sendMessage"] {
                Object(message) => {
                    if let String(date) = &message["date"] {
                        DateTime::<Local>::from_str(date).unwrap();
                        return;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        panic!("Did not find expected date in {:?}", sent_message.data);
    }
}

async fn poll_once_to_make_stream_perform_work<
    I: PartialEq + Debug,
    S: Stream<Item = I> + Unpin,
>(
    stream: &mut S,
) -> Option<Option<S::Item>> {
    let response = poll_once(stream.next()).await;
    assert_eq!(None, response, "{response:?}");
    response
}

fn assert_no_error(response: &Response) {
    assert_eq!(0, response.errors.len(), "{:?}", response.errors)
}

fn assert_missing_authentication(response: Response) {
    assert_eq!(Null, response.data);
    assert_eq!(1, response.errors.len());
    assert_eq!(
        ServerError::new("Missing authentication for operation", None),
        response.errors[0]
    )
}

async fn register_user(schema: &Schema) -> Response {
    schema
        .execute("mutation { register(name:\"user name\") { id, name } }")
        .await
}

fn some_auth_user() -> AuthUser {
    auth_user("User#0".to_string())
}

fn auth_user(user: String) -> AuthUser {
    AuthUser { id: user }
}

#[macro_export]
macro_rules! from_json {
    ($($json:tt)+) => {
        Value::from_json(serde_json::json!($($json)+)).unwrap()
    };
}
