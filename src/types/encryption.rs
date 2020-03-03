use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "ServerSideEncryptionConfiguration")]
pub struct BucketEncryption {
    pub server_side_encryption_rule: Vec<ServerSideEncryptionRule>,
}

// #[derive(Default, Debug, Deserialize, Serialize)]
// pub struct EncryptionRule {
//     #[serde(rename = "SSEAlgorithm")]
//     pub sse: Option<String>,
//     #[serde(rename = "KMSMasterKeyID")]
//     pub kms_key: Option<String>,
// }

// pub enum AwsEncryption<'a> {
//     Sse,
//     Kms(&'a str),
// }

// impl<'a> Into<&'static str> for AwsEncryption<'a> {
//     fn into(self) -> &'static str {
//         match self {
//             AwsEncryption::Sse => "AES256",
//             AwsEncryption::Kms(_) => "aws:kms",
//         }
//     }
// }

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionRule {
    pub server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionByDefault {
    pub kms_master_key_id: Option<String>,
    pub sse_algorithm: String,
}
