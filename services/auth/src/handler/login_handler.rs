use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use validator::Validate;
use http::ApiResponse;

use crate::domains::services::auth::AuthService;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "password must not be empty"))]
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
}

pub async fn login_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("4000", &e.to_string()));
    }

    match service.login(body.into_inner()).await {
        Ok(access_token) => HttpResponse::Ok().json(ApiResponse::success(LoginResponse { access_token })),
        Err(e) if e.to_string().contains("invalid email or password") => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("4001", &e.to_string()))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("5000", &e.to_string())),
    }
}
