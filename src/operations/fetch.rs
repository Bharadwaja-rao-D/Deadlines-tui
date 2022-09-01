//TODO: Uses gclassroom apis to fetch deadlines

use std::fs;

use oauth2::basic::BasicClient;
use serde::{Deserialize, Serialize};

use crate::operations::Info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

pub fn fetch_deadlines() -> Option<Info> {
    let info = fs::read_to_string("credentials.json").unwrap();
    let info: Credentials = serde_json::from_str(&info).unwrap();

    let url = &info.redirect_uris[0];

    let client = BasicClient::new(
        oauth2::ClientId::new(info.client_id),
        Some(oauth2::ClientSecret::new(info.client_secret)),
        oauth2::AuthUrl::new(info.auth_uri).unwrap(),
        Some(oauth2::TokenUrl::new(info.token_uri).unwrap()),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(url.to_string()).unwrap());

    return None;
}
