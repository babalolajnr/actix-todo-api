use actix_web::{get, patch, post, web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    api_error::ApiError,
    todo::model::{CreateTodoForm, Todo, UpdateTodoForm},
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

#[patch("/{id}")]
async fn update(
    user: User,
    todo: Todo,
    form: web::Json<UpdateTodoForm>,
) -> Result<HttpResponse, ApiError> {
    if todo.user_id != user.id {
        return Err(ApiError::unauthorized("Unauthorized".to_string()));
    }

    let todo = Todo::update(user, todo, form.into_inner())?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Todo updated successfully",
        "data": todo
    })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(todos);
    cfg.service(update);
}
