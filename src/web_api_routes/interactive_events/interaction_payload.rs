use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::{BlockType, TextObject};
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InteractionPayload {
    /// type "block_actions" maps to BlockAction
    BlockActions,
    /// type "view_submission" maps to ViewSubmissionPayload
    ViewSubmission,
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

///"{
/// \"type\":\"view_submission\",
/// \"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},
/// \"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},
/// \"api_app_id\":\"A03UAGJC9QD\",
/// \"token\":\"iqHCM8gJry9vury2mmDiv0Os\",
/// \"trigger_id\":\"4859070373875.3923618272067.002ead2ade9a13c2cdfe3e18851ffcff\",
/// \"view\":{
///     \"id\":\"V04RBKDRGSY\",
///     \"team_id\":\"T03T5J6801Z\",
///     \"type\":\"modal\",
///     \"blocks\":[{
///         \"type\":\"input\",
///         \"block_id\":\"\\/k9Dq\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Title\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"plain_text_input\",
///             \"action_id\":\"title.input\",
///             \"placeholder\":{\"type\":\"plain_text\",\"text\":\"Snarky Title\",\"emoji\":true},
///             \"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"Xyw3\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"AO\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"channels_select\",
///             \"action_id\":\"ao.select\",
///             \"initial_channel\":\"C03TZV5RRF1\"
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"SUF\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Workout Date\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"datepicker\",
///             \"action_id\":\"date.select\",
///             \"initial_date\":\"2023-02-25\"
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"KWFc\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Workout Time\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"timepicker\",
///             \"action_id\":\"time.select\"
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"aha\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"The Q(s)\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"multi_users_select\",
///             \"action_id\":\"qs.select\",\"initial_users\":[\"U03T87KHRFE\"]
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"O9jl\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"The Why\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"plain_text_input\",
///             \"action_id\":\"why.input\",
///             \"dispatch_action_config\":{
///                 \"trigger_actions_on\":[\"on_enter_pressed\"]
///             }
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"kqt\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Equipment\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"multi_static_select\",
///             \"action_id\":\"equipment.select\",
///             \"initial_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}],
///             \"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}]
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"kNii\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Other Equipment\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"plain_text_input\",
///             \"action_id\":\"other_equipment.input\",
///             \"placeholder\":{\"type\":\"plain_text\",\"text\":\"Anything else to bring?\",\"emoji\":true},
///             \"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"yGq\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"FNGs\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"plain_text_input\",
///             \"action_id\":\"fngs.input\",
///             \"initial_value\":\"Always\",
///             \"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"aZA1p\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"The Moleskine\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"plain_text_input\",
///             \"action_id\":\"moleskin.textbox\",
///             \"initial_value\":\"Notice\",
///             \"multiline\":true,
///             \"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}
///         }
///     },{
///         \"type\":\"input\",
///         \"block_id\":\"40Vy\",
///         \"label\":{\"type\":\"plain_text\",\"text\":\"Choose where to post this\",\"emoji\":true},
///         \"optional\":false,
///         \"dispatch_action\":false,
///         \"element\":{
///             \"type\":\"static_select\",
///             \"action_id\":\"where_to_post.select\",
///             \"initial_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"The AO Channel\",\"emoji\":true},\"value\":\"ao_channel\"},
///             \"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"The AO Channel\",\"emoji\":true},\"value\":\"ao_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Me\",\"emoji\":true},\"value\":\"self\"}]
///         }
///     },{
///         \"type\":\"context\",
///         \"block_id\":\"=FXxk\",
///         \"elements\":[{\"type\":\"mrkdwn\",\"text\":\"Please wait after hitting submit!\",\"verbatim\":false}]
///     }],
///     \"private_metadata\":\"\",
///     \"callback_id\":\"\",
///     \"state\":{
///         \"values\":{
///             \"\\/k9Dq\":{
///                 \"title.input\":{
///                     \"type\":\"plain_text_input\",
///                     \"value\":\"test t\"
///                 }
///             },
///             \"Xyw3\":{
///                 \"ao.select\":{
///                     \"type\":\"channels_select\",
///                     \"selected_channel\":\"C03TZV5RRF1\"
///                 }
///             },
///             \"SUF\":{
///                 \"date.select\":{
///                     \"type\":\"datepicker\",
///                     \"selected_date\":\"2023-02-25\"
///                 }
///             },
///             \"KWFc\":{
///                 \"time.select\":{
///                     \"type\":\"timepicker\",
///                     \"selected_time\":\"01:00\"
///                 }
///             },
///             \"aha\":{
///                 \"qs.select\":{
///                     \"type\":\"multi_users_select\",
///                     \"selected_users\":[\"U03T87KHRFE\"]
///                 }
///             },
///             \"O9jl\":{
///                 \"why.input\":{
///                     \"type\":\"plain_text_input\",
///                     \"value\":\"test\"
///                 }
///             },
///             \"kqt\":{
///                 \"equipment.select\":{
///                     \"type\":\"multi_static_select\",
///                     \"selected_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}]
///                 }
///             },
///             \"kNii\":{
///                 \"other_equipment.input\":{
///                     \"type\":\"plain_text_input\",
///                     \"value\":\"other\"
///                 }
///             },
///             \"yGq\":{
///                 \"fngs.input\":{
///                     \"type\":\"plain_text_input\",
///                     \"value\":\"Always\"
///                 }
///             },
///             \"aZA1p\":{
///                 \"moleskin.textbox\":{
///                     \"type\":\"plain_text_input\",
///                     \"value\":\"Notice\"
///                 }
///             },
///             \"40Vy\":{
///                 \"where_to_post.select\":{
///                     \"type\":\"static_select\",
///                     \"selected_option\":{
///                         \"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},
///                         \"value\":\"current_channel\"
///                     }
///                 }
///             }
///         }
///     },
///     \"hash\":\"1677310976.Chuf6Jgv\",
///     \"title\":{\"type\":\"plain_text\",\"text\":\"Pre Blast\",\"emoji\":true},
///     \"clear_on_close\":false,
///     \"notify_on_close\":false,
///     \"close\":null,
///     \"submit\":{\"type\":\"plain_text\",\"text\":\"Submit\",\"emoji\":true},
///     \"previous_view_id\":null,
///     \"root_view_id\":\"V04RBKDRGSY\",
///     \"app_id\":\"A03UAGJC9QD\",
///     \"external_id\":\"\",
///     \"app_installed_team_id\":\"T03T5J6801Z\",
///     \"bot_id\":\"B03UG6KRSN8\"
/// },
/// \"response_urls\":[],
/// \"is_enterprise_install\":false,
/// \"enterprise\":null}"
#[derive(Serialize, Deserialize, Debug)]
pub struct ViewSubmissionPayload {
    /// The user who interacted to trigger this request
    pub user: ActionUser,
    pub view: ViewSubmissionPayloadView,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ViewSubmissionPayloadView {
    Modal(ViewSubmissionPayloadViewModal),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewSubmissionPayloadViewModal {
    pub id: String,
    pub external_id: Option<String>,
    /// An optional string that will be sent to your app in view_submission and block_actions events. Max length of 3000 characters
    pub private_metadata: Option<String>,
    pub state: InteractionStateValues,
}

impl ViewSubmissionPayloadViewModal {
    pub fn modal_view_id(&self) -> Option<ViewIds> {
        self.external_id
            .as_ref()
            .map(|external_id| ViewIds::from(external_id.as_str()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionStateValues {
    pub values: HashMap<String, HashMap<String, StateValueOptions>>,
}

impl InteractionStateValues {
    pub fn get_values(&self) -> HashMap<String, BasicValue> {
        let mut results: HashMap<String, BasicValue> = HashMap::new();

        for obj in self.values.values() {
            for (key, value_option) in obj {
                let key = key.to_string();
                match value_option {
                    StateValueOptions::MultiStaticSelect(MultiStaticSelectValue {
                        selected_options,
                    }) => {
                        results.insert(
                            key,
                            BasicValue::Multi(
                                selected_options
                                    .iter()
                                    .map(|op| op.value.to_string())
                                    .collect(),
                            ),
                        );
                    }
                    StateValueOptions::StaticSelect(StaticSelectValue { selected_option }) => {
                        results.insert(
                            key,
                            BasicValue::Single(
                                selected_option
                                    .as_ref()
                                    .map(|op| op.value.to_string())
                                    .unwrap_or_default(),
                            ),
                        );
                    }
                    StateValueOptions::MultiUsersSelect(MultiUserSelectValue {
                        selected_users,
                    }) => {
                        results.insert(
                            key,
                            BasicValue::Multi(
                                selected_users.iter().map(|user| user.to_string()).collect(),
                            ),
                        );
                    }
                    StateValueOptions::ChannelsSelect(ChannelSelectValue { selected_channel }) => {
                        results.insert(
                            key,
                            BasicValue::Single(
                                selected_channel
                                    .as_ref()
                                    .map(|ch| ch.to_string())
                                    .unwrap_or_default(),
                            ),
                        );
                    }
                    StateValueOptions::PlainTextInput(PlainTextValue { value }) => {
                        results.insert(
                            key,
                            BasicValue::Single(
                                value.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                            ),
                        );
                    }
                    StateValueOptions::Datepicker(DatePickerValue { selected_date }) => {
                        results.insert(
                            key,
                            BasicValue::Single(
                                selected_date
                                    .as_ref()
                                    .map(|date| date.to_string())
                                    .unwrap_or_default(),
                            ),
                        );
                    }
                    StateValueOptions::Timepicker(TimePickerValue { selected_time }) => {
                        results.insert(
                            key,
                            BasicValue::Single(
                                selected_time
                                    .as_ref()
                                    .map(|time| time.to_string())
                                    .unwrap_or_default(),
                            ),
                        );
                    }
                    StateValueOptions::FileInput(FileInputValue { files }) => {
                        let file_urls: Vec<String> = files
                            .iter()
                            .map(|file| file.permalink.to_string())
                            .collect();
                        results.insert(key, BasicValue::Multi(file_urls));
                    }
                }
            }
        }
        results
    }
}

#[derive(Debug, PartialEq)]
pub enum BasicValue {
    Single(String),
    Multi(Vec<String>),
}

impl BasicValue {
    pub fn get_single(&self) -> Option<String> {
        if let BasicValue::Single(value) = self {
            Some(value.to_string())
        } else {
            None
        }
    }

    pub fn get_multi_value(&self) -> Option<Vec<String>> {
        if let BasicValue::Multi(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StateValueOptions {
    StaticSelect(StaticSelectValue),
    PlainTextInput(PlainTextValue),
    MultiStaticSelect(MultiStaticSelectValue),
    MultiUsersSelect(MultiUserSelectValue),
    Timepicker(TimePickerValue),
    Datepicker(DatePickerValue),
    ChannelsSelect(ChannelSelectValue),
    FileInput(FileInputValue),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlainTextValue {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticSelectValue {
    pub selected_option: Option<OptionElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelSelectValue {
    pub selected_channel: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatePickerValue {
    pub selected_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInputValue {
    pub files: Vec<FileItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileItem {
    pub id: String,
    pub title: String,
    pub mimetype: String,
    pub filetype: String,
    /// id of slack user who uploaded
    pub user: String,
    pub size: usize,
    pub permalink: String,
    pub permalink_public: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePickerValue {
    pub selected_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiUserSelectValue {
    pub selected_users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiStaticSelectValue {
    pub selected_options: Vec<OptionElement>,
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
    pub user: Option<String>,
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

impl ActionType {
    pub fn get_block_id(&self) -> String {
        match self {
            ActionType::Button(ButtonAction { action, .. }) => action.block_id.to_string(),
            ActionType::Overflow(OverflowAction { action, .. }) => action.block_id.to_string(),
        }
    }

    pub fn get_action_id(&self) -> String {
        match self {
            ActionType::Button(ButtonAction { action, .. }) => action.action_id.to_string(),
            ActionType::Overflow(OverflowAction { action, .. }) => action.action_id.to_string(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web_api_routes::interactive_events::interaction_types::InteractionTypes;
    use crate::web_api_routes::slash_commands::black_diamond_rating::black_diamond_rating_post;
    use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post::PreBlastPost;
    use chrono::NaiveTime;

    #[test]
    fn q_sheet_interaction() {
        let payload = "{\"type\":\"block_actions\",\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"container\":{\"type\":\"message\",\"message_ts\":\"1677340709.667729\",\"channel_id\":\"C03TZV5RRF1\",\"is_ephemeral\":false},\"trigger_id\":\"4859714798418.3923618272067.519ba7c819d2f12c01366860d0b78ecd\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"enterprise\":null,\"is_enterprise_install\":false,\"channel\":{\"id\":\"C03TZV5RRF1\",\"name\":\"bot-playground\"},\"message\":{\"bot_id\":\"B03UG6KRSN8\",\"type\":\"message\",\"text\":\"This content can't be displayed.\",\"user\":\"U03UDNMQTR8\",\"ts\":\"1677340709.667729\",\"app_id\":\"A03UAGJC9QD\",\"blocks\":[{\"type\":\"header\",\"block_id\":\"o2N\",\"text\":{\"type\":\"plain_text\",\"text\":\":calendar: Q Line-up for bleach :calendar:\",\"emoji\":true}},{\"type\":\"context\",\"block_id\":\"XkZT\\/\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"*February 2023*  |  Fill em up!\",\"verbatim\":false}]},{\"type\":\"divider\",\"block_id\":\"pnL\"},{\"type\":\"section\",\"block_id\":\"4oE5\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`02\\/27 (Mon)` - <@U040VL1TAS3>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-02-27::bleach\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"}]}},{\"type\":\"section\",\"block_id\":\"laQ\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/01 (Wed)` - <@U04140ZQPM0>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-03-01::bleach\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"}]}},{\"type\":\"section\",\"block_id\":\"3Z6E\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/04 (Sat)` - <@U03SR452HL7>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-03-04::bleach\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"}]}},{\"type\":\"section\",\"block_id\":\"j+b\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/06 (Mon)` - <@U040B20NAS1>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-03-06::bleach\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"}]}},{\"type\":\"section\",\"block_id\":\"eKD\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/08 (Wed)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2023-03-08::bleach\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},{\"type\":\"section\",\"block_id\":\"DzNP\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/11 (Sat)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2023-03-11::bleach\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},{\"type\":\"section\",\"block_id\":\"RSW\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/13 (Mon)` - _EMPTY_\",\"verbatim\":false},\"accessory\":{\"type\":\"button\",\"action_id\":\"q_line_up::2023-03-13::bleach\",\"text\":{\"type\":\"plain_text\",\"text\":\"Take it\",\"emoji\":true}}},{\"type\":\"section\",\"block_id\":\"aWcyz\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"`03\\/15 (Wed)` - <@U03T87KHRFE>\",\"verbatim\":false},\"accessory\":{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-03-15::bleach\",\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"}]}},{\"type\":\"divider\",\"block_id\":\"JPk\"},{\"type\":\"section\",\"block_id\":\"DuVv\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"bleach::2023-02-25::2023-03-17\",\"verbatim\":false}}],\"team\":\"T03T5J6801Z\",\"edited\":{\"user\":\"B03UG6KRSN8\",\"ts\":\"1677340722.000000\"}},\"state\":{\"values\":{}},\"response_url\":\"https:\\/\\/hooks.slack.com\\/actions\\/T03T5J6801Z\\/4845181455591\\/e2BsuuqBvcQH6nq6w4DC5Jvd\",\"actions\":[{\"type\":\"overflow\",\"action_id\":\"q_line_up::2023-03-15::bleach\",\"block_id\":\"aWcyz\",\"selected_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Clear\",\"emoji\":true},\"value\":\"Clear\"},\"action_ts\":\"1677340727.381966\"}]}";

        let interaction_payload = serde_json::from_str::<InteractionPayload>(payload).unwrap();
        assert_eq!(interaction_payload, InteractionPayload::BlockActions);
        let block_action = serde_json::from_str::<BlockAction>(payload).unwrap();
        assert_eq!(block_action.user.username, "edwardtkachev".to_string());
    }

    #[test]
    fn black_diamond_rating() {
        let payload = "{\"type\":\"view_submission\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"trigger_id\":\"5126995803505.3923618272067.da27da029e24e150af8ab048745ce24f\",\"view\":{\"id\":\"V053EQTKLUC\",\"team_id\":\"T03T5J6801Z\",\"type\":\"modal\",\"blocks\":[{\"type\":\"input\",\"block_id\":\"4k5H\",\"label\":{\"type\":\"plain_text\",\"text\":\"Number of Pax\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"pax-count.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"How many Pax?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"E\\/C\",\"label\":{\"type\":\"plain_text\",\"text\":\"Vests Removed\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"vests-removed.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"How many vests removed?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"5nmR\",\"label\":{\"type\":\"plain_text\",\"text\":\"Miles\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"miles.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"How many miles?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"ukR\",\"label\":{\"type\":\"plain_text\",\"text\":\"Avg Heart Rate\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"avg-heart-rate.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Avg heart rate of pax\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"hprYk\",\"label\":{\"type\":\"plain_text\",\"text\":\"Where to Post\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"channels_select\",\"action_id\":\"where_to_post.select\",\"initial_channel\":\"C03TZV5RRF1\"}}],\"private_metadata\":\"\",\"callback_id\":\"\",\"state\":{\"values\":{\"4k5H\":{\"pax-count.input\":{\"type\":\"plain_text_input\",\"value\":\"8\"}},\"E\\/C\":{\"vests-removed.input\":{\"type\":\"plain_text_input\",\"value\":\"8\"}},\"5nmR\":{\"miles.input\":{\"type\":\"plain_text_input\",\"value\":\"3.72\"}},\"ukR\":{\"avg-heart-rate.input\":{\"type\":\"plain_text_input\",\"value\":\"140.5\"}},\"hprYk\":{\"where_to_post.select\":{\"type\":\"channels_select\",\"selected_channel\":\"C03TZV5RRF1\"}}}},\"hash\":\"1681506037.vmNbE5Uf\",\"title\":{\"type\":\"plain_text\",\"text\":\"Black Diamond Rating\",\"emoji\":true},\"clear_on_close\":false,\"notify_on_close\":false,\"close\":null,\"submit\":{\"type\":\"plain_text\",\"text\":\"Submit\",\"emoji\":true},\"previous_view_id\":null,\"root_view_id\":\"V053EQTKLUC\",\"app_id\":\"A03UAGJC9QD\",\"external_id\":\"black_diamond_rating::70697\",\"app_installed_team_id\":\"T03T5J6801Z\",\"bot_id\":\"B03UG6KRSN8\"},\"response_urls\":[],\"is_enterprise_install\":false,\"enterprise\":null}";
        let parsed = serde_json::from_str::<InteractionPayload>(payload).unwrap();
        assert_eq!(parsed, InteractionPayload::ViewSubmission);
        let parsed = serde_json::from_str::<ViewSubmissionPayload>(payload).unwrap();
        match parsed.view {
            ViewSubmissionPayloadView::Modal(modal) => {
                let values = modal.state.get_values();
                let post = black_diamond_rating_post::BlackDiamondRatingPost::from(values);
                println!("{:?}", post);
                assert_eq!(post.total_fmt(), 3.86.to_string());
            }
        }
    }

    #[test]
    fn pre_blast_submit() {
        let payload = "{\"type\":\"view_submission\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"trigger_id\":\"4859734125698.3923618272067.d1a479e9ace60fdce0f01083a6f76d77\",\"view\":{\"id\":\"V04RNAX0T6D\",\"team_id\":\"T03T5J6801Z\",\"type\":\"modal\",\"blocks\":[{\"type\":\"input\",\"block_id\":\"wXC7n\",\"label\":{\"type\":\"plain_text\",\"text\":\"Title\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"title.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Snarky Title\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"fOUB\",\"label\":{\"type\":\"plain_text\",\"text\":\"AO\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"channels_select\",\"action_id\":\"ao.select\",\"initial_channel\":\"C03TZV5RRF1\"}},{\"type\":\"input\",\"block_id\":\"sJs7\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Date\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"datepicker\",\"action_id\":\"date.select\",\"initial_date\":\"2023-02-25\"}},{\"type\":\"input\",\"block_id\":\"lfc\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Time\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"timepicker\",\"action_id\":\"time.select\"}},{\"type\":\"input\",\"block_id\":\"lqB\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Q(s)\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"multi_users_select\",\"action_id\":\"qs.select\",\"initial_users\":[\"U03T87KHRFE\"]}},{\"type\":\"divider\",\"block_id\":\"Jrw\"},{\"type\":\"input\",\"block_id\":\"+hS\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Why\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"why.input\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"CdmN\",\"label\":{\"type\":\"plain_text\",\"text\":\"Equipment\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"multi_static_select\",\"action_id\":\"equipment.select\",\"initial_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}],\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}]}},{\"type\":\"input\",\"block_id\":\"2Zt+\",\"label\":{\"type\":\"plain_text\",\"text\":\"Other Equipment\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"other_equipment.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Anything else to bring?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"kro=\",\"label\":{\"type\":\"plain_text\",\"text\":\"FNGs\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"fngs.input\",\"initial_value\":\"Always\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"divider\",\"block_id\":\"wCLAv\"},{\"type\":\"input\",\"block_id\":\"8Nz\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Moleskine\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"moleskin.textbox\",\"initial_value\":\"Notice\",\"multiline\":true,\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"d4Vq\",\"label\":{\"type\":\"plain_text\",\"text\":\"Choose where to post this\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"static_select\",\"action_id\":\"where_to_post.select\",\"initial_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"The AO Channel\",\"emoji\":true},\"value\":\"ao_channel\"},\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"The AO Channel\",\"emoji\":true},\"value\":\"ao_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Me\",\"emoji\":true},\"value\":\"self\"}]}},{\"type\":\"context\",\"block_id\":\"o=Mb\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"Please wait after hitting submit!\",\"verbatim\":false}]}],\"private_metadata\":\"\",\"callback_id\":\"\",\"state\":{\"values\":{\"wXC7n\":{\"title.input\":{\"type\":\"plain_text_input\",\"value\":\"First One\"}},\"fOUB\":{\"ao.select\":{\"type\":\"channels_select\",\"selected_channel\":\"C03UBFXVBGD\"}},\"sJs7\":{\"date.select\":{\"type\":\"datepicker\",\"selected_date\":\"2023-02-25\"}},\"lfc\":{\"time.select\":{\"type\":\"timepicker\",\"selected_time\":\"05:15\"}},\"lqB\":{\"qs.select\":{\"type\":\"multi_users_select\",\"selected_users\":[\"U03T87KHRFE\"]}},\"+hS\":{\"why.input\":{\"type\":\"plain_text_input\",\"value\":\"Come out\"}},\"CdmN\":{\"equipment.select\":{\"type\":\"multi_static_select\",\"selected_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupons\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"}]}},\"2Zt+\":{\"other_equipment.input\":{\"type\":\"plain_text_input\",\"value\":null}},\"kro=\":{\"fngs.input\":{\"type\":\"plain_text_input\",\"value\":\"Always\"}},\"8Nz\":{\"moleskin.textbox\":{\"type\":\"plain_text_input\",\"value\":\"Notice\"}},\"d4Vq\":{\"where_to_post.select\":{\"type\":\"static_select\",\"selected_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel\"}}}}},\"hash\":\"1677341558.4sEYtlrJ\",\"title\":{\"type\":\"plain_text\",\"text\":\"Pre Blast\",\"emoji\":true},\"clear_on_close\":false,\"notify_on_close\":false,\"close\":null,\"submit\":{\"type\":\"plain_text\",\"text\":\"Submit\",\"emoji\":true},\"previous_view_id\":null,\"root_view_id\":\"V04RNAX0T6D\",\"app_id\":\"A03UAGJC9QD\",\"external_id\":\"\",\"app_installed_team_id\":\"T03T5J6801Z\",\"bot_id\":\"B03UG6KRSN8\"},\"response_urls\":[],\"is_enterprise_install\":false,\"enterprise\":null}";
        let parsed = serde_json::from_str::<InteractionPayload>(payload).unwrap();
        assert_eq!(parsed, InteractionPayload::ViewSubmission);
        let parsed = serde_json::from_str::<ViewSubmissionPayload>(payload).unwrap();
        match parsed.view {
            ViewSubmissionPayloadView::Modal(modal) => {
                let values = modal.state.get_values();
                let where_post = values.get("where_to_post.select").unwrap();
                assert_eq!(
                    where_post,
                    &BasicValue::Single("current_channel".to_string())
                );

                let equipment = values.get("equipment.select").unwrap();
                assert_eq!(
                    equipment,
                    &BasicValue::Multi(vec![
                        "coupons".to_string(),
                        "sandbag".to_string(),
                        "ruck".to_string()
                    ])
                );

                let post = PreBlastPost::from(values);
                assert_eq!(post.start_time, NaiveTime::from_hms(5, 15, 0));
            }
        }
    }

    #[test]
    fn pre_blast_no_equipment() {
        let payload = "{\"type\":\"view_submission\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"trigger_id\":\"5279857144755.3923618272067.918c42855ae2929bf5a85815705de167\",\"view\":{\"id\":\"V0587UXTJRY\",\"team_id\":\"T03T5J6801Z\",\"type\":\"modal\",\"blocks\":[{\"type\":\"input\",\"block_id\":\"JjnV\",\"label\":{\"type\":\"plain_text\",\"text\":\"Title\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"title.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Snarky Title\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"BR0o\",\"label\":{\"type\":\"plain_text\",\"text\":\"AO\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"channels_select\",\"action_id\":\"ao.select\",\"initial_channel\":\"C03TZV5RRF1\"}},{\"type\":\"input\",\"block_id\":\"PGCiL\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Date\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"datepicker\",\"action_id\":\"date.select\",\"initial_date\":\"2023-05-18\"}},{\"type\":\"input\",\"block_id\":\"\\/Qc=u\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Time\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"timepicker\",\"action_id\":\"time.select\"}},{\"type\":\"input\",\"block_id\":\"qQ=\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Q(s)\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"multi_users_select\",\"action_id\":\"qs.select\",\"initial_users\":[\"U03T87KHRFE\"]}},{\"type\":\"divider\",\"block_id\":\"t6A2I\"},{\"type\":\"input\",\"block_id\":\"cg2N2\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Why\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"why.input\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"qCzD\",\"label\":{\"type\":\"plain_text\",\"text\":\"Equipment\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"multi_static_select\",\"action_id\":\"equipment.select\",\"initial_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupon\"}],\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupon\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Running Shoes \\ud83d\\udc5f\",\"emoji\":true},\"value\":\"shoes\"}]}},{\"type\":\"input\",\"block_id\":\"HwC\",\"label\":{\"type\":\"plain_text\",\"text\":\"Other Equipment\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"other_equipment.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Anything else to bring?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"ZU7\",\"label\":{\"type\":\"plain_text\",\"text\":\"FNGs\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"fngs.input\",\"initial_value\":\"Always\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"divider\",\"block_id\":\"XL=\"},{\"type\":\"input\",\"block_id\":\"bKy+J\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Moleskine\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"moleskin.textbox\",\"multiline\":true,\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"tZ+QY\",\"label\":{\"type\":\"plain_text\",\"text\":\"Choose where to post this\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"static_select\",\"action_id\":\"where_to_post.select\",\"initial_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Ao Channel\",\"emoji\":true},\"value\":\"ao_channel\"},\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Ao Channel\",\"emoji\":true},\"value\":\"ao_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel::C03TZV5RRF1\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Me\",\"emoji\":true},\"value\":\"self\"}]}},{\"type\":\"context\",\"block_id\":\"6Vw\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"Please wait after hitting submit!\",\"verbatim\":false}]}],\"private_metadata\":\"\",\"callback_id\":\"\",\"state\":{\"values\":{\"JjnV\":{\"title.input\":{\"type\":\"plain_text_input\",\"value\":\"testing no equipment\"}},\"BR0o\":{\"ao.select\":{\"type\":\"channels_select\",\"selected_channel\":\"C03UT46303T\"}},\"PGCiL\":{\"date.select\":{\"type\":\"datepicker\",\"selected_date\":\"2023-05-18\"}},\"\\/Qc=u\":{\"time.select\":{\"type\":\"timepicker\",\"selected_time\":\"06:00\"}},\"qQ=\":{\"qs.select\":{\"type\":\"multi_users_select\",\"selected_users\":[\"U03T87KHRFE\"]}},\"cg2N2\":{\"why.input\":{\"type\":\"plain_text_input\",\"value\":\"testing no equipment\"}},\"qCzD\":{\"equipment.select\":{\"type\":\"multi_static_select\",\"selected_options\":[]}},\"HwC\":{\"other_equipment.input\":{\"type\":\"plain_text_input\",\"value\":null}},\"ZU7\":{\"fngs.input\":{\"type\":\"plain_text_input\",\"value\":\"Always\"}},\"bKy+J\":{\"moleskin.textbox\":{\"type\":\"plain_text_input\",\"value\":\"blah blah\"}},\"tZ+QY\":{\"where_to_post.select\":{\"type\":\"static_select\",\"selected_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel::C03TZV5RRF1\"}}}}},\"hash\":\"1684335315.LvEXN9eH\",\"title\":{\"type\":\"plain_text\",\"text\":\"Pre Blast\",\"emoji\":true},\"clear_on_close\":false,\"notify_on_close\":false,\"close\":null,\"submit\":{\"type\":\"plain_text\",\"text\":\"Submit\",\"emoji\":true},\"previous_view_id\":null,\"root_view_id\":\"V0587UXTJRY\",\"app_id\":\"A03UAGJC9QD\",\"external_id\":\"pre_blast::96522\",\"app_installed_team_id\":\"T03T5J6801Z\",\"bot_id\":\"B03UG6KRSN8\"},\"response_urls\":[],\"is_enterprise_install\":false,\"enterprise\":null}";
        let parsed = serde_json::from_str::<ViewSubmissionPayload>(payload).unwrap();
        match parsed.view {
            ViewSubmissionPayloadView::Modal(modal) => {
                let values = modal.state.get_values();
                let equipment = values.get("equipment.select").unwrap();
                assert_eq!(equipment, &BasicValue::Multi(vec![]));
            }
        }
    }

    #[test]
    fn edit_back_blast() {
        let payload = "{\"type\":\"block_actions\",\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"container\":{\"type\":\"message\",\"message_ts\":\"1689882778.307339\",\"channel_id\":\"C03TZV5RRF1\",\"is_ephemeral\":false},\"trigger_id\":\"5616359762052.3923618272067.d52e2d86d98a2c36b62c9d8912326397\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"enterprise\":null,\"is_enterprise_install\":false,\"channel\":{\"id\":\"C03TZV5RRF1\",\"name\":\"bot-playground\"},\"message\":{\"type\":\"message\",\"subtype\":\"bot_message\",\"text\":\"*Slackblast*:\\n\\ntest\\n\\n*DATE*: 2023-07-20\\n\\n*AO*: <#C03UR7GM7Q9>\\n\\n*Q(s)*: <@U03T87KHRFE>\\n\\n*PAX*: <@U03SR452HL7>, TestPax\\n\\n*FNGs*: \\n\\n*COUNT*: 3 *WARMUP:* test\\n\\n*THE THANG:*\\nHi\\n\\n*MARY:*\\n*ANNOUNCEMENTS:*\\n*COT:* Edit Backblast button Saved backblast\",\"ts\":\"1689882778.307339\",\"username\":\"Stinger (via BoiseBot)\",\"icons\":{\"image_48\":\"https:\\/\\/s3-us-west-2.amazonaws.com\\/slack-files2\\/bot_icons\\/2023-03-10\\/4933887289794_48.png\"},\"bot_id\":\"B03UG6KRSN8\",\"app_id\":\"A03UAGJC9QD\",\"blocks\":[{\"type\":\"section\",\"block_id\":\"Giu2f\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"*Slackblast*:\\n\\ntest\\n\\n*DATE*: 2023-07-20\\n\\n*AO*: <#C03UR7GM7Q9>\\n\\n*Q(s)*: <@U03T87KHRFE>\\n\\n*PAX*: <@U03SR452HL7>, TestPax\\n\\n*FNGs*: \\n\\n*COUNT*: 3\",\"verbatim\":false}},{\"type\":\"divider\",\"block_id\":\"oXj\"},{\"type\":\"section\",\"block_id\":\"ztG\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"*WARMUP:* test\\n\\n*THE THANG:*\\nHi\\n\\n*MARY:*\\n*ANNOUNCEMENTS:*\\n*COT:*\",\"verbatim\":false}},{\"type\":\"actions\",\"block_id\":\"8Kt\",\"elements\":[{\"type\":\"button\",\"action_id\":\"edit_back_blast::fde3616a-413e-40ba-a8d1-ac1999baae8d\",\"text\":{\"type\":\"plain_text\",\"text\":\"Edit Backblast\",\"emoji\":true},\"value\":\"edit-backblast\"}]},{\"type\":\"context\",\"block_id\":\"i6b\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"Saved backblast\",\"verbatim\":false}]}]},\"state\":{\"values\":{}},\"response_url\":\"https:\\/\\/hooks.slack.com\\/actions\\/T03T5J6801Z\\/5599381131943\\/UcVNhazZlzor1mGkerodoInU\",\"actions\":[{\"action_id\":\"edit_back_blast::fde3616a-413e-40ba-a8d1-ac1999baae8d\",\"block_id\":\"8Kt\",\"text\":{\"type\":\"plain_text\",\"text\":\"Edit Backblast\",\"emoji\":true},\"value\":\"edit-backblast\",\"type\":\"button\",\"action_ts\":\"1689882782.229794\"}]}";
        let interaction_payload = serde_json::from_str::<InteractionPayload>(payload).unwrap();
        assert_eq!(interaction_payload, InteractionPayload::BlockActions);
        let block_action = serde_json::from_str::<BlockAction>(payload).unwrap();
        println!("{:?}", block_action);
        let first_action = block_action.actions.get(0).unwrap();
        let action = InteractionTypes::from(first_action.get_action_id().as_str());
        assert_eq!(
            action,
            InteractionTypes::EditBackBlast("fde3616a-413e-40ba-a8d1-ac1999baae8d".to_string())
        );
    }

    #[test]
    fn parse_file_input() {
        let payload = "{\"type\":\"view_submission\",\"team\":{\"id\":\"T03T5J6801Z\",\"domain\":\"f3-boise\"},\"user\":{\"id\":\"U03T87KHRFE\",\"username\":\"edwardtkachev\",\"name\":\"edwardtkachev\",\"team_id\":\"T03T5J6801Z\"},\"api_app_id\":\"A03UAGJC9QD\",\"token\":\"iqHCM8gJry9vury2mmDiv0Os\",\"trigger_id\":\"6481040169750.3923618272067.c592204d90d61abbc14320ad757a40a8\",\"view\":{\"id\":\"V06EBKNT2NN\",\"team_id\":\"T03T5J6801Z\",\"type\":\"modal\",\"blocks\":[{\"type\":\"input\",\"block_id\":\"lnMij\",\"label\":{\"type\":\"plain_text\",\"text\":\"Title\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"title.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Snarky Title\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"\\/o5Bh\",\"label\":{\"type\":\"plain_text\",\"text\":\"AO\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"channels_select\",\"action_id\":\"ao.select\",\"initial_channel\":\"C03TZV5RRF1\"}},{\"type\":\"input\",\"block_id\":\"bbmMo\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Date\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"datepicker\",\"action_id\":\"date.select\",\"initial_date\":\"2024-01-18\"}},{\"type\":\"input\",\"block_id\":\"YKm+V\",\"label\":{\"type\":\"plain_text\",\"text\":\"Workout Time\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"timepicker\",\"action_id\":\"time.select\"}},{\"type\":\"input\",\"block_id\":\"zd6aQ\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Q(s)\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"multi_users_select\",\"action_id\":\"qs.select\",\"initial_users\":[\"U03T87KHRFE\"]}},{\"type\":\"divider\",\"block_id\":\"juxUf\"},{\"type\":\"input\",\"block_id\":\"x87Qz\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Why\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"why.input\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"Tfv25\",\"label\":{\"type\":\"plain_text\",\"text\":\"Equipment\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"multi_static_select\",\"action_id\":\"equipment.select\",\"initial_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupon\"}],\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupon\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Sandbag \\ud83d\\udc5d\",\"emoji\":true},\"value\":\"sandbag\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Ruck \\ud83c\\udf92\",\"emoji\":true},\"value\":\"ruck\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Running Shoes \\ud83d\\udc5f\",\"emoji\":true},\"value\":\"shoes\"}]}},{\"type\":\"input\",\"block_id\":\"oo5gq\",\"label\":{\"type\":\"plain_text\",\"text\":\"Other Equipment\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"other_equipment.input\",\"placeholder\":{\"type\":\"plain_text\",\"text\":\"Anything else to bring?\",\"emoji\":true},\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"xVRyF\",\"label\":{\"type\":\"plain_text\",\"text\":\"FNGs\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"fngs.input\",\"initial_value\":\"Always\",\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"divider\",\"block_id\":\"OgJ7b\"},{\"type\":\"input\",\"block_id\":\"Y9MdG\",\"label\":{\"type\":\"plain_text\",\"text\":\"The Moleskine\",\"emoji\":true},\"optional\":true,\"dispatch_action\":false,\"element\":{\"type\":\"plain_text_input\",\"action_id\":\"moleskin.textbox\",\"multiline\":true,\"dispatch_action_config\":{\"trigger_actions_on\":[\"on_enter_pressed\"]}}},{\"type\":\"input\",\"block_id\":\"AwLSA\",\"label\":{\"type\":\"plain_text\",\"text\":\"Upload Image\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"file_input\",\"action_id\":\"file.input\",\"max_files\":10,\"max_file_size_bytes\":10000000}},{\"type\":\"input\",\"block_id\":\"x6BGO\",\"label\":{\"type\":\"plain_text\",\"text\":\"Choose where to post this\",\"emoji\":true},\"optional\":false,\"dispatch_action\":false,\"element\":{\"type\":\"static_select\",\"action_id\":\"where_to_post.select\",\"initial_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel::C03TZV5RRF1\"},\"options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Ao Channel\",\"emoji\":true},\"value\":\"ao_channel\"},{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel::C03TZV5RRF1\"}]}},{\"type\":\"context\",\"block_id\":\"jC2Zn\",\"elements\":[{\"type\":\"mrkdwn\",\"text\":\"Please wait after hitting submit!\",\"verbatim\":false}]}],\"private_metadata\":\"\",\"callback_id\":\"\",\"state\":{\"values\":{\"lnMij\":{\"title.input\":{\"type\":\"plain_text_input\",\"value\":\"testing preblast\"}},\"\\/o5Bh\":{\"ao.select\":{\"type\":\"channels_select\",\"selected_channel\":\"C03UEBT1QRZ\"}},\"bbmMo\":{\"date.select\":{\"type\":\"datepicker\",\"selected_date\":\"2024-01-18\"}},\"YKm+V\":{\"time.select\":{\"type\":\"timepicker\",\"selected_time\":\"02:00\"}},\"zd6aQ\":{\"qs.select\":{\"type\":\"multi_users_select\",\"selected_users\":[\"U03T87KHRFE\"]}},\"x87Qz\":{\"why.input\":{\"type\":\"plain_text_input\",\"value\":\"testing\"}},\"Tfv25\":{\"equipment.select\":{\"type\":\"multi_static_select\",\"selected_options\":[{\"text\":{\"type\":\"plain_text\",\"text\":\"Coupons \\ud83e\\uddf1\",\"emoji\":true},\"value\":\"coupon\"}]}},\"oo5gq\":{\"other_equipment.input\":{\"type\":\"plain_text_input\",\"value\":null}},\"xVRyF\":{\"fngs.input\":{\"type\":\"plain_text_input\",\"value\":\"Always\"}},\"Y9MdG\":{\"moleskin.textbox\":{\"type\":\"plain_text_input\",\"value\":\"testing some things\"}},\"AwLSA\":{\"file.input\":{\"type\":\"file_input\",\"files\":[{\"id\":\"F06DX3RG1DM\",\"created\":1705512948,\"timestamp\":1705512948,\"name\":\"IMG_7940.HEIC\",\"title\":\"IMG_7940.HEIC\",\"mimetype\":\"image\\/heic\",\"filetype\":\"heic\",\"pretty_type\":\"High Efficiency Image File\",\"user\":\"U03T87KHRFE\",\"user_team\":\"T03T5J6801Z\",\"editable\":false,\"size\":1447396,\"mode\":\"hosted\",\"is_external\":false,\"external_type\":\"\",\"is_public\":false,\"public_url_shared\":false,\"display_as_bot\":false,\"username\":\"\",\"url_private\":\"https:\\/\\/files.slack.com\\/files-pri\\/T03T5J6801Z-F06DX3RG1DM\\/img_7940.heic\",\"url_private_download\":\"https:\\/\\/files.slack.com\\/files-pri\\/T03T5J6801Z-F06DX3RG1DM\\/download\\/img_7940.heic\",\"media_display_type\":\"unknown\",\"thumb_64\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_64.png\",\"thumb_80\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_80.png\",\"thumb_360\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_360.png\",\"thumb_360_w\":360,\"thumb_360_h\":270,\"thumb_480\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_480.png\",\"thumb_480_w\":480,\"thumb_480_h\":360,\"thumb_160\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_160.png\",\"thumb_720\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_720.png\",\"thumb_720_w\":720,\"thumb_720_h\":540,\"thumb_800\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_800.png\",\"thumb_800_w\":800,\"thumb_800_h\":600,\"thumb_960\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_960.png\",\"thumb_960_w\":960,\"thumb_960_h\":720,\"thumb_1024\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06DX3RG1DM-867d4297ab\\/img_7940_1024.png\",\"thumb_1024_w\":1024,\"thumb_1024_h\":768,\"original_w\":3827,\"original_h\":2870,\"permalink\":\"https:\\/\\/f3-boise.slack.com\\/files\\/U03T87KHRFE\\/F06DX3RG1DM\\/img_7940.heic\",\"permalink_public\":\"https:\\/\\/slack-files.com\\/T03T5J6801Z-F06DX3RG1DM-d1e8dd0d8b\",\"comments_count\":0,\"shares\":{},\"channels\":[],\"groups\":[],\"ims\":[],\"has_more_shares\":false,\"has_rich_preview\":false,\"file_access\":\"visible\"},{\"id\":\"F06EBKYGG4S\",\"created\":1705512957,\"timestamp\":1705512957,\"name\":\"Capture.JPG\",\"title\":\"Capture.JPG\",\"mimetype\":\"image\\/jpeg\",\"filetype\":\"jpg\",\"pretty_type\":\"JPEG\",\"user\":\"U03T87KHRFE\",\"user_team\":\"T03T5J6801Z\",\"editable\":false,\"size\":29968,\"mode\":\"hosted\",\"is_external\":false,\"external_type\":\"\",\"is_public\":false,\"public_url_shared\":false,\"display_as_bot\":false,\"username\":\"\",\"url_private\":\"https:\\/\\/files.slack.com\\/files-pri\\/T03T5J6801Z-F06EBKYGG4S\\/capture.jpg\",\"url_private_download\":\"https:\\/\\/files.slack.com\\/files-pri\\/T03T5J6801Z-F06EBKYGG4S\\/download\\/capture.jpg\",\"media_display_type\":\"unknown\",\"thumb_64\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06EBKYGG4S-7c85ade98b\\/capture_64.jpg\",\"thumb_80\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06EBKYGG4S-7c85ade98b\\/capture_80.jpg\",\"thumb_360\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06EBKYGG4S-7c85ade98b\\/capture_360.jpg\",\"thumb_360_w\":360,\"thumb_360_h\":358,\"thumb_160\":\"https:\\/\\/files.slack.com\\/files-tmb\\/T03T5J6801Z-F06EBKYGG4S-7c85ade98b\\/capture_160.jpg\",\"original_w\":463,\"original_h\":461,\"thumb_tiny\":\"AwAvADBjetRVI\\/QU0AEisoFSHAZSoytWAMDFMK1drC3IwM091G3NBIzgU9x8lSAjKTTQpDCpgKXAqYlNDQKRhwaHJVcgdKQncMityCIDLcVO4+SmbcHipXGUrF6lbDWcK3qaTcTUSc5Y9zTxWkYpCbBnIpsTrvCkYBpTzTcDNUxEjMyMR1welPWQOOOD6VD1qMkq+RUyQ0z\\/2Q==\",\"permalink\":\"https:\\/\\/f3-boise.slack.com\\/files\\/U03T87KHRFE\\/F06EBKYGG4S\\/capture.jpg\",\"permalink_public\":\"https:\\/\\/slack-files.com\\/T03T5J6801Z-F06EBKYGG4S-52fa0526a7\",\"comments_count\":0,\"shares\":{},\"channels\":[],\"groups\":[],\"ims\":[],\"has_more_shares\":false,\"has_rich_preview\":false,\"file_access\":\"visible\"}]}},\"x6BGO\":{\"where_to_post.select\":{\"type\":\"static_select\",\"selected_option\":{\"text\":{\"type\":\"plain_text\",\"text\":\"Current Channel\",\"emoji\":true},\"value\":\"current_channel::C03TZV5RRF1\"}}}}},\"hash\":\"1705512868.U36n81lw\",\"title\":{\"type\":\"plain_text\",\"text\":\"Pre Blast\",\"emoji\":true},\"clear_on_close\":false,\"notify_on_close\":false,\"close\":null,\"submit\":{\"type\":\"plain_text\",\"text\":\"Submit\",\"emoji\":true},\"previous_view_id\":null,\"root_view_id\":\"V06EBKNT2NN\",\"app_id\":\"A03UAGJC9QD\",\"external_id\":\"pre_blast::382a8\",\"app_installed_team_id\":\"T03T5J6801Z\",\"bot_id\":\"B03UG6KRSN8\"},\"response_urls\":[],\"is_enterprise_install\":false,\"enterprise\":null}";
        let parsed = serde_json::from_str::<ViewSubmissionPayload>(payload).unwrap();
        match parsed.view {
            ViewSubmissionPayloadView::Modal(modal) => {
                let values = modal.state.get_values();
                let files = values.get("file.input").unwrap();
                if let BasicValue::Multi(files) = files {
                    assert_eq!(files.len(), 2);
                } else {
                    assert_eq!(false, true);
                }
            }
        }
    }
}
