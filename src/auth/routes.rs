use std::collections::BTreeMap;

use actix_web::{get, web, HttpResponse, post};

use crate::{
    api_error::ApiError,
    auth::{
        models::{LoginForm, LoginResponse},
        service::sign,
    },
    user::User,
};

#[post("auth/login")]
async fn login(form: web::Json<LoginForm>) -> Result<HttpResponse, ApiError> {
    // TODO: Validate form
    let user = User::find_by_email(&form.email)?;

    if !user.verify_password(&form.password).unwrap() {
        return Err(ApiError::unauthorized("Invalid credentials".to_string()));
    }
    let user_id = user.id.to_string();
    let name = user.name.to_string();
    let email = user.email.to_string();

    let mut claims = BTreeMap::new();
    claims.insert("id", user_id.as_str());
    claims.insert("name", name.as_str());
    claims.insert("email", email.as_str());

    let token = sign(claims).map_err(|_| ApiError::internal_server_error());

    Ok(HttpResponse::Ok().json(LoginResponse {
        token: token.unwrap(),
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
