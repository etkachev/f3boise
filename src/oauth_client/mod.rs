use crate::web_api_state::{LOCAL_URL, PORT_NUMBER, SLACK_SERVER};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::env;

pub fn get_oauth_client() -> BasicClient {
    let client_id =
        ClientId::new(env::var("SLACK_CLIENT_ID").expect("Missing slack client id env"));
    let client_secret = ClientSecret::new(
        env::var("SLACK_CLIENT_SECRET").expect("Missing slack client secret env"),
    );
    let auth_url = AuthUrl::new(format!("https://{}/oauth/v2/authorize", SLACK_SERVER))
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(format!("https://{}/api/oauth.v2.access", SLACK_SERVER))
        .expect("Invalid token url");
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(
            RedirectUrl::new(format!("http://{}:{}/auth", LOCAL_URL, PORT_NUMBER))
                .expect("Invalid redirect URL"),
        );
    client
}
