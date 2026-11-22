#[derive(Debug, Clone)]
pub struct Item {
    label: String,
    message: Option<String>,
}

impl Item {
    pub fn new(label: String, message: Option<String>) -> Self {
        Self {
            label: label,
            message: message,
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }

    pub fn summary(&self) -> String {
        format!("{}{}", self.formatted_label(), self.formatted_message())
    }

    fn formatted_label(&self) -> String {
        format!("Adding \"{}\"", self.get_label())
    }
    fn formatted_message(&self) -> String {
        format!("\n  └─ note: {:?}", self.get_message().unwrap_or_default())
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            label: "<unlabeled>".to_string(),
            message: None,
        }
    }
}
