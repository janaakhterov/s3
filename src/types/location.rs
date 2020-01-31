use crate::region::Region;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketLocationOutput")]
pub struct BucketLocation {
    location_constraint: Option<Region>,
}
