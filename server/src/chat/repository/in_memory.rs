use crate::chat::repository::ChatRepository;
use crate::chat::User;
use std::sync::Mutex;

#[derive(Default)]
pub struct InMemoryRepository {
    users: Mutex<Vec<User>>,
}

impl ChatRepository for InMemoryRepository {
    fn new() -> Self {
        Self::default()
    }

    fn get_users(&self) -> Vec<User> {
        self.users.lock().unwrap().clone()
    }

    fn get_user(&self, id: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.iter().find(|user| user.id == id).cloned()
    }

    fn register_new_user(&self, name: String) -> User {
        let mut users = self.users.lock().unwrap();
        let new_user = User {
            id: format!("User#{id}", id = users.len()),
            name,
        };
        users.push(new_user.clone());
        new_user
    }
}
