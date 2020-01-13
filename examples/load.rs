extern crate s3;

use chrono::{
    offset::TimeZone,
    Utc,
};
use s3::{
    client::Client,
    GetObject,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = Client::load("http://127.0.0.1:9000")?;

    let resp = client
        .send(
            GetObject::new("test", "putobject_example_file")
                .range(0u64, 100u64)
                .if_modified_since(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)),
        )
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
