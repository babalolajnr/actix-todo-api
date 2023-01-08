use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

use crate::{
    api_error::ApiError,
    todo::model::{CreateTodoForm, Todo},
    user::User,
};

#[post("/")]
async fn create(user: User, form: web::Json<CreateTodoForm>) -> Result<HttpResponse, ApiError> {
    let todo = Todo::create(user, form.into_inner())?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Todo created successfully",
        "data": todo
    })))
}

#[get("/")]
async fn todos(user: User) -> Result<HttpResponse, ApiError> {

    let todos = Todo::todos(user)?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Todos fetched successfully",
        "data": todos
    })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(todos);
}
