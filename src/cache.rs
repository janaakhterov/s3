/// The CacheControl header used to determine how an object is to be cached.
/// For more information go to [rfc2616](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9)
pub enum CacheControl<T: AsRef<str>> {
    NoCache,
    NoStore,
    MaxAge(u64),
    MaxStale(Option<u64>),
    MinFresh(u64),
    NoTransform,
    OnlyIfCached,
    Public,
    Private(Option<T>),
    MustRevalidate,
    ProxyRevalidate,
    SMaxAge(u64),
    Extension(T),
}

impl<T: AsRef<str>> Into<String> for CacheControl<T> {
    fn into(self) -> String {
        let cache = match self {
            CacheControl::NoCache => "no-cache".to_owned(),
            CacheControl::NoStore => "no-store".to_owned(),
            CacheControl::MaxAge(value) => return format!("max-age={}", value),
            CacheControl::MaxStale(value) => {
                if let Some(value) = value {
                    format!("max-stale={}", value)
                } else {
                    "max-stale".to_owned()
                }
            }
            CacheControl::MinFresh(value) => return format!("min-fresh={}", value),
            CacheControl::NoTransform => return "no-transform".to_owned(),
            CacheControl::OnlyIfCached => return "only-if-cached".to_owned(),
            CacheControl::Public => return "public".to_owned(),
            CacheControl::Private(value) => {
                if let Some(value) = value {
                    format!("private={}", value.as_ref())
                } else {
                    "private".to_owned()
                }
            }
            CacheControl::MustRevalidate => return "must-revalidate".to_owned(),
            CacheControl::ProxyRevalidate => return "proxy-revalidate".to_owned(),
            CacheControl::SMaxAge(value) => return format!("s-maxage={}", value),
            CacheControl::Extension(value) => return value.as_ref().to_owned(),
        };

        format!("CacheControl:{}", cache)
    }
}
