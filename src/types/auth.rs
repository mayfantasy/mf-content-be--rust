use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfo {
  pub email: String,
  pub username: String,
  pub tier: i8,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoResponse {
  pub token: String,
  pub account: IBasicAccountInfo,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoWithoutTokenResponse {
  pub account: IBasicAccountInfo,
}

#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoResponseResult {
  pub result: IBasicAccountInfoResponse,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoWithoutTokenResponseResult {
  pub result: IBasicAccountInfoWithoutTokenResponse,
}
#[derive(Deserialize, Serialize)]
pub struct ILoginPayload {
  pub email: String,
  pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct ITokenPayload {
  pub token: String,
}
#[derive(Deserialize, Serialize)]
pub struct IEmailPayload {
  pub email: String,
}

#[derive(Debug)]
pub enum ApiKeyError {
  Missing,
  Invalid,
}

pub struct IAuthObject {
  pub db_key: String,
  pub api_key: String,
  pub account_id: String,
}

pub struct ApiKey<'r>(&'r str);
pub struct Token<'r>(pub &'r str);
