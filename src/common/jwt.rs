use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub res: i64,
    pub rl: String,
    pub exp: usize,
}

pub fn encode_jwt(my_claims: Claims, secret: &String) -> Option<String> {
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    return match token {
        Ok(token) => Some(token),
        Err(_) => None,
    };
}

pub fn decode_jwt(token: String, secret: &String) -> Option<Claims> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    return match token {
        Ok(token) => Some(token.claims),
        Err(_) => None,
    };
}
