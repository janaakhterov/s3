use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Retention {
    mode: String,
    retain_until_date: String,
}
