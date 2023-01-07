use crate::api_error::ApiError;
use crate::auth::models::RegisterForm;
use crate::auth::verify;
use crate::db;
use crate::schema::*;
use actix_web::FromRequest;
use bcrypt::BcryptError;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
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

impl FromRequest for User {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authorization = req.headers().get("Authorization");

        let token = match authorization {
            Some(token) => token.to_str().unwrap(),
            None => {
                return Box::pin(async {
                    Err(ApiError::unauthorized(
                        "This request is unauthorized".to_string(),
                    ))
                })
            }
        };

        let claims = verify(token).unwrap();
        let user = User::find(
            Uuid::parse_str(&claims.get("id").unwrap())
                .map_err(|_e| ApiError::unauthorized("This request is unauthorized".to_string()))
                .unwrap(),
        )
        .unwrap();

        Box::pin(async { Ok(user) })
    }
}
