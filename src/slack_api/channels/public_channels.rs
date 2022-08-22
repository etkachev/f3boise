#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PublicChannels {
    BotPlayground,
    Unknown(String),
}

impl From<String> for PublicChannels {
    fn from(name: String) -> Self {
        match name.as_str() {
            "bot-playground" => PublicChannels::BotPlayground,
            _ => PublicChannels::Unknown(name),
        }
    }
}
