use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationConfiguration {
    topic_config: Vec<TopicConfiguration>,
    queue_config: Vec<QueueConfiguration>,
    cloud_function_config: Vec<CloudFunctionConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct TopicConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    arn: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationConfigFilter {
    key: Option<S3KeyFilter>,
}

#[derive(Debug, Deserialize)]
pub struct S3KeyFilter {
    rules: Vec<FilterRule>,
}

#[derive(Debug, Deserialize)]
pub struct FilterRule {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct QueueConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    arn: String,
}

#[derive(Debug, Deserialize)]
pub struct CloudFunctionConfiguration {
    events: Vec<String>,
    filter: Option<NotificationConfigFilter>,
    id: Option<String>,
    arn: String,
}
