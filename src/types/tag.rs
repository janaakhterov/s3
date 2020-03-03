use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketTaggingOutput")]
#[serde(rename_all = "PascalCase")]
pub struct BucketTagging {
    tag_set: TagSet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagSet {
    #[serde(rename = "Tag")]
    pub tags: Vec<Tag>,
}
