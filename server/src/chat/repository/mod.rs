pub use crate::chat::repository::in_memory::InMemoryRepository;
use crate::chat::User;
use async_trait::async_trait;

pub mod in_memory;
pub mod surreal_db;

#[async_trait]
pub trait ChatRepository: Send + Sync + Default {
    async fn get_users(&self) -> Vec<User>;

    async fn get_user(&self, id: &str) -> Option<User>;

    async fn register_new_user(&self, name: String) -> User;
}
