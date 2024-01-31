#[derive(Debug, serde::Serialize)]
pub struct InitializePayload {
    api_version: String,
    author: Option<String>,
    color: Option<String>,
    head: Option<String>,
    tail: Option<String>,
    version: Option<String>,
}

impl InitializePayload {
    pub fn new(
        author: Option<String>,
        color: Option<String>,
        head: Option<String>,
        tail: Option<String>,
        version: Option<String>,
    ) -> Self {
        Self {
            api_version: String::from("1"),
            author,
            color,
            head,
            tail,
            version,
        }
    }
}
