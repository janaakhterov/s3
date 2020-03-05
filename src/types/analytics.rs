use super::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucaetAnalyticsConfig")]
#[serde(rename_all = "PascalCase")]
pub struct BucketAnalytics {
    #[serde(rename = "ID")]
    id: String,

    filter: AnalyticsFilter,
    storage_class_analysis: StorageClassAnalysis,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageClassAnalysis {
    data_export: Option<StorageClassAnalysisDataExport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageClassAnalysisDataExport {
    destination: AnalyticsExportDestination,
    output_schema_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsFilter {
    and: Option<AndOperator>,
    prefix: Option<String>,
    tag: Option<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AndOperator {
    prefix: Option<String>,

    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageClassAnalytics {
    data_export: Option<StorageClassAnalyticsDataExport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageClassAnalyticsDataExport {
    destination: AnalyticsExportDestination,
    output_schema_version: String,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsExportDestination {
    #[serde(rename = "S3BucketDestination")]
    bucket_destination: AnalyticsS3BucketDestination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsS3BucketDestination {
    bucket: String,
    bucket_account_id: Option<String>,
    format: String,
    prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListBucketAnalyticsConfiguration")]
#[serde(rename_all = "PascalCase")]
pub struct BucketAnalyticsConfiguration {
    analytics_configuration: Vec<AnalyticsConfiguration>,
    continuation_token: String,
    is_truncated: bool,
    next_continuation_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsConfiguration {
    filter: Option<AnalyticsFilter>,
    id: String,
    storage_class_analysis: StorageClassAnalysis,
}
