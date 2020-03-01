use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    // Currently quick-xml does not support unit variants
    // like the `Region` enum, so a work around is to use
    // `String`
    pub location_constraint: Option<String>,
}
