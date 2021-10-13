use crate::config::jwt_config::Claims;
use crate::config::route_config::ERoutes;
use crate::config::tier_config::get_route_tier_map;
use crate::services::auth_services::get_account_by_email;
use crate::types::access_key_types::{IAccessKey, ICreateAccessKeyPayload};
use crate::types::auth_types::IEmailPayload;
use rocket::http::Status;
use rocket::serde::json;

#[post("/access-key/create", data = "<payload>")]
pub async fn create_asccess_key(
  payload: json::Json<ICreateAccessKeyPayload>,
  claims: Claims, // This request guard returns the claims
) -> Result<json::Json<IAccessKey>, Status> {
  let email = claims.user.email;
  let account_tier = get_account_by_email(&IEmailPayload { email }).await.tier;

  let route_tier = get_route_tier_map()
    .get(&ERoutes::CREATE_ACCESS_KEY)
    .unwrap()
    .tier;

  println!(
    "Account tier is {}, route tier is {}",
    account_tier, route_tier
  );

  // Check if user has authorized tier
  if account_tier < route_tier {
    return Ok(json::Json(IAccessKey {
      id: String::from("IDIDIDIDIDIDID"),
      key: String::from("ienjfllwkefj2l3rf2ofsldkfnasdlf.welkf"),
      name: String::from(payload.name.to_string()),
      description: String::from(payload.description.to_string()),
    }));
  } else {
    println!("Unauthorized");
    return Err(Status::Forbidden);
  }
}
// #[get("/access-key/list")]
// #[delete("/access-key/<id>")]
