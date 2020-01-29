use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BucketCors {
    rules: Vec<CorsRule>,
}

#[derive(Debug, Deserialize)]
pub struct CorsRule {
    allowed_headers: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_origins: Vec<String>,
    exposed_headers: Vec<String>,
    max_age_seconds: u64,
}
