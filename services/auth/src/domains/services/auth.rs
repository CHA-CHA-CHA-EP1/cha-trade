use anyhow::Result;
use crate::handler::register_handler::RegisterRequest;

#[async_trait::async_trait]
pub trait AuthService: Sync + Send {
    async fn register(&self, req: RegisterRequest) -> Result<String>;
}
