use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    // Currently quick-xml does not support unit variants
    // like the `Region` enum, so a work around is to use
    // `String`
    pub location_constraint: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: Option<String>,

    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketsResponse {
    pub owner: Owner,
    pub buckets: Buckets,
}

#[derive(Debug, Deserialize)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub creation_date: DateTime<Utc>,
    pub name: String,
}
