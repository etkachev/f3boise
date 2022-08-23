use oauth2::basic::BasicClient;

pub const LOCAL_URL: &str = "127.0.0.1";
pub const SLACK_SERVER: &str = "slack.com";
pub const PORT_NUMBER: u16 = 8080;

pub struct AppState {
    pub oauth: BasicClient,
    pub api_base_url: String,
    pub signing_secret: String,
    pub bot_auth_token: String,
    /// Deprecated verify token
    pub verify_token: String,
}
