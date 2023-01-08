use crate::user::User;
use crate::{api_error::ApiError, db, schema::*};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use futures::future::LocalBoxFuture;
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, Debug)]
#[table_name = "todo"]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Todo {
    pub fn create(user: User, todo: CreateTodoForm) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let todo = Todo::from(todo).set_user(user);

        let todo = diesel::insert_into(todo::table)
            .values(todo)
            .get_result(&mut conn)?;

        Ok(todo)
    }

    pub fn todos(user: User) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;

        let todos = todo::table
            .filter(todo::user_id.eq(user.id))
            .load::<Todo>(&mut conn)?;

        Ok(todos)
    }

    pub fn update(user: User, todo: Todo, form: UpdateTodoForm) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let todo = Todo {
            id: todo.id,
            title: form.title.as_ref().unwrap_or(&todo.title).to_string(),
            description: form
                .description
                .as_ref()
                .unwrap_or(&todo.description)
                .to_string(),
            done: todo.done,
            user_id: todo.user_id,
            created_at: todo.created_at,
            updated_at: Some(Utc::now().naive_utc()),
        };

        let todo = diesel::update(todo::table)
            .filter(todo::id.eq(todo.id))
            .filter(todo::user_id.eq(user.id))
            .set(todo)
            .get_result(&mut conn)?;

        Ok(todo)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let todo = todo::table.filter(todo::id.eq(id)).first(&mut conn)?;

        Ok(todo)
    }

    pub fn delete(user: User, todo: Todo) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let deleted = diesel::delete(todo::table)
            .filter(todo::id.eq(todo.id))
            .filter(todo::user_id.eq(user.id))
            .execute(&mut conn)?;

        Ok(deleted)
    }

    pub fn toggle_completion(user: User, todo: Todo) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let todo = Todo {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            done: !todo.done,
            user_id: todo.user_id,
            created_at: todo.created_at,
            updated_at: Some(Utc::now().naive_utc()),
        };

        let todo = diesel::update(todo::table)
            .filter(todo::id.eq(todo.id))
            .filter(todo::user_id.eq(user.id))
            .set(todo)
            .get_result(&mut conn)?;

        Ok(todo)
    }
}

impl Todo {
    pub fn set_user(self, user: User) -> Self {
        Todo {
            user_id: user.id,
            ..self
        }
    }
}

impl FromRequest for Todo {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let todo_id = req.match_info().get("id").unwrap();

        let todo_id = match Uuid::parse_str(todo_id) {
            Ok(id) => id,
            Err(e) => {
                error!("Error: {}", e);
                return Box::pin(async {
                    Err(ApiError::not_found("Invalid id provided".to_string()))
                });
            }
        };

        let todo = match Todo::find(todo_id) {
            Ok(todo) => todo,
            Err(e) => {
                error!("Error: {}", e);
                return Box::pin(async { Err(ApiError::not_found("Todo not found".to_string())) });
            }
        };

        Box::pin(async { Ok(todo) })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTodoForm {
    #[validate(required(message = "Title is required"))]
    pub title: Option<String>,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTodoForm {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl From<CreateTodoForm> for Todo {
    fn from(todo: CreateTodoForm) -> Self {
        Todo {
            id: Uuid::new_v4(),
            title: todo.title.unwrap(),
            description: todo.description.unwrap_or("".to_string()),
            done: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            user_id: Uuid::new_v4(),
        }
    }
}
