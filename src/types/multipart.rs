use super::Owner;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "ListMultipartUploadsOutput")]
#[serde(rename_all = "PascalCase")]
pub struct MultipartUploads {
    bucket: String,
    common_prefixes: Vec<CommonPrefix>,
    delimiter: String,
    encoding_type: String,
    is_truncated: bool,
    key_marker: String,
    next_key_marker: String,
    next_upload_id_marker: String,
    prefix: String,
    upload: Vec<MultipartUpload>,
    upload_id_marker: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefix {
    prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultipartUpload {
    initiated: Option<String>,
    initiator: Option<Initiator>,
    key: Option<String>,
    owner: Option<Owner>,
    storage_class: Option<String>,
    upload_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Initiator {
    #[serde(rename = "ID")]
    id: Option<String>,

    display_name: Option<String>,
}
