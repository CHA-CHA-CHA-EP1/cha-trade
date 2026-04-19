#[derive(Debug)]
pub struct User {
    pub id:              String,
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
