//TODO: Uses gclassroom apis to fetch deadlines

use std::env;

use oauth2::{basic::BasicClient, AuthUrl, Client, ClientId, ClientSecret, TokenUrl};

use super::Info;

pub fn authenticate() {
    dotenv::dotenv().ok();
    let client_id = ClientId::new(env::var("CLIENT_ID").expect("Not found in env"));
    let client_secret = ClientSecret::new(env::var("CLIENT_ID").expect("Not found in env"));
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).expect("Invalid url");
    let token_url =
        TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).expect("Invalid url");

    let client: BasicClient =
        Client::new(client_id, Some(client_secret), auth_url, Some(token_url));

    println!("{:?}", client);
}

pub fn fetch_deadlines() -> Option<Info> {
    authenticate();
    return None;
}
