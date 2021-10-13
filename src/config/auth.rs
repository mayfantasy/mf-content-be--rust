use crate::config::jwt_config::{verify, Claims};
use crate::types::auth_types::ApiKeyError;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
  type Error = ApiKeyError;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    match req.headers().get_one("Authorization") {
      None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
      Some(token) => Outcome::Success(verify(&String::from(token)).unwrap().claims),
    }
  }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for ApiKey<'r> {
//   type Error = ApiKeyError;

//   async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//     /// Returns true if `key` is a valid API key string.
//     fn is_valid(key: &str) -> bool {
//       key == "valid_api_key"
//     }

//     match req.headers().get_one("x-api-key") {
//       None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
//       Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
//       Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
//     }
//   }
// }
