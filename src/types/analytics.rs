use super::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BucketAnalytics {
    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "Filter")]
    list: AnalyticsFilter,
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
    output_schema_version: OutputSchemaVersion,
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
    format: Format,
    prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Format {
    #[serde(rename = "CSV")]
    Csv,
}

#[derive(Debug, Deserialize)]
pub enum OutputSchemaVersion {
    #[serde(rename = "V_1")]
    V1,
}
