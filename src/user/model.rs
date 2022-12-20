use crate::api_error::ApiError;
use crate::auth::models::RegisterForm;
use crate::db;
use crate::schema::*;
use bcrypt::BcryptError;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
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

    pub fn create(user: RegisterForm) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(user::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: RegisterForm) -> Result<Self, ApiError> {
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

    pub fn find_by_email(email: &str) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = user::table.filter(user::email.eq(email)).first(&mut conn)?;
        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, BcryptError> {
        let is_valid = bcrypt::verify(password, &self.password)?;
        Ok(is_valid)
    }
}

impl From<RegisterForm> for User {
    fn from(user: RegisterForm) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email.unwrap(),
            password: bcrypt::hash(&user.password.unwrap(), 4).unwrap(),
            name: user.name.unwrap(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
