use nom::{
    bytes::complete::take_till,
    IResult,
};

use super::{
    key::Key,
    profile::{
        profile,
        ProfileName,
    },
};

#[derive(Debug, PartialEq)]
pub struct Credentials<'a> {
    pub aws_secret_access_key: &'a str,
    pub aws_access_key_id: &'a str,
}

pub(super) fn credentials<'a>(
    profile_name: &'a str,
    input: &'a str,
) -> IResult<&'a str, Option<Credentials<'a>>> {
    let (input, _) = take_till(|ch: char| ch == '[')(input)?;

    let mut input = input;

    let mut profiles = Vec::new();

    loop {
        let result: IResult<&str, &str> = take_till(|ch: char| ch == '[')(input);
        if let Ok((skip, _)) = result {
            input = skip;
        } else {
            break;
        }

        if let Ok((end, profile)) = profile(input) {
            input = end;
            profiles.push(profile);
        } else {
            break;
        }
    }

    let mut aws_access_key_id: Option<&'a str> = None;
    let mut aws_secret_access_key: Option<&'a str> = None;

    for profile in profiles {
        if profile.name == ProfileName(profile_name) {
            for pair in profile.options {
                match pair.key {
                    Key("aws_access_key_id") => aws_access_key_id = Some(pair.value.0),
                    Key("aws_secret_access_key") => aws_secret_access_key = Some(pair.value.0),
                    _ => {}
                }
            }
        }
    }

    if let (Some(aws_access_key_id), Some(aws_secret_access_key)) =
        (aws_access_key_id, aws_secret_access_key)
    {
        Ok((
            input,
            Some(Credentials {
                aws_access_key_id,
                aws_secret_access_key,
            }),
        ))
    } else {
        Ok((input, None))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn credentials_test() {
        let token = Ok((
            "",
            Some(Credentials {
                aws_access_key_id: "6KSUI28SEVTXB63GLSLU",
                aws_secret_access_key: "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG",
            }),
        ));
        assert_eq!(
            credentials(
                "default",
                r#"
[aws_real]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG

[default]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
random_key              = random_data
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG
random_key2             = random_data
"#
            ),
            token
        );
    }

    #[test]
    fn credentials_different_profile_test() {
        let token = Ok((
            "",
            Some(Credentials {
                aws_access_key_id: "REAL_KEY",
                aws_secret_access_key: "REAL_SECRET",
            }),
        ));
        assert_eq!(
            credentials(
                "aws_real",
                r#"
[aws_real]
aws_access_key_id       = REAL_KEY
aws_secret_access_key   = REAL_SECRET

[default]
aws_access_key_id       = 6KSUI28SEVTXB63GLSLU
random_key              = random_data
aws_secret_access_key   = NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG
random_key2             = random_data
"#
            ),
            token
        );
    }
}
