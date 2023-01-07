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
    println!("user: {:?}", user);

    Ok(HttpResponse::Ok().json(json!({
        "message": "Todos fetched successfully",
        "data": []
    })))
    // let todos = Todo::find_all()?;

    // Ok(HttpResponse::Ok().json(json!({
    //     "message": "Todos fetched successfully",
    //     "data": todos
    // })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(todos);
}
