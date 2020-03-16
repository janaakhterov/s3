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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = Client::load("http://s3.amazonaws.com/")?;

    // let resp = CreateBucket::new("cadims")
    //     .location(Region::UsWest1)
    //     .send(&client)
    //     .await?;

    // println!("CreateBucket: {:#?}", resp);

    // let resp = client.list_buckets().await?;

    // println!("ListBuckets: {:#?}", resp);

    let resp = PutObject::new(
        "cadims",
        "putobject_example_file",
        "random bytes".as_bytes().to_vec(),
    )
    .expires(Utc.ymd(2020, 3, 6).and_hms(0, 0, 0))
    // .grant_read_email("example@gmail.com")
    .send(&client)
    .await?;

    println!("PutObject: {:#?}", resp);

    // let resp = GetObject::new("cadims", "vimrc")
    //     .range(0u64, 10u64)
    //     .if_modified_since(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0))
    //     .send(&client)
    //     .await?;

    // println!("GetObject: {:#?}", resp);

    // let resp = DeleteObject::new("cadims", "putobject_example_file")
    //     .send(&client)
    //     .await?;

    // println!("DeleteObject: {:#?}", resp);

    // let resp = DeleteBucket::new("cadims").send(&client).await?;

    // println!("DeleteBucket: {:#?}", resp);

    // let resp = client.list_buckets().await?;

    // println!("ListBuckets: {:#?}", resp);

    Ok(())
}
