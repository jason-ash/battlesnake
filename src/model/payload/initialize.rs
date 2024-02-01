#[derive(Debug, serde::Serialize)]
pub struct InitializePayload {
    #[serde(rename = "apiversion")]
    api_version: String,
    author: String,
    #[serde(flatten)]
    customizations: Customizations,
    version: String,
}

impl InitializePayload {
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
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
            version: String::from("0.1.0"),
            customizations: Customizations::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Customizations {
    pub color: String,
    pub head: String,
    pub tail: String,
}

impl Default for Customizations {
    fn default() -> Self {
        Self {
            color: String::from("#581c87"),
            head: String::from("default"),
            tail: String::from("default"),
        }
    }
}
