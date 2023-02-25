use crate::slack_api::block_kit::TextObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockElementType {
    Button(ButtonElement),
    Checkboxes(CheckboxesBlock),
    Datepicker(DatePickerBlock),
    Image(ImageBlock),
    MultiStaticSelect(MultiStaticSelectBlock),
    MultiExternalSelect,
    MultiUsersSelect(MultiUsersSelectBlock),
    MultiConversationsSelect,
    MultiChannelsSelect(MultiChannelsSelectBlock),
    Overflow(OverflowElement),
    PlainTextInput(PlainTextInputBlock),
    RadioButtons(RadioButtonsBlock),
    StaticSelect(StaticSelectBlock),
    ExternalSelect,
    UsersSelect(UsersSelectBlock),
    ConversationsSelect,
    ChannelsSelect(ChannelsSelectBlock),
    Timepicker(TimepickerBlock),
}

impl Default for BlockElementType {
    fn default() -> Self {
        BlockElementType::new_plain_input("", None, None)
    }
}

impl BlockElementType {
    pub fn new_btn(text: &str, action_id: &str) -> Self {
        BlockElementType::Button(ButtonElement::new(text, action_id))
    }

    pub fn new_danger_btn(text: &str, action_id: &str) -> Self {
        BlockElementType::Button(ButtonElement::new_danger(text, action_id))
    }

    pub fn new_overflow(action_id: &str, options: Vec<OptionElement>) -> Self {
        BlockElementType::Overflow(OverflowElement::new(action_id, options))
    }

    pub fn new_plain_input(
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
    ) -> Self {
        BlockElementType::PlainTextInput(
            PlainTextInputBlock::new(action_id.to_string())
                .with_placeholder(placeholder)
                .with_initial_value(initial_value),
        )
    }

    pub fn new_text_box(
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
    ) -> Self {
        BlockElementType::PlainTextInput(
            PlainTextInputBlock::new(action_id.to_string())
                .with_placeholder(placeholder)
                .with_initial_value(initial_value)
                .with_multi_line(),
        )
    }

    pub fn new_channel_select(action_id: &str, initial_channel: Option<String>) -> Self {
        BlockElementType::ChannelsSelect(
            ChannelsSelectBlock::new(action_id.to_string()).with_initial(initial_channel),
        )
    }

    pub fn new_date_selector(action_id: &str, date: Option<String>) -> Self {
        BlockElementType::Datepicker(DatePickerBlock::new(action_id.to_string()).with_date(date))
    }

    pub fn new_time_picker(action_id: &str, time: Option<String>) -> Self {
        BlockElementType::Timepicker(TimepickerBlock::new(action_id.to_string()).with_time(time))
    }

    pub fn new_multi_user_select(action_id: &str, initial_users: Option<Vec<String>>) -> Self {
        BlockElementType::MultiUsersSelect(
            MultiUsersSelectBlock::new(action_id.to_string()).with_users(initial_users),
        )
    }

    pub fn new_select(
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<OptionElement>,
    ) -> Self {
        BlockElementType::StaticSelect(
            StaticSelectBlock::new(action_id.to_string(), options).with_default(default),
        )
    }

    pub fn new_select_with_groups(
        action_id: &str,
        option_groups: Vec<OptionGroupElement>,
        default: Option<OptionElement>,
    ) -> Self {
        BlockElementType::StaticSelect(
            StaticSelectBlock::with_groups(action_id.to_string(), option_groups)
                .with_default(default),
        )
    }

    pub fn new_multi_select(
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<Vec<OptionElement>>,
    ) -> Self {
        BlockElementType::MultiStaticSelect(
            MultiStaticSelectBlock::new(action_id.to_string(), options).with_default(default),
        )
    }

    pub fn new_multi_select_with_groups(
        action_id: &str,
        options: Vec<OptionGroupElement>,
        default: Option<Vec<OptionElement>>,
    ) -> Self {
        BlockElementType::MultiStaticSelect(
            MultiStaticSelectBlock::new_with_groups(action_id.to_string(), options)
                .with_default(default),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CheckboxesBlock {
    pub action_id: String,
    /// An array of option objects. A maximum of 10 options are allowed
    pub options: Vec<OptionElement>,
    /// An array of option objects that exactly matches one or more of the options within options. These options will be selected when the checkbox group initially loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<OptionElement>>,
    /// A confirm object that defines an optional confirmation dialog that appears after clicking one of the checkboxes in this element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TimepickerBlock {
    pub action_id: String,
    /// The initial time that is selected when the element is loaded.
    /// This should be in the format HH:mm, where HH is the 24-hour format of an hour (00 to 23) and mm is minutes with leading zeros (00 to 59),
    /// for example 22:25 for 10:25pm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
    /// A confirm object that defines an optional confirmation dialog that appears after a time is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the time picker. Maximum length for the text in this field is 150 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    /// A string in the IANA format, e.g. "America/Chicago".
    /// The timezone is displayed to end users as hint text underneath the time picker.
    /// It is also passed to the app upon certain inteactions, such as view_submission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl TimepickerBlock {
    pub fn new(action_id: String) -> Self {
        TimepickerBlock {
            action_id,
            ..Default::default()
        }
    }

    /// This should be in the format HH:mm, where HH is the 24-hour format of an hour (00 to 23) and mm is minutes with leading zeros (00 to 59),
    pub fn with_time(mut self, time: Option<String>) -> Self {
        self.initial_time = time;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RadioButtonsBlock {
    pub action_id: String,
    /// An array of option objects. A maximum of 10 options are allowed.
    pub options: Vec<OptionElement>,
    /// An option object that exactly matches one of the options within options. This option will be selected when the radio button group initially loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionElement>,
    /// A confirm object that defines an optional confirmation dialog that appears after clicking one of the checkboxes in this element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageBlock {
    /// The URL of the image to be displayed
    pub image_url: String,
    /// A plain-text summary of the image. This should not contain any markup
    pub alt_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DatePickerBlock {
    pub action_id: String,
    /// The initial date that is selected when the element is loaded. This should be in the format YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    /// A confirm object that defines an optional confirmation dialog that appears after a date is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl DatePickerBlock {
    pub fn new(action_id: String) -> Self {
        DatePickerBlock {
            action_id,
            ..Default::default()
        }
    }

    /// The initial date that is selected when the element is loaded. This should be in the format YYYY-MM-DD
    pub fn with_date(mut self, date: Option<String>) -> Self {
        self.initial_date = date;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfirmDialogConfig {
    /// A plain_text-only text object that defines the dialog's title. Maximum length for this field is 100 characters.
    pub title: TextObject,
    /// A text object that defines the explanatory text that appears in the confirm dialog. Maximum length for the text in this field is 300 characters.
    pub text: TextObject,
    /// A plain_text-only text object to define the text of the button that confirms the action. Maximum length for the text in this field is 30 characters.
    pub confirm: TextObject,
    /// A plain_text-only text object to define the text of the button that cancels the action. Maximum length for the text in this field is 30 characters.
    pub deny: TextObject,
    // Defines the color scheme applied to the confirm button.
    // A value of danger will display the button with a red background on desktop, or red text on mobile.
    // A value of primary will display the button with a green background on desktop, or blue text on mobile.
    // If this field is not provided, the default value will be primary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StaticSelectBlock {
    pub action_id: String,
    /// An array of option objects. Maximum number of options is 100. If option_groups is specified, this field should not be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OptionElement>>,
    /// An array of option group objects. Maximum number of option groups is 100. If options is specified, this field should not be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_groups: Option<Vec<OptionGroupElement>>,
    /// A single option that exactly matches one of the options within options or option_groups. This option will be selected when the menu initially loads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionElement>,
    /// A confirm object that defines an optional confirmation dialog that appears after a menu item is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl StaticSelectBlock {
    pub fn new(action_id: String, options: Vec<OptionElement>) -> Self {
        StaticSelectBlock {
            action_id,
            options: Some(options),
            ..Default::default()
        }
    }

    pub fn with_groups(action_id: String, option_groups: Vec<OptionGroupElement>) -> Self {
        StaticSelectBlock {
            action_id,
            option_groups: Some(option_groups),
            ..Default::default()
        }
    }

    pub fn with_default(mut self, option: Option<OptionElement>) -> Self {
        self.initial_option = option;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UsersSelectBlock {
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters.
    pub action_id: String,
    /// The user ID of any valid user to be pre-selected when the menu loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_user: Option<String>,
    /// A confirm object that defines an optional confirmation dialog that appears after a menu item is selected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChannelsSelectBlock {
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters
    pub action_id: String,
    /// The ID of any valid public channel to be pre-selected when the menu loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_channel: Option<String>,
    // A confirm object that defines an optional confirmation dialog that appears after a menu item is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// This field only works with menus in input blocks in modals.
    // When set to true, the view_submission payload from the menu's parent view will contain a response_url.
    // This response_url can be used for message responses.
    // The target channel for the message will be determined by the value of this select menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url_enabled: Option<bool>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl ChannelsSelectBlock {
    pub fn new(action_id: String) -> Self {
        ChannelsSelectBlock {
            action_id,
            ..Default::default()
        }
    }

    pub fn with_initial(mut self, initial_channel: Option<String>) -> Self {
        self.initial_channel = initial_channel;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultiStaticSelectBlock {
    pub action_id: String,
    /// An array of option objects. Maximum number of options is 100. If option_groups is specified, this field should not be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OptionElement>>,
    /// An array of option group objects. Maximum number of option groups is 100. If options is specified, this field should not be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_groups: Option<Vec<OptionGroupElement>>,
    /// An array of option objects that exactly match one or more of the options within options or option_groups.
    /// These options will be selected when the menu initially loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<OptionElement>>,
    /// A confirm object that defines an optional confirmation dialog that appears before the multi-select choices are submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Specifies the maximum number of items that can be selected in the menu. Minimum number is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u16>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl MultiStaticSelectBlock {
    pub fn new(action_id: String, options: Vec<OptionElement>) -> Self {
        MultiStaticSelectBlock {
            action_id,
            options: Some(options),
            ..Default::default()
        }
    }

    pub fn new_with_groups(action_id: String, options: Vec<OptionGroupElement>) -> Self {
        MultiStaticSelectBlock {
            action_id,
            option_groups: Some(options),
            ..Default::default()
        }
    }

    pub fn with_default(mut self, options: Option<Vec<OptionElement>>) -> Self {
        self.initial_options = options;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultiUsersSelectBlock {
    pub action_id: String,
    /// An array of user IDs of any valid users to be pre-selected when the menu loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_users: Option<Vec<String>>,
    /// A confirm object that defines an optional confirmation dialog that appears before the multi-select choices are submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Specifies the maximum number of items that can be selected in the menu. Minimum number is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u16>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl MultiUsersSelectBlock {
    pub fn new(action_id: String) -> Self {
        MultiUsersSelectBlock {
            action_id,
            ..Default::default()
        }
    }

    pub fn with_users(mut self, users: Option<Vec<String>>) -> Self {
        self.initial_users = users;
        self
    }

    pub fn with_placeholder(mut self, placeholder: Option<TextObject>) -> Self {
        self.placeholder = placeholder;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultiChannelsSelectBlock {
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block.
    /// Maximum length for this field is 255 characters
    pub action_id: String,
    /// An array of one or more IDs of any valid public channel to be pre-selected when the menu loads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_channels: Option<Vec<String>>,
    /// A confirm object that defines an optional confirmation dialog that appears before the multi-select choices are submitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmDialogConfig>,
    /// Specifies the maximum number of items that can be selected in the menu. Minimum number is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u16>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown on the menu. Maximum length for the text in this field is 150 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlainTextInputBlock {
    /// An identifier for the input value when the parent modal is submitted.
    /// You can use this when you receive a view_submission payload to identify the value of the input element.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters
    pub action_id: String,
    /// The initial value in the plain-text input when it is loaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    /// Indicates whether the input will be a single line (false) or a larger textarea (true). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    /// The minimum length of input that the user must provide.
    /// If the user provides less, they will receive an error. Maximum value is 3000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// The maximum length of input that the user can provide. If the user provides more, they will receive an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// TODO config object https://api.slack.com/reference/block-kit/composition-objects#dispatch_action_config
    /// A dispatch configuration object that determines when during text input the element returns a block_actions payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<bool>,
    /// Indicates whether the element will be set to auto focus within the view object. Only one element can be set to true. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    /// A plain_text only text object that defines the placeholder text shown in the plain-text input. Maximum length for the text in this field is 150 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl PlainTextInputBlock {
    pub fn new(action_id: String) -> Self {
        PlainTextInputBlock {
            action_id,
            ..Default::default()
        }
    }

    /// new input with placeholder
    pub fn with_placeholder(mut self, placeholder: Option<String>) -> Self {
        self.placeholder = placeholder.map(|text| TextObject::new_text(text.as_str()));
        self
    }

    pub fn with_initial_value(mut self, value: Option<String>) -> Self {
        self.initial_value = value;
        self
    }

    pub fn with_multi_line(mut self) -> Self {
        self.multiline = Some(true);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverflowElement {
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block.
    /// Maximum length for this field is 255 characters
    action_id: String,
    /// An array of up to five option objects to display in the menu
    options: Vec<OptionElement>,
}

impl OverflowElement {
    pub fn new(action_id: &str, options: Vec<OptionElement>) -> Self {
        OverflowElement {
            action_id: action_id.to_string(),
            options,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionElement {
    /// A text object that defines the text shown in the option on the menu.
    /// Overflow, select, and multi-select menus can only use plain_text objects,
    /// while radio buttons and checkboxes can use mrkdwn text objects.
    /// Maximum length for the text in this field is 75 characters
    pub text: TextObject,
    /// A unique string value that will be passed to your app when this option is chosen.
    /// Maximum length for this field is 75 characters
    pub value: String,
    /// A plain_text only text object that defines a line of descriptive text shown below the text field beside the radio button.
    /// Maximum length for the text object within this field is 75 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextObject>,
    /// A URL to load in the user's browser when the option is clicked.
    /// The url attribute is only available in overflow menus. Maximum length for this field is 3000 characters.
    /// If you're using url, you'll still receive an interaction payload and will need to send an acknowledgement response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl OptionElement {
    pub fn new(text: &str, value: &str) -> Self {
        OptionElement {
            text: TextObject::new_text(text),
            value: value.to_string(),
            description: None,
            url: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionGroupElement {
    /// A plain_text only text object that defines the label shown above this group of options. Maximum length for the text in this field is 75 characters.
    pub label: TextObject,
    /// An array of option objects that belong to this specific group. Maximum of 100 items
    pub options: Vec<OptionElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonElement {
    /// can only be plain_text
    text: TextObject,
    /// An identifier for this action. You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters
    action_id: String,
    /// either 'primary' or 'danger'. uses default if none passed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

pub enum ButtonStyle {
    Primary,
    Danger,
}

const BTN_PRIMARY_STYLE: &str = "primary";
const BTN_DANGER_STYLE: &str = "danger";

impl ToString for ButtonStyle {
    fn to_string(&self) -> String {
        match self {
            ButtonStyle::Danger => BTN_DANGER_STYLE.to_string(),
            ButtonStyle::Primary => BTN_PRIMARY_STYLE.to_string(),
        }
    }
}

impl ButtonElement {
    /// new button with default style
    pub fn new(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: None,
        }
    }

    /// new button with danger style
    pub fn new_danger(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: Some(ButtonStyle::Danger.to_string()),
        }
    }

    /// new button with primary style
    pub fn new_primary(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: Some(ButtonStyle::Primary.to_string()),
        }
    }
}
