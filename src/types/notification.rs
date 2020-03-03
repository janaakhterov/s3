use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationConfiguration {
    cloud_function_config: Vec<LamdaFunctionConfiguration>,
    queue_config: Vec<QueueConfiguration>,
    topic_config: Vec<TopicConfiguration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TopicConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    topic_arn: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    queue_arn: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LamdaFunctionConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    lambda_function_arn: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationConfigFilter {
    key: Option<S3KeyFilter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct S3KeyFilter {
    filter_rules: Vec<FilterRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilterRule {
    name: Option<String>,
    value: Option<String>,
}
