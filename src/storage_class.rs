use crate::error;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
/// Provides storage class information of the object.
/// Default storage class is `STANDARD`
pub enum StorageClass {
    Standard,
    ReducedRedundancy,
    StandardIa,
    OnezoneIa,
    IntelligentTiering,
    Glacier,
    DeepArchive,
}

impl FromStr for StorageClass {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STANDARD" => Ok(Self::Standard),
            "REDUCED_REDUNDANCY" => Ok(Self::ReducedRedundancy),
            "STANDARD_IA" => Ok(Self::StandardIa),
            "ONEZONE_IA" => Ok(Self::OnezoneIa),
            "INTELLIGENT_TIERING" => Ok(Self::IntelligentTiering),
            "DEEP_ARCHIVE" => Ok(Self::DeepArchive),
            _ => Err(error::Error::from(error::Internal::ParseStorageClassError)),
        }
    }
}
