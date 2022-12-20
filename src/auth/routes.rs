use std::collections::BTreeMap;

use actix_web::{post, web, HttpResponse};

use validator::Validate;

use crate::{
    api_error::ApiError,
    auth::{
        models::{LoginForm, LoginResponse, RegisterForm},
        service::sign,
    },
    user::User,
};

#[post("/login")]
async fn login(form: web::Json<LoginForm>) -> Result<HttpResponse, ApiError> {
    match form.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(e.to_string()));
        }
    }

    let user = User::find_by_email(form.email.as_ref().unwrap())?;

    if !user
        .verify_password(form.password.as_ref().unwrap())
        .unwrap()
    {
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

#[post("/register")]
async fn register(user: web::Json<RegisterForm>) -> Result<HttpResponse, ApiError> {
    match user.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(e.to_string()));
        }
    };

    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
