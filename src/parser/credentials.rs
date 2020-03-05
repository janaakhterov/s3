use super::profile::profiles;
use crate::{
    error,
    Error,
};

#[derive(Debug, PartialEq)]
pub struct Credentials<'a> {
    pub aws_secret_access_key: &'a str,
    pub aws_access_key_id: &'a str,
}

pub(crate) fn credentials<'a>(
    profile_name: &'a str,
    input: &'a str,
) -> Result<Option<Credentials<'a>>, Error> {
    let profiles = profiles(input)
        .map_err(|_| error::Credentials::AwsCredentialsParseError)?
        .1;
    let mut aws_access_key_id: Option<&'a str> = None;
    let mut aws_secret_access_key: Option<&'a str> = None;

    if let Some(options) = profiles.get(profile_name) {
        aws_access_key_id = options.get("aws_access_key_id").map(|key| *key);
        aws_secret_access_key = options.get("aws_secret_access_key").map(|key| *key);
    }

    if let (Some(aws_access_key_id), Some(aws_secret_access_key)) =
        (aws_access_key_id, aws_secret_access_key)
    {
        Ok(Some(Credentials {
            aws_access_key_id,
            aws_secret_access_key,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn credentials_test() -> Result<(), Error> {
        let credentials = credentials(
            "default",
            r#"
# Just a comment
# Just a comment

[aws_real]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG

# Just a comment
[default]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
random_key              = random_data
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG
random_key2             = random_data

# Just a comment
"#,
        )?;

        assert!(credentials.is_some());

        let credentials = credentials.unwrap();

        assert!(credentials.aws_access_key_id == "6KSUI28SEVTXB63GLSLU");
        assert!(credentials.aws_secret_access_key == "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG");

        Ok(())
    }

    #[test]
    fn credentials_different_profile_test() -> Result<(), Error> {
        let credentials = credentials(
            "aws_real",
            r#"
# Just a comment
# Just a comment

[aws_real]
aws_access_key_id       = REAL_KEY
aws_secret_access_key   = REAL_SECRET

# Just a comment
[default]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
random_key              = random_data
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG
random_key2             = random_data

# Just a comment
"#,
        )?;

        assert!(credentials.is_some());

        let credentials = credentials.unwrap();

        assert!(credentials.aws_access_key_id == "REAL_KEY");
        assert!(credentials.aws_secret_access_key == "REAL_SECRET");

        Ok(())
    }

    #[test]
    fn credentials_no_keys() -> Result<(), Error> {
        let credentials = credentials(
            "aws_real",
            r#"
# Just a comment
# Just a comment

[aws_real]

# Just a comment
"#,
        )?;

        assert!(credentials.is_none());

        Ok(())
    }

    #[test]
    fn credentials_one_key() -> Result<(), Error> {
        let credentials = credentials(
            "aws_real",
            r#"
# Just a comment
# Just a comment

[aws_real]
aws_access_key_id = REAL_KEY

# Just a comment
"#,
        )?;

        assert!(credentials.is_none());

        Ok(())
    }
}
