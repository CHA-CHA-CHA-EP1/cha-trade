use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domains::models::auth::{CreateUser, User};
use crate::domains::repositories::auth::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, first_name, first_name_hmac, last_name, last_name_hmac, email, password_hash
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, user: CreateUser) -> Result<User> {
        let created = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (first_name, first_name_hmac, last_name, last_name_hmac, email, password_hash)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, first_name, first_name_hmac, last_name, last_name_hmac, email, password_hash
            "#,
        )
        .bind(user.first_name)
        .bind(user.first_name_hmac)
        .bind(user.last_name)
        .bind(user.last_name_hmac)
        .bind(user.email)
        .bind(user.password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }
}
