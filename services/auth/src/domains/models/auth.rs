use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id:              Uuid,
    pub first_name:      String,
    pub first_name_hmac: String,
    pub last_name:       String,
    pub last_name_hmac:  String,
    pub email:           String,
    pub password_hash:   String,
}

#[derive(Debug)]
pub struct CreateUser {
    pub first_name:      String,
    pub first_name_hmac: String,
    pub last_name:       String,
    pub last_name_hmac:  String,
    pub email:           String,
    pub password_hash:   String,
}
