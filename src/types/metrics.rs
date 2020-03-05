use super::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "MetricsConfiguration")]
pub struct BucketMetrics {
    #[serde(rename = "ID")]
    id: String,
    filter: MetricsFilter,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricsFilter {
    and: Option<MetricsAndOperator>,
    prefix: Option<String>,
    tag: Option<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricsAndOperator {
    prefix: Option<String>,

    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ListBucketMetricsConfigurationsOutput")]
pub struct BucketMetricsConfigs {
    metrics_configuration: Vec<BucketMetrics>,
    continuation_token: String,
    is_truncated: bool,
    next_continuation_token: String,
}
