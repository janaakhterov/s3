use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectLockConfiguration {
    object_lock_enabled: String,
    rule: ObjectLockRule,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectLockRule {
    default_retention: Option<DefaultRetention>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DefaultRetention {
    days: Option<i32>,
    mode: Option<String>,
    years: Option<i32>,
}
