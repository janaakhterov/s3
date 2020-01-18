use nom::{
    bytes::complete::take_while,
    character::complete::space0,
    IResult,
};

use super::equals;

#[derive(Debug, PartialEq)]
pub(super) struct Pair<'a> {
    pub(super) key: &'a str,
    pub(super) value: &'a str,
}

#[derive(Debug, PartialEq)]
pub(super) struct Value<'a>(pub(super) &'a str);

#[derive(Debug, PartialEq)]
pub(super) struct Key<'a>(pub(super) &'a str);

pub(super) fn key_value<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let result = key(input)?;
    let key = result.1;

    let result = space0(result.0)?;
    let result = equals(result.0)?;
    let result = space0(result.0)?;

    let result = value(result.0)?;
    let value = result.1;

    Ok((result.0, (key, value)))
}

pub(super) fn value<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    take_while(|ch: char| ch.is_alphanumeric() || ch == '-' || ch == '_')(input)
}

pub(super) fn key<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    take_while(|ch: char| ch.is_lowercase() || ch == '_')(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aws_access_key_id_test() {
        let token = Ok(("", "aws_access_key_id"));
        assert_eq!(key("aws_access_key_id"), token);
    }

    #[test]
    fn aws_secret_access_key_test() {
        let token = Ok(("", "aws_secret_access_key"));
        assert_eq!(key("aws_secret_access_key"), token);
    }

    #[test]
    fn access_key_value_test() -> Result<(), anyhow::Error> {
        let token = ("aws_access_key_id", "6KSUI28SEVTXB63GLSLU");

        let tokens = key_value("aws_access_key_id=6KSUI28SEVTXB63GLSLU")?.1;

        assert_eq!(tokens, token);

        Ok(())
    }

    #[test]
    fn secret_key_value_test() -> Result<(), anyhow::Error> {
        let token = (
            "aws_secret_access_key",
            "NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG",
        );

        let tokens =
            key_value("aws_secret_access_key   =    NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG")?.1;

        assert_eq!(tokens, token);

        Ok(())
    }
}
