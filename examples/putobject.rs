extern crate s3;

use chrono::{offset::TimeZone, Utc};
use s3::{Client, PutObject};

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

    let resp = client
        .send(
            PutObject::new(
                "test",
                "putobject_example_file",
                "random bytes".as_bytes().to_vec(),
            )
            .expires(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)),
        )
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
