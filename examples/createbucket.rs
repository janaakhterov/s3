extern crate s3;

use s3::{
    client::Client,
    CreateBucket,
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

    let resp = client
        .send(CreateBucket::new("createbuckettest").location(Region::UsWest1))
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
