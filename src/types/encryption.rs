use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "ServerSideEncryptionConfiguration")]
pub struct BucketEncryption {
    #[serde(rename = "Rule")]
    pub rule: Rule,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "SSEAlgorithm")]
    pub sse: Option<String>,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_key: Option<String>,
}
