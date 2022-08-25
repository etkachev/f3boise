use crate::app_state::backblast_data::{BackBlastData, BACK_BLAST_TAG};
use crate::slack_api::channels::{
    history::{
        request::ChannelHistoryRequest,
        response::{ChannelsHistoryResponse, MessageData},
    },
    list::{
        request::ConversationListRequest,
        response::{ChannelData, ChannelsListResponse},
    },
    public_channels::PublicChannels,
    reactions_add::{request::ReactionsAddRequest, response::ReactionsAddResponse},
    types::ChannelTypes,
};
use crate::slack_api::url_requests::SlackUrlRequest;
use crate::slack_api::users::users_list::{request::UsersListRequest, response::UsersListResponse};
use crate::users::f3_user::F3User;
use http::{HeaderMap, Method};
use oauth2::reqwest::async_http_client;
use std::collections::HashMap;

pub mod ao_data;
pub mod backblast_data;
pub mod parse_backblast;

#[derive(Debug)]
pub struct AppState {
    token: String,
    base_api_url: String,
    pub channels: HashMap<PublicChannels, ChannelData>,
    pub users: HashMap<String, F3User>,
}

const AUTH_HEADER: &str = "Authorization";

impl AppState {
    pub fn new(token: String) -> Self {
        AppState {
            token,
            ..Default::default()
        }
    }

    pub async fn get_back_blasts(&self) {
        if let Some(channel_data) = self.get_channel_data(PublicChannels::BotPlayground) {
            let request = ChannelHistoryRequest::new(&channel_data.id);
            let url = request.get_url_request(&self.base_api_url);
            println!("Calling: {:?}", url.as_str());
            let response = self.make_get_url_request(url).await;
            let response: ChannelsHistoryResponse =
                serde_json::from_slice(&response.body).expect("Could not parse response");
            if let Some(messages) = response.messages {
                let backblasts = messages
                    .iter()
                    .filter(|message| {
                        let (first_line, _) =
                            message.text.split_once('\n').unwrap_or_else(|| ("", ""));
                        first_line.to_lowercase().starts_with(BACK_BLAST_TAG)
                    })
                    .collect::<Vec<&MessageData>>();

                for entry in backblasts {
                    println!("Entry: {}", entry.ts);
                    let data = parse_backblast::parse_back_blast(entry.text.as_str(), &self.users);
                    println!("{:?}", data);
                }
            }
        }
    }

    pub async fn initialize_data(&mut self) {
        self.get_public_channels().await;
        self.get_users().await;
    }

    async fn get_users(&mut self) {
        let url = UsersListRequest::default().get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;
        let response: UsersListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");
        if let Some(users) = response.members {
            let users_map: HashMap<String, F3User> = users
                .into_iter()
                .filter(|user| !user.is_bot && !user.deleted)
                .fold(HashMap::new(), |mut acc, user| {
                    acc.insert(user.id.to_string(), F3User::from(&user));
                    acc
                });
            self.users = users_map;
        } else {
            eprintln!("{:?}", response.error);
        }
    }

    pub fn add_user(&mut self, name: &str, user: F3User) {
        self.users.insert(name.to_string(), user);
    }

    pub async fn back_blast_verified(&self, verified: bool, backblast: &BackBlastData) {
        if let Some(event_times) = &backblast.event_times {
            // only if event times exist, we can find the message times
            if let Some(channel) = self.get_channel_data(PublicChannels::from(&backblast.ao)) {
                let emoji = if verified { "white_check_mark" } else { "x" };
                let request = ReactionsAddRequest::new(
                    channel.id.to_string(),
                    emoji,
                    event_times.ts.to_string(),
                );
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

    async fn get_public_channels(&mut self) {
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
            self.channels = public_channels;
        } else {
            eprintln!(
                "Error from response: {:?}",
                response.error.unwrap_or_else(|| "No error".to_string())
            );
        }
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

    fn get_auth_header(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        let bearer = format!("Bearer {}", self.token);
        header_map.insert(AUTH_HEADER, bearer.parse().unwrap());
        header_map
    }

    fn get_channel_data(&self, channel: PublicChannels) -> Option<&ChannelData> {
        self.channels.get(&channel)
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            token: String::default(),
            base_api_url: String::from("https://slack.com/api/"),
            channels: HashMap::new(),
            users: HashMap::new(),
        }
    }
}
