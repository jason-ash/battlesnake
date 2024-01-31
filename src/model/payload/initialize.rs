#[derive(Debug, serde::Serialize)]
pub struct InitializePayload {
    api_version: String,
    author: String,
    color: String,
    head: String,
    tail: String,
    version: String,
}

impl InitializePayload {
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    pub fn with_head(mut self, head: &str) -> Self {
        self.head = head.to_string();
        self
    }

    pub fn with_tail(mut self, tail: &str) -> Self {
        self.tail = tail.to_string();
        self
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }
}

impl Default for InitializePayload {
    fn default() -> Self {
        Self {
            api_version: String::from("1"),
            author: String::from("jason-ash"),
            color: String::from("#581c87"),
            head: String::from("default"),
            tail: String::from("default"),
            version: String::from("0.1.0"),
        }
    }
}
