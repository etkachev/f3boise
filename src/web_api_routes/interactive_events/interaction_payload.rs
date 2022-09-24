use crate::slack_api::block_kit::TextObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InteractionPayload {
    BlockActions(BlockAction),
}

/// Example:
/// Object({"payload": String("{
/// \"type\":\"block_actions\",
/// \"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},
/// \"api_app_id\":\"A03UAGJC9QD\",
/// \"token\":\"iqHCM8gJry9vury2mmDiv0Os\",
/// \"container\":{\"type\":\"message\",
///    \"message_ts\":\"1663713060.000300\",
///    \"channel_id\":\"D03TJCRQKQR\",
///    \"is_ephemeral\":true},
/// \"trigger_id\":\"4133440533088.3923618272067.6114f29d387c6e49fbd8b8287ffff332\",
/// \"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},
/// \"enterprise\":null,
/// \"is_enterprise_install\":false,
/// \"channel\":{\"id\":\"D03TJCRQKQR\",\"name\":\"directmessage\"},
/// \"state\":{\"values\":{}},
/// \"response_url\":\"https:\\/\\/hooks.slack.com\\/actions\\/T03T5J6801Z\\/4109740859074\\/BsOJzAHGotpXKvryhSEpYfXh\",
/// \"actions\":[{
///   \"action_id\":\"11\\/05::bleach\",
///   \"block_id\":\"j4F\",
///   \"text\":{\"type\":\"plain_text\",\"text\":\"Sign up\",\"emoji\":true},
///   \"type\":\"button\",
///   \"action_ts\":\"1663716457.226860\"}]
/// }")})
///
/// Api reference: https://api.slack.com/reference/interaction-payloads/block-actions
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockAction {
    pub token: String,
    /// The user who interacted to trigger this request
    pub user: ActionUser,
    /// The source app surface the user initiated the interaction from.
    /// This will include the full state of the message, or the view within a modal or Home tab.
    /// If the source was an ephemeral message, this field will not be included
    pub container: Option<InteractionContainer>,
    /// A short-lived ID that can be used to open modals
    pub trigger_id: String,
    pub actions: Vec<ActionType>,
    /// source the interaction happened in.
    pub channel: Option<ActionChannel>,
    /// A short-lived webhook that can be used to send messages in response to interactions
    pub response_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionChannel {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Button(ButtonAction),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonAction {
    pub text: TextObject,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    /// Identifies the block within a surface that contained the interactive component that was used.
    /// See the reference guide for the block you're using for more info on the block_id field
    pub block_id: String,
    /// Identifies the interactive component itself. Some blocks can contain multiple interactive components,
    /// so the block_id alone may not be specific enough to identify the source component.
    /// See the reference guide for the interactive element you're using for more info on the action_id field
    pub action_id: String,
    // TODO below
    // pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionUser {
    pub id: String,
    pub username: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InteractionContainer {
    MessageAttachment,
    Message(MessageContainer),
    View(ViewContainer),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContainer {
    pub message_ts: String,
    pub channel_id: String,
    pub is_ephemeral: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewContainer {
    pub view_id: String,
}
