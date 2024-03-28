pub trait ProcessedType {
    /// unique string to identify your type id of process items
    fn get_type_id(&self) -> Option<String>;
}

pub trait ResolvingProcessedItems {
    fn get_processed_items(&self) -> Vec<NewProcessItem>;
}

pub struct NewProcessItem {
    pub item_type: String,
    pub item_ids: Vec<String>,
}

impl NewProcessItem {
    pub fn new(item_type: &str, item_ids: Vec<String>) -> Self {
        NewProcessItem {
            item_type: item_type.to_string(),
            item_ids,
        }
    }
}
