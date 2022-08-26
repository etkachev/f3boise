use crate::app_state::backblast_data::{BackBlastData, BACK_BLAST_TAG};
use crate::app_state::{parse_backblast, AppState};
use crate::bot_data::{BotUser, UserBotCombo};
use crate::db::DbStore;
use crate::slack_api::channels::history::request::ChannelHistoryRequest;
use crate::slack_api::channels::history::response::{ChannelsHistoryResponse, MessageData};
use crate::slack_api::channels::list::request::ConversationListRequest;
use crate::slack_api::channels::list::response::{ChannelData, ChannelsListResponse};
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::channels::reactions_add::request::ReactionsAddRequest;
use crate::slack_api::channels::reactions_add::response::ReactionsAddResponse;
use crate::slack_api::channels::types::ChannelTypes;
use crate::slack_api::url_requests::SlackUrlRequest;
use crate::slack_api::users::users_list::request::UsersListRequest;
use crate::slack_api::users::users_list::response::UsersListResponse;
use crate::users::f3_user::F3User;
use http::{HeaderMap, Method};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use std::collections::HashMap;
use std::sync::Mutex;

pub const LOCAL_URL: &str = "127.0.0.1";
pub const SLACK_SERVER: &str = "slack.com";
pub const PORT_NUMBER: u16 = 8080;
const AUTH_HEADER: &str = "Authorization";

pub struct MutableWebState {
    pub token: String,
    pub base_api_url: String,
    pub oauth: BasicClient,
    pub signing_secret: String,
    pub bot_auth_token: String,
    /// Deprecated verify token
    pub verify_token: String,
    pub app: Mutex<WebAppState>,
    pub db: DbStore,
}

impl MutableWebState {
    pub async fn initialize_data(&mut self) {
        if let Err(err) = self.db.init_db() {
            println!("Error initializing db: {:?}", err);
        }
        // load users from local db
        if let Ok(users) = self.db.get_stored_users() {
            {
                let mut app = self.app.lock().unwrap();
                app.data_state.users.extend(users);
            }
            println!("Loaded db users");
        }
        self.get_public_channels().await;
        self.get_users().await;
        {
            let app = self.app.lock().unwrap();
            // sync latest slack users with local db
            if let Err(err) = self.db.sync_users_local(&app.data_state.users) {
                println!("Error syncing users to local: {:?}", err);
            }
        }
    }

    async fn get_public_channels(&self) {
        let request = ConversationListRequest::with_types(vec![ChannelTypes::Public]);
        let url = request.get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;

        let response: ChannelsListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");

        if let Some(channels) = response.channels {
            let public_channels: HashMap<PublicChannels, ChannelData> =
                channels
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, channel| {
                        let channel_name = PublicChannels::from_name(channel.name.to_string());
                        acc.insert(channel_name, channel);
                        acc
                    });
            // separate scope for minimizing lock
            {
                let mut app = self.app.lock().unwrap();
                app.data_state.channels = public_channels;
            }
        } else {
            eprintln!(
                "Error from response: {:?}",
                response.error.unwrap_or_else(|| "No error".to_string())
            );
        }
    }

    /// get users that exist in slack then sync it to local db
    async fn get_users(&mut self) {
        let url = UsersListRequest::default().get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;
        let response: UsersListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");
        if let Some(users) = response.members {
            let users_bots: UserBotCombo = users
                .into_iter()
                .filter(|user| !user.deleted && user.name.as_str() != "slackbot")
                .fold(UserBotCombo::new(), |mut acc, user| {
                    if user.is_bot {
                        acc.bots.insert(user.id.to_string(), BotUser::from(&user));
                    } else {
                        acc.users.insert(user.id.to_string(), F3User::from(&user));
                    }
                    acc
                });
            // scoped to limit lock
            {
                let mut app = self.app.lock().unwrap();
                app.data_state.users.extend(users_bots.users);
                app.data_state.bots = users_bots.bots;
                app.data_state.set_self_bot_id();
            }
        } else {
            eprintln!("{:?}", response.error);
        }
    }

    pub async fn get_back_blasts(&self) {
        let history_request = {
            let app = self.app.lock().unwrap();
            app.data_state
                .get_channel_data(PublicChannels::BotPlayground)
                .map(|channel_data| ChannelHistoryRequest::new(&channel_data.id))
        };
        if let Some(request) = history_request {
            let url = request.get_url_request(&self.base_api_url);
            println!("Calling: {:?}", url.as_str());
            let response = self.make_get_url_request(url).await;
            let response: ChannelsHistoryResponse =
                serde_json::from_slice(&response.body).expect("Could not parse response");
            if let Some(messages) = response.messages {
                let backblasts = messages
                    .iter()
                    .filter(|message| {
                        let (first_line, _) = message.text.split_once('\n').unwrap_or(("", ""));
                        first_line.to_lowercase().starts_with(BACK_BLAST_TAG)
                    })
                    .collect::<Vec<&MessageData>>();

                for entry in backblasts {
                    println!("Entry: {}", entry.ts);
                    // scoped to limit lock
                    {
                        let app = self.app.lock().unwrap();
                        let data = parse_backblast::parse_back_blast(
                            entry.text.as_str(),
                            &app.data_state.users,
                        );
                        println!("{:?}", data);
                    }
                }
            }
        }
    }

    pub async fn back_blast_verified(&self, verified: bool, backblast: &BackBlastData) {
        if let Some(event_times) = &backblast.event_times {
            // only if event times exist, we can find the message times
            // scope to minimize lock
            let channel_data = {
                let app = self.app.lock().unwrap();
                app.data_state
                    .get_channel_data(PublicChannels::from(&backblast.ao))
                    .map(|data| {
                        let emoji = if verified { "white_check_mark" } else { "x" };
                        ReactionsAddRequest::new(
                            data.id.to_string(),
                            emoji,
                            event_times.ts.to_string(),
                        )
                    })
            };
            if let Some(request) = channel_data {
                let url = request.get_url_request(&self.base_api_url);
                println!("Calling: {:?}", url.as_str());
                let response = self.make_get_url_request(url).await;
                let response: ReactionsAddResponse =
                    serde_json::from_slice(&response.body).expect("Could not parse response");
                println!("Emoji added!: {}", response.ok);
                if !response.ok {
                    eprintln!(
                        "Err: {}",
                        response.error.unwrap_or_else(|| "err".to_string())
                    );
                }
            }
        }
    }

    fn get_auth_header(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        let bearer = format!("Bearer {}", self.token);
        header_map.insert(AUTH_HEADER, bearer.parse().unwrap());
        header_map
    }

    async fn make_get_url_request(&self, url: url::Url) -> oauth2::HttpResponse {
        let request = oauth2::HttpRequest {
            url,
            method: Method::GET,
            headers: self.get_auth_header(),
            body: Vec::new(),
        };
        async_http_client(request)
            .await
            .expect("Failed to make request")
    }
}

pub struct WebAppState {
    pub data_state: AppState,
}
