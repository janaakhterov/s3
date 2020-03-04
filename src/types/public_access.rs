use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicAccessBookConfiguration {
    block_public_acls: bool,
    block_public_policy: bool,
    ignore_public_acls: bool,
    restrict_public_buckets: bool,
}
