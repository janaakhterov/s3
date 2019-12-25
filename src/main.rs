use reqwest::{Request, Client};
use chrono::{offset::TimeZone, Utc};

use crate::{signing_key::SigningKey, get_object::GetObject, region::Region};

pub mod client;
pub mod region;
pub mod signing_key;
pub mod string_to_sign;
pub mod buf_mut;
pub mod auth_builder;

pub mod get_object;

static SECRET_ACCESS_KEY: &'static str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
static ACCESS_KEY: &'static str = "AKIAIOSFODNN7EXAMPLE";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let client = Client::new();

    let request = client.get("https://examplebucket.s3.amazonaws.com")
        .build();

    println!("{:#?}", request);

    // request.sign(&client);
    let date = Utc.ymd(2013, 05, 24).and_hms(0, 0, 0);
    let region = Region::UsEast1;
    let key = SigningKey::from_date(&SECRET_ACCESS_KEY, &date.clone(), region.clone());
    let url = reqwest::Url::parse("https://s3.amazonaws.com")?;

    let s: Request = GetObject::new("examplebucket", "test.txt")
        .range(0, 9)
        .into_request(url, &ACCESS_KEY, &key, region)
        .unwrap();


    Ok(())
}

// fn sign_request(request: &mut reqwest::Request, key: SigningKey): {
//     let headers = request.headers_mut();
// }

// SigningKey:
// SigningKey {
//     key: Key {
//         algorithm: SHA256,
//     },
// }
// ------------------------------------------------------------------------
// CanonicalRequest:
// GET
// /test.txt

// host:examplebucket.s3.amazonaws.com
// range:bytes=0-9
// x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
// x-amz-date:20130524T000000Z

// host;range;x-amz-content-sha256;x-amz-date
// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
// ------------------------------------------------------------------------
// StringToSign:
// AWS4-HMAC-SHA256
// 20130524T000000Z
// 20130524/us-east-1/s3/aws4_request
// 7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972
// ------------------------------------------------------------------------
// Signature:
// f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41
// ------------------------------------------------------------------------
