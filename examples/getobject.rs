extern crate s3;

use s3::{Client, GetObject};
use chrono::{offset::TimeZone, Utc};

static SECRET_ACCESS_KEY: &'static str = "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG";
static ACCESS_KEY: &'static str = "6KSUI28SEVTXB63GLSLU";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = Client::builder()
        .host("http://127.0.0.1:9000")
        .access_key(&ACCESS_KEY)
        .secret_key(&SECRET_ACCESS_KEY)
        .build()?;

    let resp = client.send(GetObject::new("test", "vimrc")
        .range(0u64, 100u64)
        .if_modified_since(Utc.ymd(2019, 12, 25).and_hms(0, 0, 0))).await?;

    println!("{:#?}", resp);

    Ok(())
}
