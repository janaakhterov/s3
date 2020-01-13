use crate::{
    Acl,
    Headers,
    OptionalHeader,
};
use http::{
    request::Builder,
    Error,
};

pub(crate) enum GrantValue {
    Email,
    Id,
    Uri,
}

pub(crate) enum GrantType {
    Read,
    WriteAcp,
    ReadAcp,
    FullControl,
}

pub(crate) struct Grants {
    pub(crate) read: Option<String>,
    pub(crate) write_acp: Option<String>,
    pub(crate) read_acp: Option<String>,
    pub(crate) full_control: Option<String>,
}

impl Into<&'static str> for GrantValue {
    fn into(self) -> &'static str {
        match self {
            GrantValue::Email => "emailAddress",
            GrantValue::Id => "id",
            GrantValue::Uri => "uri",
        }
    }
}

pub(crate) trait IntoGrants {
    fn into_grants(self) -> Grants;
}

impl<T: AsRef<str>> Into<Grants> for Vec<(GrantType, GrantValue, T)> {
    fn into(self) -> Grants {
        let mut read = Vec::new();
        let mut write_acp = Vec::new();
        let mut read_acp = Vec::new();
        let mut full_control = Vec::new();

        for (grant_type, value_type, value) in self {
            let value_type: &'static str = value_type.into();
            let value = format!(r#"{}="{}""#, value_type, value.as_ref());

            match grant_type {
                GrantType::Read => read.push(value),
                GrantType::WriteAcp => write_acp.push(value),
                GrantType::ReadAcp => read_acp.push(value),
                GrantType::FullControl => full_control.push(value),
            }
        }

        let read = if !read.is_empty() {
            Some(read.join(", "))
        } else {
            None
        };

        let write_acp = if !write_acp.is_empty() {
            Some(write_acp.join(", "))
        } else {
            None
        };

        let read_acp = if !read_acp.is_empty() {
            Some(read_acp.join(", "))
        } else {
            None
        };

        let full_control = if !full_control.is_empty() {
            Some(full_control.join(", "))
        } else {
            None
        };

        Grants {
            read,
            write_acp,
            read_acp,
            full_control,
        }
    }
}

pub(crate) trait OptionalGrants {
    fn optional_grants<T: AsRef<str>>(
        self,
        acl: Option<Acl>,
        grants: Vec<(GrantType, GrantValue, T)>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl OptionalGrants for Builder {
    fn optional_grants<T: AsRef<str>>(
        self,
        acl: Option<Acl>,
        grants: Vec<(GrantType, GrantValue, T)>,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let (Some(acl), true) = (acl, grants.is_empty()) {
            let acl: &'static str = acl.into();
            self.optional_header(Headers::X_AMZ_ACL, &Some(acl))
        } else {
            let grants: Grants = grants.into();
            self.optional_header(Headers::X_AMZ_GRANT_READ, &grants.read)?
                .optional_header(Headers::X_AMZ_GRANT_WRITE_ACP, &grants.write_acp)?
                .optional_header(Headers::X_AMZ_GRANT_READ_ACP, &grants.read_acp)?
                .optional_header(Headers::X_AMZ_GRANT_FULL_CONTROL, &grants.full_control)
        }
    }
}
