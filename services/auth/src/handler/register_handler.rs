use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use validator::Validate;
use http::ApiResponse;

use crate::domains::services::auth::AuthService;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "first_name must not be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last_name must not be empty"))]
    pub last_name: String,
    #[validate(email(message = "email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    user_id: String,
}

pub async fn register_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("4000", &e.to_string()));
    }

    match service.register(body.into_inner()).await {
        Ok(user_id) => HttpResponse::Created().json(ApiResponse::success(RegisterResponse { user_id })),
        Err(e) if e.to_string().contains("email already registered") => {
            HttpResponse::Conflict().json(ApiResponse::<()>::error("4009", &e.to_string()))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("5000", &e.to_string())),
    }
}
