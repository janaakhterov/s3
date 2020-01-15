use nom::{
    bytes::complete::{
        tag,
        take_while,
    },
    character::complete::line_ending,
    IResult,
};

use super::key::{
    key_value,
    Pair,
};

#[derive(Debug, PartialEq)]
pub(super) struct Profile<'a> {
    pub(super) name: ProfileName<'a>,
    pub(super) options: Vec<Pair<'a>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct ProfileName<'a>(pub(super) &'a str);

pub(super) fn profile<'a>(input: &'a str) -> IResult<&'a str, Profile> {
    let name = profile_name(input)?;
    let result = line_ending(name.0)?;
    let name = name.1;

    let mut options = Vec::new();
    let mut input = result.0;

    loop {
        if let Ok(result) = key_value(input) {
            input = result.0;

            let pair = result.1;

            options.push(Pair {
                key: pair.0,
                value: pair.1,
            });

            if let Ok(ending) = line_ending::<&str, ()>(input) {
                input = ending.0;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok((input, Profile { name, options }))
}

pub(super) fn profile_name<'a>(input: &'a str) -> IResult<&'a str, ProfileName<'a>> {
    let result = tag("[")(input)?;
    let result = take_while(|ch: char| ch.is_alphabetic() || ch == '_')(result.0)?;
    let end = tag("]")(result.0)?;

    Ok((end.0, ProfileName(result.1)))
}

#[cfg(test)]
mod test {
    use super::{
        super::key::{
            Key,
            Value,
        },
        *,
    };

    #[test]
    fn profile_test() {
        let token = Ok((
            "",
            Profile {
                name: ProfileName("default"),
                options: vec![
                    Pair {
                        key: Key("aws_access_key_id"),
                        value: Value("6KSUI28SEVTXB63GLSLU"),
                    },
                    Pair {
                        key: Key("aws_secret_access_key"),
                        value: Value("NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG"),
                    },
                ],
            },
        ));
        assert_eq!(
            profile(
                r#"[default]
aws_access_key_id=6KSUI28SEVTXB63GLSLU
aws_secret_access_key=NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG"#
            ),
            token
        );
    }

    #[test]
    fn profile_name_test() {
        let token = Ok(("", ProfileName("default")));
        assert_eq!(profile_name("[default]"), token);
    }
}
