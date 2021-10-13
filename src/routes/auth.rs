use crate::types::auth_types::{
    IBasicAccountInfoResponse, IBasicAccountInfoResponseResult,
    IBasicAccountInfoWithoutTokenResponse, IBasicAccountInfoWithoutTokenResponseResult,
    IEmailPayload, ILoginPayload, ITokenPayload,
};

use crate::config::jwt_config;
use crate::services::auth_services::{get_account_by_email, get_account_by_email_and_password};
// use crate::config::route_config;
// use crate::config::tier_config;
use jwt_config::{sign, verify, Claims};
use rocket::serde::json;
// use route_config::ERoutes;
// use tier_config::getRouteTierMap;
// use tier_config::ITier;

// use std::collections::HashMap;

#[post("/auth/login", format = "application/json", data = "<input>")]
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
    let account = get_account_by_email_and_password(&input).await;

    // Response object
    json::Json(IBasicAccountInfoResponseResult {
        result: IBasicAccountInfoResponse { token, account },
    })
}
#[post("/auth/token", format = "application/json", data = "<input>")]
pub async fn login_with_token_route(
    input: json::Json<ITokenPayload>,
) -> Result<json::Json<IBasicAccountInfoWithoutTokenResponseResult>, String> {
    // Generate JWT token
    let user = verify(&input.token);

    return match user {
        Ok(user) => {
            // Get account info from DB
            let account = get_account_by_email(&IEmailPayload {
                email: user.claims.user.email,
            })
            .await;

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
