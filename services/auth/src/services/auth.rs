use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::domains::models::auth::CreateUser;
use crate::domains::repositories::auth::UserRepository;
use crate::domains::services::auth::AuthService;
use crate::handler::register_handler::RegisterRequest;
use crypto::{FieldEncryptor, FieldHmac, PasswordHasher};

pub struct AuthServiceImpl<R: UserRepository> {
    user_repo:       Arc<R>,
    encryptor:       Arc<dyn FieldEncryptor>,
    password_hasher: Arc<dyn PasswordHasher>,
    hmac:            Arc<dyn FieldHmac>,
}

impl<R: UserRepository> AuthServiceImpl<R> {
    pub fn new(
        user_repo:       Arc<R>,
        encryptor:       Arc<dyn FieldEncryptor>,
        password_hasher: Arc<dyn PasswordHasher>,
        hmac:            Arc<dyn FieldHmac>,
    ) -> Self {
        Self { user_repo, encryptor, password_hasher, hmac }
    }
}

#[async_trait]
impl<R: UserRepository> AuthService for AuthServiceImpl<R> {
    async fn register(&self, req: RegisterRequest) -> Result<String> {
        // 1. check email duplicate
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(anyhow!("email already registered"));
        }

        // 2. encrypt first_name, last_name
        let first_name      = self.encryptor.encrypt(&req.first_name)?;
        let first_name_hmac = self.hmac.generate(&req.first_name)?;
        let last_name       = self.encryptor.encrypt(&req.last_name)?;
        let last_name_hmac  = self.hmac.generate(&req.last_name)?;

        // 3. hash password
        let password_hash = self.password_hasher.hash(&req.password)?;

        // 4. insert user
        let user = self.user_repo.create(CreateUser {
            first_name,
            first_name_hmac,
            last_name,
            last_name_hmac,
            email: req.email,
            password_hash,
        }).await?;

        Ok(user.id.to_string())
    }
}
