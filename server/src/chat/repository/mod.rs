mod in_memory;

pub use crate::chat::repository::in_memory::InMemoryRepository;
use crate::chat::User;

pub trait ChatRepository: Send + Sync + Default {
    fn new() -> Self;

    fn get_users(&self) -> Vec<User>;

    fn get_user(&self, id: &str) -> Option<User>;

    fn register_new_user(&self, name: String) -> User;
}
