use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    is_public: bool,
}
