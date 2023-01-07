use actix_web::{error::PayloadError, FromRequest, HttpMessage};
use jwt::VerifyWithKey;
use std::{collections::BTreeMap, env, error::Error, pin::Pin};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

pub fn sign(claims: BTreeMap<&str, &str>) -> Result<String, Box<dyn Error>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let token_str = claims.sign_with_key(&key)?;

    Ok(token_str)
}

pub fn verify(token_str: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    
    let token_str = token_str.replace("Bearer ", "");

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
    Ok(claims)
}

#[derive(Debug, Clone)]
pub struct Claims {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl FromRequest for Claims {
    type Error = PayloadError;
    type Future = futures::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Pin<Box<(dyn futures::Future<Output = Result<Claims, PayloadError>> + 'static)>> {
        let claims: Claims = req.extensions().get::<Claims>().unwrap().clone();
        Box::pin(async { Ok(claims) })
    }
}
