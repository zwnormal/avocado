use crate::session::{Session, SessionId, UserId};
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

pub(crate) mod sqlite;

#[async_trait]
pub(crate) trait SessionStore: Send + Sync + Debug {
    async fn insert(&self, session: Session) -> Result<SessionId>;
    async fn update(&self, session: Session) -> Result<()>;
    async fn get(&self, session_id: &SessionId) -> Result<Option<Session>>;
    async fn delete(&self, session_id: &SessionId) -> Result<()>;
    async fn logout(&self, user_id: &UserId) -> Result<()>;
}
