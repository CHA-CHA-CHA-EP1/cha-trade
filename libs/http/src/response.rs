use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code:    String,
    pub message: String,
    pub data:    Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code:    "0000".to_string(),
            message: "success".to_string(),
            data:    Some(data),
        }
    }

    pub fn error(code: &str, message: &str) -> ApiResponse<T> {
        ApiResponse {
            code:    code.to_string(),
            message: message.to_string(),
            data:    None,
        }
    }
}
