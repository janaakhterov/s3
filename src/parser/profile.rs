use super::{
    comment,
    eof,
    key::key_value,
};
use nom::{
    bytes::complete::{
        tag,
        take_till,
        take_while,
    },
    character::complete::line_ending,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(super) struct Profile<'a> {
    pub(super) name: &'a str,
    pub(super) options: HashMap<&'a str, &'a str>,
}

fn profile<'a>(input: &'a str) -> IResult<&'a str, Profile> {
    let name = profile_name(input)?;
    let result = line_ending(name.0)?;
    let name = name.1;

    let mut options = HashMap::new();
    let mut input = result.0;

    loop {
        if let Ok(result) = key_value(input) {
            input = result.0;

            let pair = result.1;

            options.insert(pair.0, pair.1);

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

pub(super) fn profiles<'a>(
    input: &'a str,
) -> IResult<&'a str, HashMap<&'a str, HashMap<&'a str, &'a str>>> {
    let (input, _) = take_till(|ch: char| ch == '[')(input)?;

    let mut input = input;

    let mut profiles: HashMap<&'a str, HashMap<&'a str, &'a str>> = HashMap::new();

    loop {
        // Skip comments
        if let Ok((end, _)) = comment(input) {
            input = end;
        }

        // Slip random spaces
        let result: IResult<&str, &str> = take_till(|ch: char| ch == '[')(input);
        if let Ok((skip, _)) = result {
            input = skip;
        } else {
            break;
        }

        // Attempt to parse profile
        if let Ok((end, profile)) = profile(input) {
            input = end;
            profiles.insert(profile.name, profile.options);
        }

        // If end of file is reached break out of loop
        if let Ok(_) = eof(input) {
            break;
        }
    }

    Ok((input, profiles))
}

fn profile_name<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let result = tag("[")(input)?;
    let result = take_while(|ch: char| ch.is_alphabetic() || ch == '_')(result.0)?;
    let end = tag("]")(result.0)?;

    Ok((end.0, result.1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn profile_test() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("aws_access_key_id", "6KSUI28SEVTXB63GLSLU");
        map.insert(
            "aws_secret_access_key",
            "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG",
        );

        let token = Ok((
            "",
            Profile {
                name: "default",
                options: map,
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
        let token = Ok(("", "default"));
        assert_eq!(profile_name("[default]"), token);
    }
}
