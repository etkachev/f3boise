/// Different AO options.
#[derive(PartialEq, Debug)]
pub enum AO {
    Bleach,
    Gem,
    OldGlory,
    Rebel,
    IronMountain,
    Ruckership,
    Backyard,
    DR,
    Unknown(String),
}

impl From<String> for AO {
    fn from(ao: String) -> Self {
        let ao = ao.trim().to_lowercase();
        let split_text: Vec<&str> = ao.splitn(2, '#').collect();
        let cleaned_ao = if split_text.len() == 2 {
            split_text[1].to_lowercase()
        } else {
            ao.to_lowercase()
        };
        match cleaned_ao.as_str() {
            "bleach" => AO::Bleach,
            "gem" => AO::Gem,
            "old-glory" | "oldglory" => AO::OldGlory,
            "rebel" => AO::Rebel,
            "iron-mountain" | "ironmountain" => AO::IronMountain,
            "ruckership" | "rucker-ship" => AO::Ruckership,
            "backyard" => AO::Backyard,
            "dr" => AO::DR,
            _ => AO::Unknown(ao.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bleach_test() {
        let ao = AO::from("#bleach ".to_string());
        assert_eq!(ao, AO::Bleach);
        let ao = AO::from("#bleach\n".to_string());
        assert_eq!(ao, AO::Bleach);
    }
}
