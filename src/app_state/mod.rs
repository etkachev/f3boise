use crate::app_state::backblast_data::BACK_BLAST_TAG;
use crate::slack_api::channels::{
    history_request::ChannelHistoryRequest,
    history_response::{ChannelsHistoryResponse, MessageData},
    list_request::ConversationListRequest,
    list_response::{ChannelData, ChannelsListResponse},
    public_channels::PublicChannels,
    types::ChannelTypes,
};
use crate::slack_api::url_requests::SlackUrlRequest;
use crate::slack_api::users::{
    users_list_request::UsersListRequest, users_list_response::UsersListResponse,
};
use crate::users::f3_user::F3User;
use http::{HeaderMap, Method};
use oauth2::reqwest::http_client;
use std::collections::HashMap;

pub mod ao_data;
pub mod backblast_data;
pub mod parse_backblast;

pub struct AppState {
    token: String,
    base_api_url: String,
    channels: HashMap<PublicChannels, ChannelData>,
    users: HashMap<String, F3User>,
}

const AUTH_HEADER: &str = "Authorization";

impl AppState {
    pub fn new(token: String) -> Self {
        AppState {
            token,
            ..Default::default()
        }
    }

    pub fn get_back_blasts(&self) {
        if let Some(channel_data) = self.get_channel_data(PublicChannels::BotPlayground) {
            let request = ChannelHistoryRequest::new(&channel_data.id);
            let url = request.get_url_request(&self.base_api_url);
            println!("Calling: {:?}", url.as_str());
            let response = self.make_get_url_request(url);
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

    pub fn initialize_data(&mut self) {
        self.get_public_channels();
        self.get_users();
    }

    fn get_users(&mut self) {
        let url = UsersListRequest::default().get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url);
        let response: UsersListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");
        if let Some(users) = response.members {
            let users_map: HashMap<String, F3User> = users
                .into_iter()
                .filter(|user| !user.is_bot && !user.deleted)
                .fold(HashMap::new(), |mut acc, user| {
                    acc.insert(user.id.to_string(), F3User::from(user));
                    acc
                });
            self.users = users_map;
        } else {
            eprintln!("{:?}", response.error);
        }
    }

    fn get_public_channels(&mut self) {
        let request = ConversationListRequest::with_types(vec![ChannelTypes::Public]);
        let url = request.get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url);

        let response: ChannelsListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");

        if let Some(channels) = response.channels {
            let public_channels: HashMap<PublicChannels, ChannelData> =
                channels
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, channel| {
                        let channel_name = PublicChannels::from(channel.name.to_string());
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

    fn make_get_url_request(&self, url: url::Url) -> oauth2::HttpResponse {
        http_client(oauth2::HttpRequest {
            url,
            method: Method::GET,
            headers: self.get_auth_header(),
            body: Vec::new(),
        })
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
