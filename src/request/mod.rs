macro_rules! impl_sub_resource {
    ($name: ident => $output: ty) => {
        use crate::{
            error,
            AwsRequest,
            Error,
            QueryParameter,
            Region,
            SigningKey,
            SubResource,
        };
        use futures_core::future::BoxFuture;
        use hyper::{
            Body as HttpBody,
            Method,
            Request,
            Response,
        };
        use url::Url;

        pub struct $name<'a>(SubResource<'a>);

        impl<'a> AwsRequest for $name<'a> {
            type Response = $output;

            fn into_request<AR: AsRef<str>>(
                self,
                url: Url,
                access_key: AR,
                signing_key: &SigningKey,
                region: Region,
            ) -> Result<Request<HttpBody>, Error> {
                self.0.into_request(url, access_key, signing_key, region)
            }

            fn into_response(
                response: Response<HttpBody>,
            ) -> BoxFuture<'static, Result<Self::Response, Error>> {
                Box::pin(async move {
                    let bytes = SubResource::<'a>::into_response(response).await?;
                    let string = String::from_utf8_lossy(&bytes);

                    let resp: $output =
                        quick_xml::de::from_str(&string).map_err(error::Internal::from)?;

                    Ok(resp)
                })
            }
        }
    };
}

pub mod create_bucket;
pub mod list_buckets;
// pub mod put_bucket_encryption;
pub mod put_object;

pub mod delete;
pub mod get;

pub(crate) mod sub_resource;

pub use create_bucket::*;
pub use list_buckets::*;
// pub use put_bucket_encryption::*;
pub use put_object::*;

pub use delete::*;
pub use get::*;
