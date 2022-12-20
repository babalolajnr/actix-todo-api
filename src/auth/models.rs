use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginData {
    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid email format"),
        length(
            min = 3,
            message = "Email is too short. It must be at least 3 characters long"
        )
    )]
    pub email: Option<String>,

    #[validate(
        required(message = "Password is required"),
        length(
            min = 8,
            message = "Password is too short. It must be at least 8 characters long"
        )
    )]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
