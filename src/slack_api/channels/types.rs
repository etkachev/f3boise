#[derive(PartialEq, Eq, Hash)]
pub enum ChannelTypes {
    Public,
    Private,
    MPIM,
    IM,
}

impl ChannelTypes {
    pub fn name(&self) -> String {
        match self {
            ChannelTypes::Public => String::from("public_channel"),
            ChannelTypes::Private => String::from("private_channel"),
            ChannelTypes::MPIM => String::from("mpim"),
            ChannelTypes::IM => String::from("im"),
        }
    }
}
