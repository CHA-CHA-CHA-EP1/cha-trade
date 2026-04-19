use anyhow::Result;
use crate::handler::{login_handler::LoginRequest, register_handler::RegisterRequest};

#[async_trait::async_trait]
pub trait AuthService: Sync + Send {
    async fn register(&self, req: RegisterRequest) -> Result<String>;
    async fn login(&self, req: LoginRequest) -> Result<String>;
}
