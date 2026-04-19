use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn register_handler() -> impl Responder {
    HttpResponse::Ok().body("i'm alive")
}
