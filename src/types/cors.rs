use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketCorsOutput")]
pub struct BucketCors {
    #[serde(rename = "CORSRule")]
    rules: Vec<CorsRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CorsRule {
    allowed_headers: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_origins: Vec<String>,
    exposed_headers: Vec<String>,
    max_age_seconds: Option<u64>,
}
