use crate::{
    api_error::ApiError,
    todo::model::{CreateTodoForm, Todo, UpdateTodoForm},
    user::User,
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde_json::json;

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

#[delete("/{id}")]
async fn delete(todo: Todo, user: User) -> Result<HttpResponse, ApiError> {
    if todo.user_id != user.id {
        return Err(ApiError::unauthorized("Unauthorized".to_string()));
    }

    Todo::delete(user, todo)?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Todo deleted successfully",
        "data": []
    })))
}

#[patch("/done/{id}")]
async fn done(todo: Todo, user: User) -> Result<HttpResponse, ApiError> {
    if todo.user_id != user.id {
        return Err(ApiError::unauthorized("Unauthorized".to_string()));
    }

    let todo = Todo::toggle_completion(user, todo)?;

    let message = if todo.done {
        "Todo marked as done"
    } else {
        "Todo marked as not done"
    };

    Ok(HttpResponse::Ok().json(json!({
        "message": message,
        "data": todo
    })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(todos);
    cfg.service(update);
    cfg.service(delete);
}
