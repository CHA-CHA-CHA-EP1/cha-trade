use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

use crate::domains::models::auth::CreateUser;
use crate::domains::repositories::auth::UserRepository;
use crate::domains::services::auth::AuthService;
use crate::handler::{login_handler::LoginRequest, register_handler::RegisterRequest};
use crypto::{FieldEncryptor, FieldHmac, PasswordHasher};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

pub struct AuthServiceImpl<R: UserRepository> {
    user_repo:       Arc<R>,
    encryptor:       Arc<dyn FieldEncryptor>,
    password_hasher: Arc<dyn PasswordHasher>,
    hmac:            Arc<dyn FieldHmac>,
    jwt_secret:      String,
}

impl<R: UserRepository> AuthServiceImpl<R> {
    pub fn new(
        user_repo:       Arc<R>,
        encryptor:       Arc<dyn FieldEncryptor>,
        password_hasher: Arc<dyn PasswordHasher>,
        hmac:            Arc<dyn FieldHmac>,
        jwt_secret:      String,
    ) -> Self {
        Self { user_repo, encryptor, password_hasher, hmac, jwt_secret }
    }

    fn generate_token(&self, user_id: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
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

    async fn login(&self, req: LoginRequest) -> Result<String> {
        // 1. find user by email
        let user = self.user_repo.find_by_email(&req.email).await?
            .ok_or_else(|| anyhow!("invalid email or password"))?;

        // 2. verify password
        if !self.password_hasher.verify(&req.password, &user.password_hash)? {
            return Err(anyhow!("invalid email or password"));
        }

        // 3. generate JWT token
        let token = self.generate_token(&user.id.to_string())?;

        Ok(token)
    }
}
