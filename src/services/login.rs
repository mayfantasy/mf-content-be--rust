use crate::routes::loginRoutes::IBasicAccountInfo;
use crate::routes::loginRoutes::{ILoginPayload, ITokenPayload};
use tokio::time::{sleep, Duration};

pub async fn getAccountByEmailAndPassword(user: &ILoginPayload) -> IBasicAccountInfo {
    // Print input
    println!("The user's email is {}.", user.email);
    println!("The user's password is {}.", user.password);

    // Sleep 3s
    sleep(Duration::from_millis(3000)).await;

    // Return account info
    IBasicAccountInfo {
        email: String::from("hans@mayfantasy.com"),
        username: String::from("hansknowyou"),
        tier: 1,
    }
}

pub async fn getAccountByEmail(payload: &ITokenPayload) -> IBasicAccountInfo {
    // Print input
    println!("The user's token is {}.", payload.token);

    // Sleep 3s
    sleep(Duration::from_millis(3000)).await;

    // Return account info
    IBasicAccountInfo {
        email: String::from("hans@mayfantasy.com"),
        username: String::from("hansknowyou"),
        tier: 1,
    }
}
