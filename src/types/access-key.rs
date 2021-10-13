use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct IAccessKey {
  pub id: String,
  pub key: String,
  pub name: String,
  pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ICreateAccessKeyPayload {
  pub name: String,
  pub description: String,
}
