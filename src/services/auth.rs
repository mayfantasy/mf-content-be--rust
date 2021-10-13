use crate::types::auth_types::{IBasicAccountInfo, IEmailPayload, ILoginPayload};
use tokio::time::{sleep, Duration};

pub async fn get_account_by_email_and_password(user: &ILoginPayload) -> IBasicAccountInfo {
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

pub async fn get_account_by_email(payload: &IEmailPayload) -> IBasicAccountInfo {
    // Print input
    println!("The user's email is {}.", payload.email);

    // Sleep 3s
    sleep(Duration::from_millis(3000)).await;

    // Return account info
    IBasicAccountInfo {
        email: String::from(payload.email.to_string()),
        username: String::from("hansknowyou"),
        tier: 3,
    }
}
