use crate::bot_data::{BotUser, UserBotCombo};
use crate::oauth_client::get_oauth_client;
use crate::shared::common_errors::AppError;
use crate::slack_api::channels::invite::request::InviteToConvoRequest;
use crate::slack_api::channels::invite::response::InviteToConvoResponse;
use crate::slack_api::channels::list::request::ConversationListRequest;
use crate::slack_api::channels::list::response::{ChannelData, ChannelsListResponse};
use crate::slack_api::channels::members::request::ConversationMembersRequest;
use crate::slack_api::channels::members::response::ConversationMembersResponse;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::channels::reactions_add::request::ReactionsAddRequest;
use crate::slack_api::channels::reactions_add::response::ReactionsAddResponse;
use crate::slack_api::channels::types::ChannelTypes;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::slack_api::chat::post_message::response::PostMessageResponse;
use crate::slack_api::chat::update_message::request::UpdateMessageRequest;
use crate::slack_api::chat::update_message::response::UpdateMessageResponse;
use crate::slack_api::files::request::FileUploadRequest;
use crate::slack_api::files::response::FileUploadResponse;
use crate::slack_api::url_requests::SlackUrlRequest;
use crate::slack_api::users::users_list::request::UsersListRequest;
use crate::slack_api::users::users_list::response::UsersListResponse;
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::slack_api::views::response::ViewsOpenResponse;
use crate::users::f3_user::F3User;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, Method};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use std::collections::HashMap;

pub const LOCAL_URL: &str = "127.0.0.1";
pub const SLACK_SERVER: &str = "slack.com";
pub const PORT_NUMBER: u16 = 8080;
const AUTH_HEADER: &str = "Authorization";

pub struct MutableWebState {
    pub token: String,
    pub base_api_url: String,
    pub oauth: BasicClient,
    pub signing_secret: String,
    /// Deprecated verify token
    pub verify_token: String,
}

impl MutableWebState {
    pub async fn get_public_channels(
        &self,
    ) -> Result<HashMap<PublicChannels, ChannelData>, AppError> {
        let request = ConversationListRequest::with_types(vec![ChannelTypes::Public]);
        let url = request.get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;

        let response: ChannelsListResponse = serde_json::from_slice(&response.body)?;

        println!("Finished getting public channels");
        if let Some(channels) = response.channels {
            let public_channels: HashMap<PublicChannels, ChannelData> =
                channels
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, channel| {
                        let channel_name = PublicChannels::from(channel.name.to_string());
                        acc.insert(channel_name, channel);
                        acc
                    });
            Ok(public_channels)
        } else {
            Err(AppError::General(
                response.error.unwrap_or_else(|| "No error".to_string()),
            ))
        }
    }

    /// get users that exist in slack then sync it to local db
    pub async fn get_users(&self) -> Result<UserBotCombo, AppError> {
        let url = UsersListRequest::default().get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;
        let response: UsersListResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");
        println!("Got slack users back");
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
            Ok(users_bots)
        } else {
            Err(AppError::General(
                response.error.unwrap_or_else(|| "No Error".to_string()),
            ))
        }
    }

    /// get list of slack ids that are part of requested channel_id
    pub async fn get_channel_members(&self, channel_id: &str) -> Result<Vec<String>, AppError> {
        let url = ConversationMembersRequest::new(channel_id).get_url_request(&self.base_api_url);
        let response = self.make_get_url_request(url).await;
        let response: ConversationMembersResponse =
            serde_json::from_slice(&response.body).expect("Could not parse response");
        if let Some(members) = response.members {
            Ok(members)
        } else {
            Err(AppError::General(
                response.error.unwrap_or_else(|| "No error".to_string()),
            ))
        }
    }

    pub async fn get_back_blasts(&self) {
        // TODO
        // let history_request = {
        //     let app = self.app.lock().unwrap();
        //     app.get_channel_data(PublicChannels::BotPlayground)
        //         .map(|channel_data| ChannelHistoryRequest::new(&channel_data.id))
        // };
        // if let Some(request) = history_request {
        //     let url = request.get_url_request(&self.base_api_url);
        //     println!("Calling: {:?}", url.as_str());
        //     let response = self.make_get_url_request(url).await;
        //     let response: ChannelsHistoryResponse =
        //         serde_json::from_slice(&response.body).expect("Could not parse response");
        //     if let Some(messages) = response.messages {
        //         let backblasts = messages
        //             .iter()
        //             .filter(|message| {
        //                 let (first_line, _) = message.text.split_once('\n').unwrap_or(("", ""));
        //                 first_line.to_lowercase().starts_with(BACK_BLAST_TAG)
        //             })
        //             .collect::<Vec<&MessageData>>();
        //
        //         for entry in backblasts {
        //             println!("Entry: {}", entry.ts);
        //             // scoped to limit lock
        //             {
        //                 // TODO
        //                 // let app = self.app.lock().unwrap();
        //                 // let data =
        //                 //     parse_backblast::parse_back_blast(entry.text.as_str(), &app.users);
        //                 // println!("{:?}", data);
        //             }
        //         }
        //     }
        // }
    }

    pub async fn back_blast_verified(&self, channel_request: Option<ReactionsAddRequest>) {
        if let Some(request) = channel_request {
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

    /// post message to someone or channel
    pub async fn post_message(&self, request: PostMessageRequest) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let response: PostMessageResponse = serde_json::from_slice(&response.body)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
        }
    }

    /// Open view like modal or home view
    pub async fn open_view(&self, request: ViewsOpenRequest) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let response: ViewsOpenResponse = serde_json::from_slice(&response.body)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else if let Some(re) = response.view {
            println!("id: {:?}", re.id);
            Ok(())
        } else {
            Ok(())
        }
    }

    /// upload file to slack channel(s)
    pub async fn upload_file(&self, request: FileUploadRequest) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        let response = self.make_form_post(url, request.get_form_request()).await;
        let bytes = response.bytes().await?;
        let response: FileUploadResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
        }
    }

    /// update message that exists in slack
    pub async fn update_message(&self, request: UpdateMessageRequest) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let response: UpdateMessageResponse = serde_json::from_slice(&response.body)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
        }
    }

    /// invite list of users to channel
    pub async fn invite_users_to_channel(
        &self,
        request: InviteToConvoRequest,
    ) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let response: InviteToConvoResponse = serde_json::from_slice(&response.body)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
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

    async fn make_post_request(&self, url: url::Url, body: Vec<u8>) -> oauth2::HttpResponse {
        let mut request = oauth2::HttpRequest {
            url,
            method: Method::POST,
            headers: self.get_auth_header(),
            body,
        };
        request
            .headers
            .insert(CONTENT_TYPE, "application/json".parse().unwrap());
        async_http_client(request)
            .await
            .expect("Failed to make request")
    }

    /// make post request using multipart/form-data
    async fn make_form_post(
        &self,
        url: url::Url,
        form: reqwest::multipart::Form,
    ) -> reqwest::Response {
        let client = reqwest::Client::new();
        client
            .post(url)
            .headers(self.get_auth_header())
            .multipart(form)
            .send()
            .await
            .expect("Failed to make request")
    }
}

impl Default for MutableWebState {
    fn default() -> Self {
        MutableWebState {
            token: String::new(),
            base_api_url: String::new(),
            oauth: get_oauth_client(),
            signing_secret: String::new(),
            verify_token: String::new(),
        }
    }
}
