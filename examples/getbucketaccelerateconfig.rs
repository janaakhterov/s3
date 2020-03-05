extern crate s3;

use s3::{
    Client,
    GetBucketAccelerateConfig,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = Client::load("http://s3.amazonaws.com/")?;

    let resp = client
        .send(GetBucketAccelerateConfig::new("cadims"))
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
