use anyhow::Result;
use crate::domains::models::auth::{CreateUser, User};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn create(&self, user: CreateUser) -> Result<User>;
}
