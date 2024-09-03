use crate::bot_data::{BotUser, UserBotCombo};
use crate::oauth_client::get_oauth_client;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::channels::history::request::ChannelHistoryRequest;
use crate::slack_api::channels::history::response::ChannelsHistoryResponse;
use crate::slack_api::channels::invite::request::InviteToConvoRequest;
use crate::slack_api::channels::invite::response::InviteToConvoResponse;
use crate::slack_api::channels::kick::request::KickFromChannelRequest;
use crate::slack_api::channels::kick::response::KickFromChannelResponse;
use crate::slack_api::channels::list::request::ConversationListRequest;
use crate::slack_api::channels::list::response::{ChannelData, ChannelsListResponse};
use crate::slack_api::channels::members::request::ConversationMembersRequest;
use crate::slack_api::channels::members::response::ConversationMembersResponse;
use crate::slack_api::channels::open::request::OpenConversationRequest;
use crate::slack_api::channels::open::response::OpenConversationResponse;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::channels::types::ChannelTypes;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::slack_api::chat::post_message::response::PostMessageResponse;
use crate::slack_api::chat::update_message::request::UpdateMessageRequest;
use crate::slack_api::chat::update_message::response::UpdateMessageResponse;
use crate::slack_api::files::complete_upload_url_external;
use crate::slack_api::files::files_list::request::FilesListRequest;
use crate::slack_api::files::files_list::response::{FilesListItem, FilesListResponse};
use crate::slack_api::files::get_upload_url_external::request::GetUploadUrlExternalRequest;
use crate::slack_api::files::get_upload_url_external::response::GetUploadUrlExternalResponse;
use crate::slack_api::files::request::FileUpload;
use crate::slack_api::url_requests::SlackUrlRequest;
use crate::slack_api::users::users_list::request::UsersListRequest;
use crate::slack_api::users::users_list::response::UsersListResponse;
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::slack_api::views::response::ViewsOpenResponse;
use crate::users::f3_user::F3User;
use http::header::CONTENT_TYPE;
use oauth2::basic::BasicClient;
use reqwest::header::HeaderMap;
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
    /// secret key for handling internal calls
    pub boise_key: String,
}

impl MutableWebState {
    pub async fn get_public_channels(
        &self,
    ) -> Result<HashMap<PublicChannels, ChannelData>, AppError> {
        let request = ConversationListRequest::with_types(vec![ChannelTypes::Public]);
        let url = request.get_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let response = self.make_get_url_request(url).await;
        let bytes = response.bytes().await?;
        let response: ChannelsListResponse = serde_json::from_slice(&bytes)?;

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
        let bytes = response.bytes().await?;
        let response: UsersListResponse =
            serde_json::from_slice(&bytes).expect("Could not parse response");
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
        let bytes = response.bytes().await?;
        let response: ConversationMembersResponse =
            serde_json::from_slice(&bytes).expect("Could not parse response");
        if let Some(members) = response.members {
            Ok(members)
        } else {
            Err(AppError::General(
                response.error.unwrap_or_else(|| "No error".to_string()),
            ))
        }
    }

    /// post message to someone or channel. return ts from message posted
    pub async fn post_message(
        &self,
        request: PostMessageRequest,
    ) -> Result<Option<String>, AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let bytes = response.bytes().await?;
        let response: PostMessageResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(response.ts)
        }
    }

    pub async fn send_direct_message(
        &self,
        user: &str,
        message: BlockBuilder,
    ) -> Result<(), AppError> {
        let users = vec![user.to_string()];
        self.send_direct_messages(&users, message).await?;
        Ok(())
    }

    pub async fn send_direct_messages(
        &self,
        users: &[String],
        message: BlockBuilder,
    ) -> Result<(), AppError> {
        let request = OpenConversationRequest::new(users);
        let url = request.get_plain_url_request(&self.base_api_url);
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let bytes = response.bytes().await?;
        let response: OpenConversationResponse = serde_json::from_slice(&bytes)?;
        let channel_id = response.channel.map(|c| c.id);
        match (channel_id, response.error) {
            (_, Some(err)) => {
                println!("Error opening conversation: {}", err);
            }
            (Some(channel_id), _) => {
                let post_message_req = PostMessageRequest::new(&channel_id, message.blocks);
                self.post_message(post_message_req).await?;
            }
            _ => (),
        }
        Ok(())
    }

    /// Open view like modal or home view
    pub async fn open_view(&self, request: ViewsOpenRequest) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let bytes = response.bytes().await?;
        let response: ViewsOpenResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else if let Some(re) = response.view {
            println!("id: {:?}", re.id);
            Ok(())
        } else {
            Ok(())
        }
    }

    /// get files from slack workspace
    pub async fn get_files(
        &self,
        request: FilesListRequest,
    ) -> Result<Vec<FilesListItem>, AppError> {
        let url = request.get_url_request(&self.base_api_url);
        let response = self.make_get_url_request(url).await;
        let bytes = response.bytes().await?;
        let response: FilesListResponse = serde_json::from_slice(&bytes)?;
        match response {
            FilesListResponse {
                error: Some(err), ..
            } => Err(AppError::General(err)),
            FilesListResponse {
                files: Some(files), ..
            } => Ok(files),
            _ => Err(AppError::General(
                "Missing data from get files response".to_string(),
            )),
        }
    }

    pub async fn upload_file(&self, request: FileUpload) -> Result<(), AppError> {
        let step_1 = GetUploadUrlExternalRequest::new(&request.filename, request.file.len());
        let url = step_1.get_url_request(&self.base_api_url);
        let response = self.make_get_url_request(url).await;
        let bytes = response.bytes().await?;
        let response: GetUploadUrlExternalResponse = serde_json::from_slice(&bytes)?;

        match response {
            GetUploadUrlExternalResponse {
                error: Some(err), ..
            } => Err(AppError::General(err)),
            GetUploadUrlExternalResponse {
                error: None,
                upload_url: Some(upload_url),
                file_id: Some(file_id),
                ..
            } => {
                let url = url::Url::parse(&upload_url)?;
                let response = self.make_form_post(url, request.get_form_request()).await;
                if response.status().is_success() {
                    let step_3 = complete_upload_url_external::request::CompleteUploadUrlExternalRequest::new_single(&file_id, request.title.clone()).for_channel(&request.channel_id);
                    let url = step_3.get_plain_url_request(&self.base_api_url);
                    let body = serde_json::to_vec(&step_3)?;
                    let response = self.make_post_request(url, body).await;
                    let bytes = response.bytes().await?;
                    let response: complete_upload_url_external::response::CompleteUploadUrlExternalResponse = serde_json::from_slice(&bytes)?;
                    if let Some(err) = response.error {
                        Err(AppError::General(err))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(AppError::General("Error uploading file".to_string()))
                }
            }
            _ => Err(AppError::General(
                "Missing file id or upload url".to_string(),
            )),
        }
    }

    /// update message that exists in slack. returns ts
    pub async fn update_message(
        &self,
        request: UpdateMessageRequest,
    ) -> Result<Option<String>, AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        println!("Calling: {:?}", url.as_str());
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let bytes = response.bytes().await?;
        let response: UpdateMessageResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(response.ts)
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
        let bytes = response.bytes().await?;
        let response: InviteToConvoResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
        }
    }

    pub async fn kick_user_from_channel(
        &self,
        request: KickFromChannelRequest,
    ) -> Result<(), AppError> {
        let url = request.get_plain_url_request(&self.base_api_url);
        let body = serde_json::to_vec(&request)?;
        let response = self.make_post_request(url, body).await;
        let bytes = response.bytes().await?;
        let response: KickFromChannelResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(())
        }
    }

    pub async fn get_history(
        &self,
        request: ChannelHistoryRequest,
    ) -> Result<ChannelsHistoryResponse, AppError> {
        let url = request.get_url_request(&self.base_api_url);
        let response = self.make_get_url_request(url).await;
        let bytes = response.bytes().await?;
        let response: ChannelsHistoryResponse = serde_json::from_slice(&bytes)?;
        if let Some(err) = response.error {
            Err(AppError::General(err))
        } else {
            Ok(response)
        }
    }

    fn get_auth_header(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        let bearer = format!("Bearer {}", self.token);
        header_map.insert(AUTH_HEADER, bearer.parse().unwrap());
        header_map
    }

    async fn make_get_url_request(&self, url: url::Url) -> reqwest::Response {
        let client = reqwest::Client::new();
        client
            .get(url)
            .headers(self.get_auth_header())
            .send()
            .await
            .expect("Failed to make request")
        // let request = oauth2::HttpRequest {
        //     url,
        //     method: Method::GET,
        //     headers: self.get_auth_header(),
        //     body: Vec::new(),
        // };
        // async_http_client(request)
        //     .await
        //     .expect("Failed to make request")
    }

    async fn make_post_request(&self, url: url::Url, body: Vec<u8>) -> reqwest::Response {
        let client = reqwest::Client::new();
        let mut headers = self.get_auth_header();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .expect("Failed to make request")
        // let mut request = oauth2::HttpRequest {
        //     url,
        //     method: Method::POST,
        //     headers: self.get_auth_header(),
        //     body,
        // };
        // request
        //     .headers
        //     .insert(CONTENT_TYPE, "application/json".parse().unwrap());
        // async_http_client(request)
        //     .await
        //     .expect("Failed to make request")
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
            boise_key: String::new(),
        }
    }
}
