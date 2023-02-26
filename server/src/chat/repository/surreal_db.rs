use async_trait::async_trait;
use futures::executor::block_on;
use log::{debug, info};
use serde_json::json;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::thing;
use surrealdb::{Connection, Error, Surreal};

use crate::chat::repository::ChatRepository;
use crate::chat::User;

pub struct SurrealDbAdapter(GenericSurrealDbAdapter<Client>);

impl Default for SurrealDbAdapter {
    fn default() -> Self {
        SurrealDbAdapter(GenericSurrealDbAdapter {
            database: block_on(get_db()).unwrap(),
        })
    }
}

#[async_trait]
impl ChatRepository for SurrealDbAdapter {
    async fn get_users(&self) -> Vec<User> {
        self.0.get_users().await
    }

    async fn get_user(&self, id: &str) -> Option<User> {
        self.0.get_user(id).await
    }

    async fn register_new_user(&self, name: String) -> User {
        self.0.register_new_user(name).await
    }
}

struct GenericSurrealDbAdapter<C: Connection> {
    pub database: Surreal<C>,
}

const CHAT_USER: &'static str = "chat_user";

impl<C: Connection> GenericSurrealDbAdapter<C> {
    async fn get_users(&self) -> Vec<User> {
        info!("getting all users");
        self.database.select(CHAT_USER).await.unwrap()
    }

    async fn get_user(&self, id: &str) -> Option<User> {
        info!("getting user {id}");
        let thing = thing(id).ok()?;
        if thing.tb != CHAT_USER {
            return None;
        }
        return self.database.select(thing).await.unwrap();
    }

    async fn register_new_user(&self, name: String) -> User {
        info!("new user {name}");
        self.database
            .create(CHAT_USER)
            .content(json!({ "name": name }))
            .await
            .unwrap()
    }
}

async fn get_db() -> Result<Surreal<Client>, Error> {
    let surreal_address = "localhost:8000";
    debug!("connecting to SurrealDB at {surreal_address}");
    let database = Surreal::new::<Ws>(surreal_address).await?;

    database
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    database.use_ns("namespace").use_db("database").await?;
    debug!("connected");

    Ok(database)
}

#[cfg(test)]
mod tests {
    use surrealdb::engine::any::{connect, Any};

    use crate::chat::repository::surreal_db::GenericSurrealDbAdapter;

    #[tokio::test]
    async fn database_initially_returns_no_data() {
        let under_test = get_test_db_adapter().await;

        assert!(under_test.get_users().await.is_empty());
        assert_eq!(under_test.get_user("user id").await, None);
    }

    #[tokio::test]
    async fn register_user_and_query_it() {
        let under_test = get_test_db_adapter().await;

        let name = "my name";
        let user = under_test.register_new_user(String::from(name)).await;

        assert_eq!(user.name, name);

        eprintln!("user = {:?}", user);
        let queried_user = under_test.get_user(&user.id).await.unwrap();
        assert_eq!(queried_user.name, name);
        assert_eq!(under_test.get_users().await, vec![user]);
    }

    async fn get_test_db_adapter() -> GenericSurrealDbAdapter<Any> {
        let database = connect("memory").await.unwrap();
        database
            .use_ns("namespace")
            .use_db("database")
            .await
            .unwrap();
        GenericSurrealDbAdapter { database }
    }
}
