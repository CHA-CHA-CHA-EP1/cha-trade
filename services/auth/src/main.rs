use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

use auth::domains::services::auth::AuthService;
use auth::handler;
use auth::infrastructure::repositories::user::PostgresUserRepository;
use auth::services::auth::AuthServiceImpl;
use crypto::{AesGcmEncryptor, Argon2Hasher, HmacSha256};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let aes_key_hex  = std::env::var("AES_KEY").expect("AES_KEY must be set");
    let hmac_key_hex = std::env::var("HMAC_KEY").expect("HMAC_KEY must be set");
    let jwt_secret   = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let aes_key  = hex::decode(&aes_key_hex).expect("AES_KEY must be valid hex");
    let hmac_key = hex::decode(&hmac_key_hex).expect("HMAC_KEY must be valid hex");

    let encryptor       = Arc::new(AesGcmEncryptor::new(&aes_key).expect("invalid AES key"));
    let password_hasher = Arc::new(Argon2Hasher::new());
    let hmac            = Arc::new(HmacSha256::new(&hmac_key).expect("invalid HMAC key"));
    let user_repo       = Arc::new(PostgresUserRepository::new(pool));

    let auth_service: Arc<dyn AuthService> = Arc::new(AuthServiceImpl::new(
        user_repo,
        encryptor,
        password_hasher,
        hmac,
        jwt_secret,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(auth_service.clone()))
            .route("/health-check", web::get().to(handler::health_check_handler::health_check))
            .route("/auth/register", web::post().to(handler::register_handler::register_handler))
            .route("/auth/login", web::post().to(handler::login_handler::login_handler))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
