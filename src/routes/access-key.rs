use rocket::serde::{json, Deserialize, Serialize};

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

#[post("/api/access-key/create", data = "<payload>")]
pub async fn create_asccess_key(
  payload: json::Json<ICreateAccessKeyPayload>,
) -> json::Json<IAccessKey> {
  // 1. Get auth: a) token/accessKey from header b) extract email from token
  // c) get account by email d) Check if the tier of the account has access to this route
  // e) return IAuthObject
  json::Json(IAccessKey {
    id: String::from("IDIDIDIDIDIDID"),
    key: String::from("ienjfllwkefj2l3rf2ofsldkfnasdlf.welkf"),
    name: String::from(payload.name.to_string()),
    description: String::from(payload.description.to_string()),
  })
}
// #[get("/api/access-key/list")]
// #[delete("/api/access-key/<id>")]
