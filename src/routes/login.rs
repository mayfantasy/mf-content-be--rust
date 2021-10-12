use chrono::{DateTime, TimeZone, Utc};
use hmac::{Hmac, NewMac};

use crate::config::jwtConfig;
use crate::services::loginServices::{getAccountByEmail, getAccountByEmailAndPassword};
// use crate::config::routeConfig;
// use crate::config::tierConfig;
use jwtConfig::{sign, verify, Claims};
use rocket::serde::{json, Deserialize, Serialize};
// use routeConfig::ERoutes;
// use tierConfig::getRouteTierMap;
// use tierConfig::ITier;

// use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfo {
    pub email: String,
    pub username: String,
    pub tier: i8,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoResponse {
    token: String,
    account: IBasicAccountInfo,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoWithoutTokenResponse {
    account: IBasicAccountInfo,
}

#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoResponseResult {
    result: IBasicAccountInfoResponse,
}
#[derive(Deserialize, Serialize)]
pub struct IBasicAccountInfoWithoutTokenResponseResult {
    result: IBasicAccountInfoWithoutTokenResponse,
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

#[post("/api/auth/login", format = "application/json", data = "<input>")]
pub async fn login_route(
    input: json::Json<ILoginPayload>,
) -> json::Json<IBasicAccountInfoResponseResult> {
    // Generate JWT token
    let token = sign(&Claims {
        user: ILoginPayload {
            email: String::from(input.email.to_string()),
            password: String::from(input.password.to_string()),
        },
        exp: 32503680000,
    })
    .unwrap();

    // Get account info from DB
    let account = getAccountByEmailAndPassword(&input).await;

    // Response object
    json::Json(IBasicAccountInfoResponseResult {
        result: IBasicAccountInfoResponse { token, account },
    })
}
#[post("/api/auth/token", format = "application/json", data = "<input>")]
pub async fn login_with_token_route(
    input: json::Json<ITokenPayload>,
) -> Result<json::Json<IBasicAccountInfoWithoutTokenResponseResult>, String> {
    // Generate JWT token
    let user = verify(&input.token);

    return match user {
        Ok(user) => {
            // Get account info from DB
            let account = getAccountByEmail(&input).await;

            // Response object
            return Ok(json::Json(IBasicAccountInfoWithoutTokenResponseResult {
                result: IBasicAccountInfoWithoutTokenResponse { account },
            }));
        }
        Err(e) => {
            println!("Your token is: {}", e);
            Err("Can not verify token {}".into())
        }
    };
}
