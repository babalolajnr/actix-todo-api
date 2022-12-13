use crate::api_error::ApiError;
use crate::db;
use crate::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserMessage {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;

        let users = user::table.load::<User>(&mut conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = user::table.filter(user::id.eq(id)).first(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(user::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(user::table.filter(user::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
            name: user.name,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
