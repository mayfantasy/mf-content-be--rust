use crate::types::auth_types::ILoginPayload;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user: ILoginPayload,
    pub exp: u64,
}

pub fn sign(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );
    return token;
}

pub fn verify(
    token: &String,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );
    return data;
}
