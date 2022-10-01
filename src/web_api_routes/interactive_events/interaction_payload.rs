use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::{BlockType, TextObject};
use serde::{Deserialize, Serialize};

/// example of parse:
/// {
/// \"type\":\"block_actions\",
/// \"user\":{
///   \"id\":\"U03T87KHRFE\",
///   \"username\":\"edwardtkachev\",
///   \"name\":\"edwardtkachev\",
///   \"team_id\":\"T03T5J6801Z\"},
/// \"api_app_id\":\"A03UAGJC9QD\",
/// \"token\":\"iqHCM8gJry9vury2mmDiv0Os\",
/// \"container\":{
///   \"type\":\"message\",
///   \"message_ts\":\"1664404128.235039\",
///   \"channel_id\":\"C03TZV5RRF1\",
///   \"is_ephemeral\":false},
/// \"trigger_id\":\"4171754484160.3923618272067.4c182ee4830c026dd97ead2642c77366\",
/// \"team\":{
///   \"id\":\"T03T5J6801Z\",
///   \"domain\":\"f3-boise\"},
/// \"enterprise\":null,
/// \"is_enterprise_install\":false,
/// \"channel\":{
///   \"id\":\"C03TZV5RRF1\",
///   \"name\":\"bot-playground\"},
/// \"message\":{
///   \"bot_id\":\"B03UG6KRSN8\",
///   \"type\":\"message\",
///   \"text\":\"This content can't be displayed.\",
///   \"user\":\"U03UDNMQTR8\",
///   \"ts\":\"1664404128.235039\",
///   \"app_id\":\"A03UAGJC9QD\",
///   \"team\":\"T03T5J6801Z\",
///   \"blocks\":[
///     {
///     \"type\":\"header\",
///     \"block_id\":\"Tat\\/\",
///     \"text\":{\"type\":\"plain_text\",\"text\":\":calendar: Q Line-up for gem :calendar:\",\"emoji\":true}
///     },
///     {
///     \"type\":\"context\",
///     \"block_id\":\"bm8G\",
///     \"elements\":[{\"type\":\"mrkdwn\",\"text\":\"*September 2022*  |  Fill em up!\",\"verbatim\":false}]
///     },
///     {
///     \"type\":\"divider\",
///     \"block_id\":\"oSTX6\"
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"jp\\/h\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`09\\/29 (Thu)` - <@U040KGJG4FR>\",\"verbatim\":false}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"x17Y\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/01 (Sat)` - ipc week 4\",\"verbatim\":false}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"\\/TMZt\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/04 (Tue)` - <@U0405B5P7MK>\",\"verbatim\":false}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"2\\/rq\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/06 (Thu)` - _EMPTY_\",\"verbatim\":false},
///     \"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-06::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Sign up\",\"emoji\":true}}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"NX4K\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/08 (Sat)` - tbd\",\"verbatim\":false}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"d=n+K\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/11 (Tue)` - focker bday q\",\"verbatim\":false}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"zC04\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/13 (Thu)` - _EMPTY_\",\"verbatim\":false},
///     \"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-13::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Sign up\",\"emoji\":true}}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"9x\\/qp\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/15 (Sat)` - _EMPTY_\",\"verbatim\":false},
///     \"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-15::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Sign up\",\"emoji\":true}}
///     },
///     {
///     \"type\":\"section\",
///     \"block_id\":\"9f+\",
///     \"text\":{\"type\":\"mrkdwn\",\"text\":\"gem::2022-09-28::2022-10-18\",\"verbatim\":false}
///     }]
/// },
/// \"state\":{\"values\":{}},
/// \"response_url\":\"https:\\/\\/hooks.slack.com\\/actions\\/T03T5J6801Z\\/4133491171767\\/XMtx4vsJilAzsXEdrRl4eGdW\",
/// \"actions\":[
///     {
///     \"action_id\":\"q_line_up::2022-10-06::gem\",
///     \"block_id\":\"2\\/rq\",
///     \"text\":{\"type\":\"plain_text\",\"text\":\"Sign up\",\"emoji\":true},
///     \"type\":\"button\",
///     \"action_ts\":\"1664404189.699355\"}
///   ]
/// }
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
///
/// Other parsed:
/// {
/// \"type\":\"block_actions\",
/// \"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},
/// \"api_app_id\":\"A03UAGJC9QD\",
/// \"token\":\"iqHCM8gJry9vury2mmDiv0Os\",
/// \"container\":{\"type\":\"message\",\"message_ts\":\"1664512453.175729\",\"channel_id\":\"C03TZV5RRF1\",\"is_ephemeral\":false},
/// \"trigger_id\":\"4148685348038.3923618272067.8a8e23e6711808036a4a5bf13213b120\",
/// \"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},
/// \"enterprise\":null,
/// \"is_enterprise_install\":false,
/// \"channel\":{\"id\":\"C03TZV5RRF1\",\"name\":\"bot-playground\"},
/// \"message\":{
///   \"bot_id\":\"B03UG6KRSN8\",
///   \"type\":\"message\",
///   \"text\":\"This content can't be displayed.\",
///   \"user\":\"U03UDNMQTR8\",
///   \"ts\":\"1664512453.175729\",
///   \"app_id\":\"A03UAGJC9QD\",
///   \"team\":\"T03T5J6801Z\",
///   \"blocks\":[
///     {\"type\":\"header\",\"block_id\":\"wmc\\/\",\"text\":{\"type\":\"plain_text\",\"text\":\":calendar: Q Line-up for gem :calendar:\",\"emoji\":true}},
///     {\"type\":\"context\",\"block_id\":\"qRqW\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"*September 2022*  |  Fill em up!\",\"verbatim\":false}]},
///     {\"type\":\"divider\",\"block_id\":\"4KWh\"},
///     {\"type\":\"section\",\"block_id\":\"hlOAJ\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/01 (Sat)` - ipc week 4\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"clear::q_line_up::2022-10-01::gem\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear.todo\"}]}},
///     {\"type\":\"section\",\"block_id\":\"vXx6h\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/04 (Tue)` - <@U0405B5P7MK>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"clear::q_line_up::2022-10-04::gem\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear.todo\"}]}},
///     {\"type\":\"section\",\"block_id\":\"GQdVG\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/06 (Thu)` - <@U0412T740US>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"clear::q_line_up::2022-10-06::gem\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear.todo\"}]}},
///     {\"type\":\"section\",\"block_id\":\"hqyZ\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/08 (Sat)` - tbd\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"clear::q_line_up::2022-10-08::gem\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear.todo\"}]}},
///     {\"type\":\"section\",\"block_id\":\"uPj\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/11 (Tue)` - <@U041869UWDB> bday q\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"clear::q_line_up::2022-10-11::gem\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear.todo\"}]}},
///     {\"type\":\"section\",\"block_id\":\"XHXv\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/13 (Thu)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-13::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},
///     {\"type\":\"section\",\"block_id\":\"BnCYV\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/15 (Sat)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-15::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},
///     {\"type\":\"section\",\"block_id\":\"9rk\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`10\\/18 (Tue)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2022-10-18::gem\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},
///     {\"type\":\"divider\",\"block_id\":\"g8If\"},{\"type\":\"section\",\"block_id\":\"bU3\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"gem::2022-09-29::2022-10-19\",\"verbatim\":false}}]},
/// \"state\":{\"values\":{}},
/// \"response_url\":\"https:\\/\\/hooks.slack.com\\/actions\\/T03T5J6801Z\\/4152347306069\\/x43nRgCs7GYUBLZJzuK8kgjj\",
/// \"actions\":[
///   {
///     \"type\":\"overflow\",
///     \"action_id\":\"clear::q_line_up::2022-10-08::gem\",
///     \"block_id\":\"hqyZ\",
///     \"selected_option\":{
///       \"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},
///       \"value\":\"Clear.todo\"
///     },
///     \"action_ts\":\"1664512496.833098\"
///   }
/// ]
/// }
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
    /// actions that user acted upon like buttons, overflow, etc
    pub actions: Vec<ActionType>,
    /// source the interaction happened in.
    pub channel: Option<ActionChannel>,
    /// A short-lived webhook that can be used to send messages in response to interactions
    pub response_url: String,
    /// optional message that is part of where action came from.
    pub message: Option<InteractionMessageTypes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InteractionMessageTypes {
    Message(InteractionMessage),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InteractionMessage {
    pub bot_id: String,
    /// if from blocks, then this would say "This content can't be displayed
    pub text: String,
    /// slack user id
    pub user: String,
    /// timestamp the message happened
    pub ts: String,
    /// blocks of the message
    pub blocks: Option<Vec<BlockType>>,
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
    Overflow(OverflowAction),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OverflowAction {
    pub selected_option: OptionElement,
    #[serde(flatten)]
    pub action: Action,
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
