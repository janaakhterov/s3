extern crate s3;

use s3::{
    Client,
    DeleteObject,
};

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
        .send(DeleteObject::new("test", "putobject_example_file"))
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
