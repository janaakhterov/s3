use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
/// All the possible AWS regions
pub enum Region {
    Eu,
    EuWest1,
    UsEast1,
    UsWest1,
    UsWest2,
    ApSouth1,
    ApSouthEast1,
    ApSouthEast2,
    ApNorthEast1,
    SaEast1,
    CnNorth1,
    EuCentral1,
}

impl Into<String> for Region {
    fn into(self) -> String {
        match self {
            Region::Eu => "EU",
            Region::EuWest1 => "eu-west-1",
            Region::UsEast1 => "us-east-1",
            Region::UsWest1 => "us-west-1",
            Region::UsWest2 => "us-west-2",
            Region::ApSouth1 => "ap-south-1",
            Region::ApSouthEast1 => "ap-southeast-1",
            Region::ApSouthEast2 => "ap-southeast-2",
            Region::ApNorthEast1 => "ap-northeast-1",
            Region::SaEast1 => "sa-east-1",
            Region::CnNorth1 => "cn-north-1",
            Region::EuCentral1 => "eu-central-1",
        }
        .to_owned()
    }
}
