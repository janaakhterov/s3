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

pub enum AwsEncryption<'a> {
    Sse,
    Kms(&'a str),
}

impl<'a> Into<&'static str> for AwsEncryption<'a> {
    fn into(self) -> &'static str {
        match self {
            AwsEncryption::Sse => "AES256",
            AwsEncryption::Kms(_) => "aws:kms",
        }
    }
}
