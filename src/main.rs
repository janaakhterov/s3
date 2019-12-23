use serde::Serialize;
use sha2::{Sha256, Digest};
use chrono::{offset::TimeZone, NaiveDateTime, DateTime, Utc};
use ring::{digest, hmac::{self, Key, Tag, Context}, rand};

#[derive(Debug, Serialize)]
struct MinioLogin {
    username: String,
    password: String,
}

static SECRET_ACCESS_KEY: &'static str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let client: Client = Client::new(&SECRET_ACCESS_KEY, "s3.amazonaws.com".to_owned(), Utc.ymd(2013, 5, 24).and_hms(0, 0, 0), Region::UsEast1);

    println!("{:#?}", client);
    println!("------------------------------------------------------------------------");

    // let client = reqwest::Client::new();

    // let crediential = format!("{access_key}/{date}/{region}/{service}/{request}", 
    //     // access_key = std::env::var("MINIO_ACCESS_KEY").expect("env MINIO_ACCESS_KEY not defined"),
    //     access_key = "AKIAIOSFODNN7EXAMPLE",
    //     // access_key = "minio",
    //     date = "20130524",
    //     region = "us-east-1",
    //     service = "s3",
    //     request = "aws4_request"
    // );

    // println!("{:#?}", crediential);

    // let signed_headers = "date".to_owned();

    // let date: String = format!("{}", DateTime::<Utc>::from(std::time::SystemTime::now()));
    let mut request = client.request(Method::Get, "examplebucket")
        .set_resource("/test.txt".to_owned())
        .add_header("range".to_owned(), "bytes=0-9".to_owned());

    request.sign(&client);

    // let signature = signing_key.sign(string);

    // println!("Signature: ");
    // println!("{}", signature);
    // println!("------------------------------------------------------------------------");

    // mac.input(date.as_bytes());

    // let sig = hex::encode(mac.result().code().as_slice());

    // println!("{:?}", sig);
    // println!("{:?}", sig.len());

    // let auth = format!("{encryption} Credential={crediential},SignedHeaders={signed_headers},Signature={sig}", 
    //     encryption = "AWS4-HMAC-SHA256",
    //     crediential = crediential,
    //     signed_headers = signed_headers,
    //     sig = sig
    // );

    // let request = client.get("http://127.0.0.1:9000/")
    //     .header("Authorization", auth)
    //     // .header("Credential", crediential)
    //     .header("Date", date)
    //     .header("x-amz-content-sha256", "UNSIGNED-PAYLOAD");
    //     // .header("SignedHeaders", signed_headers)
    //     // .header("Signature", sig);

    // println!("{:#?}", request);

    // let resp = request.send().await?;

    // println!("{:#?}", resp);

    Ok(())
}

enum Method {
    Get,
}

impl Into<String> for Method {
    fn into(self) -> String {
        match self {
            Method::Get => "GET".to_owned()
        }
    }
}

struct CanonicalRequest {
    method: Method,
    uri: String,
    query_params: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    payload: String,
}

impl CanonicalRequest {
    fn new(method: Method) -> Self {
        CanonicalRequest {
            method,
            uri: String::new(),
            query_params: Vec::new(),
            headers: Vec::new(),
            payload: String::new(),
        }
    }

    fn add_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.push((key, value));
        self
    }

    fn add_header(mut self, key: String, value: String) -> Self {
        self.headers.push((key, value));
        self
    }

    fn set_resource(mut self, resource: String) -> Self {
        self.uri = resource;
        self
    }

    fn sign(self, client: &Client) {
        let request: String = self.into();

        println!("CanonicalRequest: ");
        println!("{}", request);
        println!("------------------------------------------------------------------------");

        let string_to_sign: String = StringToSign::new(&client.date, client.region, request)
            .into();

        println!("StringToSign: ");
        println!("{}", string_to_sign);
        println!("------------------------------------------------------------------------");

        let signature = client.key.sign(string_to_sign);

        println!("Signature: ");
        println!("{}", signature);
        println!("------------------------------------------------------------------------");
    }
}

impl Into<String> for CanonicalRequest {
    fn into(mut self) -> String {
        let mut hasher = Sha256::new();
        hasher.input(self.payload.as_bytes());
        let hashed_payload: String = hex::encode(hasher.result().as_slice());
        self = self.add_header("x-amz-content-sha256".to_owned(), hashed_payload.clone());

        let method: String = self.method.into();
        let uri = self.uri;

        self.query_params
            .sort_by(|a, b| a.0.cmp(&b.0));
        let query_params: String = self.query_params
            .iter()
            .map(|param| 
                format!("{param}={value}",
                    param = param.0.to_lowercase(), 
                    value = param.1))
            .collect::<Vec<String>>()
            .join("&");

        self.headers
            .sort_by(|a, b| a.0.cmp(&b.0));
        let headers: String = self.headers
            .iter()
            .map(|header| 
                format!("{header}:{value}\n", 
                    header = header.0.to_lowercase(), 
                    value = header.1)
            )
            .collect::<Vec<String>>()
            .join("");

        let signed_headers = self.headers
            .iter()
            .map(|header| header.0.to_lowercase())
            .collect::<Vec<String>>()
            .join(";");

        return format!("{method}\n{uri}\n{query_params}\n{headers}\n{signed_headers}\n{payload}",
            method = method,
            uri = uri,
            query_params = query_params,
            headers = headers,
            signed_headers = signed_headers,
            payload = hashed_payload);
    }
}

#[derive(Debug)]
struct StringToSign {
    timestamp: String,
    scope: String,
    request: String,
}

impl StringToSign {
    fn new(date: &DateTime<Utc>, region: Region, request: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(request);

        let timestamp = format!("{}", date.format("%Y%m%dT%H%M%SZ"));
        let date = format!("{}", date.format("%Y%m%d"));
        let region: String = region.into();
        let scope = format!("{}/{}/s3/aws4_request", date, region);

        StringToSign {
            timestamp,
            scope,
            request: hex::encode(hasher.result().as_slice()),
        }
    }
}

impl Into<String> for StringToSign {
    fn into(self) -> String {
        format!("{algorithm}\n{timestamp}\n{scope}\n{request}",
            algorithm = "AWS4-HMAC-SHA256",
            timestamp = self.timestamp,
            scope = self.scope,
            request = self.request)
    }
}

#[derive(Debug, Copy, Clone)]
enum Region {
    UsEast1,
}

impl Into<String> for Region {
    fn into(self) -> String {
        match self {
            Region::UsEast1 => "us-east-1"
        }
        .to_owned()
    }
}

#[derive(Debug)]
struct Client {
    key: SigningKey,
    region: Region,
    date: DateTime<Utc>,
    host: String,
}

impl Client {
    pub fn new(secret: &str, host: String, date: DateTime<Utc>, region: Region) -> Self {
        Self {
            key: SigningKey::from_date(&secret, &date.clone(), region.clone()),
            region,
            date,
            host,
        }
    }

    pub fn request(&self, method: Method, bucket: &str) -> CanonicalRequest {
        let date = format!("{}", self.date.format("%Y%m%dT%H%M%SZ"));
        CanonicalRequest::new(method)
            .add_header("x-amz-date".to_owned(), date)
            .add_header("host".to_owned(), format!("{}.{}", bucket, self.host))
    }
}

#[derive(Debug)]
struct SigningKey {
    key: Key,
    // date: DateTime<Utc>
}

impl SigningKey {
    pub fn from_date(secret: &str, date: &DateTime<Utc>, region: Region) -> Self {
        let key = SigningKey::generate_key(secret, &date, region);
        Self {
            key,
            // date,
        }
    }

    /// Generates a new key from a secret, date, and a region
    fn generate_key(secret: &str, date: &DateTime<Utc>, region: Region) -> Key {
        let region: String = region.into();
        let date = format!("{}", date.date().format("%Y%m%d"));

        let tag = hmac::sign(&Key::new(hmac::HMAC_SHA256, &format!("AWS4{}", secret).as_bytes()), date.as_bytes());
        let tag = hmac::sign(&Key::new(hmac::HMAC_SHA256, tag.as_ref()), region.as_bytes());
        let tag = hmac::sign(&Key::new(hmac::HMAC_SHA256, tag.as_ref()), "s3".as_bytes());
        let tag = hmac::sign(&Key::new(hmac::HMAC_SHA256, tag.as_ref()), "aws4_request".as_bytes());
        Key::new(hmac::HMAC_SHA256, tag.as_ref())
    }

    /// Signs the string and returns the hex encoded signature
    pub fn sign<T: AsRef<str>>(&self, s: T) -> String {
        let tag = hmac::sign(&self.key, s.as_ref().as_bytes());
        hex::encode(tag.as_ref())
    }
}

// SigningKey:
// SigningKey {
//     key: Key {
//         algorithm: SHA256,
//     },
// }
// ------------------------------------------------------------------------
// CanonicalRequest:
// GET
// /test.txt

// host:examplebucket.s3.amazonaws.com
// range:bytes=0-9
// x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
// x-amz-date:20130524T000000Z

// host;range;x-amz-content-sha256;x-amz-date
// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
// ------------------------------------------------------------------------
// StringToSign:
// AWS4-HMAC-SHA256
// 20130524T000000Z
// 20130524/us-east-1/s3/aws4_request
// 7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972
// ------------------------------------------------------------------------
// Signature:
// f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41
// ------------------------------------------------------------------------
