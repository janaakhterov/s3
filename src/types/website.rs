use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketWebsiteOutput")]
#[serde(rename_all = "PascalCase")]
pub struct BucketWebsite {
    error_document: ErrorDocument,
    index_document: IndexDocument,
    redirect_all_requests_to: RedirectAllRequestsTo,
    #[serde(rename = "RoutingRule")]
    routing_rules: Vec<RoutingRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorDocument {
    key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexDocument {
    suffix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RedirectAllRequestsTo {
    host_name: String,
    protocol: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Condition {
    http_error_code_returned_equals: Option<String>,
    key_prefix_equals: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Redirect {
    host_name: Option<String>,
    http_redirect_code: Option<String>,
    protocol: Option<String>,
    replace_key_prefix_with: Option<String>,
    replace_key_with: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRule {
    condition: Option<Condition>,
    redirect: Redirect,
}
