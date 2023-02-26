use crate::chat::repository::ChatRepository;
use crate::chat::User;
use async_trait::async_trait;
use std::sync::Mutex;

#[derive(Default)]
pub struct InMemoryRepository {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl ChatRepository for InMemoryRepository {
    async fn get_users(&self) -> Vec<User> {
        self.users.lock().unwrap().clone()
    }

    async fn get_user(&self, id: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.iter().find(|user| user.id == id).cloned()
    }

    async fn register_new_user(&self, name: String) -> User {
        let mut users = self.users.lock().unwrap();
        let new_user = User {
            id: format!("User#{id}", id = users.len()),
            name,
        };
        users.push(new_user.clone());
        new_user
    }
}
