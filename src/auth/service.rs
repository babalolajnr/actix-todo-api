use jwt::VerifyWithKey;
use std::{collections::BTreeMap, env, error::Error};

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

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
    Ok(claims)
}
