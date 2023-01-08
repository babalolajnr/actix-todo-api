use crate::user::User;
use crate::{api_error::ApiError, db, schema::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
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
}

impl Todo {
    pub fn set_user(self, user: User) -> Self {
        Todo {
            user_id: user.id,
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTodoForm {
    #[validate(required(message = "Title is required"))]
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
