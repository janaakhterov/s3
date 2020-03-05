extern crate s3;

use chrono::{
    offset::TimeZone,
    Utc,
};
use s3::{
    client::Client,
    AwsRequest,
    CreateBucket,
    DeleteBucket,
    DeleteObject,
    GetObject,
    PutObject,
    Region,
};

static SECRET_ACCESS_KEY: &'static str = "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG";
static ACCESS_KEY: &'static str = "6KSUI28SEVTXB63GLSLU";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = Client::builder()
        .host("http://localhost:9000")
        .access_key(&ACCESS_KEY)
        .secret_key(&SECRET_ACCESS_KEY)
        .build()?;

    let resp = CreateBucket::new("createbuckettest")
        .location(Region::UsWest1)
        .send(&client)
        .await?;

    println!("CreateBucket: {:#?}", resp);

    let resp = client.list_buckets().await?;

    println!("ListBuckets: {:#?}", resp);

    let resp = PutObject::new(
        "createbuckettest",
        "putobject_example_file",
        "random bytes".as_bytes().to_vec(),
    )
    .expires(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0))
    .grant_read_email("example@gmail.com")
    .send(&client)
    .await?;

    println!("PutObject: {:#?}", resp);

    let resp = GetObject::new("createbuckettest", "putobject_example_file")
        .range(0u64, 100u64)
        .if_modified_since(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0))
        .send(&client)
        .await?;

    println!("GetObject: {:#?}", resp);

    let resp = DeleteObject::new("createbuckettest", "putobject_example_file")
        .send(&client)
        .await?;

    println!("DeleteObject: {:#?}", resp);

    let resp = DeleteBucket::new("createbuckettest").send(&client).await?;

    println!("DeleteBucket: {:#?}", resp);

    let resp = client.list_buckets().await?;

    println!("ListBuckets: {:#?}", resp);

    Ok(())
}
