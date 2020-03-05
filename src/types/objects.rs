use super::{
    CommonPrefix,
    Owner,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "ListObjectsV2Output")]
#[serde(rename_all = "PascalCase")]
pub struct Objects {
    common_prefixes: Vec<CommonPrefix>,
    contents: Vec<Object>,
    continuation_token: String,
    delimiter: String,
    encoding_type: String,
    is_truncated: bool,
    key_count: u32,
    max_keys: i32,
    name: String,
    next_continuation_token: String,
    prefix: String,
    start_after: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListObjectsResponse")]
#[serde(rename_all = "PascalCase")]
pub struct Object {
    etag: Option<String>,
    key: Option<String>,
    last_modified: Option<String>,
    owner: Option<Owner>,
    size: Option<u32>,
    storage_class: Option<String>,
}
