pub use crate::signing_key::SigningKey;
pub use crate::headers::Headers;
pub use crate::region::Region;
pub use crate::buf_mut::BufMut;
pub use crate::auth_builder::AuthBuilder;
pub use crate::amz_request::AmzRequest;
pub use crate::error::Error;

#[macro_use]
extern crate anyhow;

mod client;
mod region;
mod signing_key;
mod buf_mut;
mod auth_builder;
mod headers;
mod amz_request;
mod error;

mod get_object;

static TEST_SECRET_ACCESS_KEY: &'static str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
static TEST_ACCESS_KEY: &'static str = "AKIAIOSFODNN7EXAMPLE";
static SECRET_ACCESS_KEY: &'static str = "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";
static ACCESS_KEY: &'static str = "6KSUI28SEVTXB63GLSLU";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let client = client::Client::builder()
        .host("http://127.0.0.1:9000")
        .access_key(&ACCESS_KEY)
        .secret_key(&SECRET_ACCESS_KEY)
        .build()?;

    let bytes = client.get_object("test", ".vimrc").await?;

    println!("{}", String::from_utf8_lossy(&bytes));

    Ok(())
}
