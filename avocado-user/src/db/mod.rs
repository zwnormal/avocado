use crate::domain::user::{User, UserId};
use anyhow::Result;
use std::fmt::Debug;

#[tonic::async_trait]
pub(crate) trait UserStore: Send + Sync + Debug {
    async fn insert(&self, user: User) -> Result<UserId>;
    async fn get(&self, user_id: &UserId) -> Result<Option<User>>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn list(&self) -> Result<Vec<User>>;
    async fn delete(&self, user_id: &UserId) -> Result<()>;
    async fn update(&self, user_id: &UserId, user: User) -> Result<()>;
}

pub mod sqlite;
