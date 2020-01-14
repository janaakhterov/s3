use nom::{
    branch::alt,
    bytes::complete::{
        tag,
        take,
        take_while,
    },
    character::complete::space0,
    error::ErrorKind,
    number::streaming::be_u16,
    sequence::tuple,
    IResult,
    Needed,
};
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Key(Key),
    Equals,
    Space,
    Value(&'a str),
}

#[derive(Debug, PartialEq)]
enum Key {
    AwsAccessKeyId,
    AwsSecretAccessKey,
}

named!(
    key_value_tokens(&str) -> (Token<'_>, Token<'_>, Token<'_>, Token<'_>, Token<'_>),
    tuple!(alt!(aws_access_key_id | aws_secret_access_key), space, equals, space, value)
);

fn aws_access_key_id<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    let tag = tag("aws_access_key_id")(input)?;
    let tag = (tag.0, Token::Key(Key::AwsAccessKeyId));
    Ok(tag)
}

fn aws_secret_access_key<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let tag = tag("aws_secret_access_key")(input)?;
    let tag = (tag.0, Token::Key(Key::AwsSecretAccessKey));
    Ok(tag)
}

fn equals<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    let tag = tag("=")(input)?;
    let tag = (tag.0, Token::Equals);
    Ok(tag)
}

fn space<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    let result = take_while(|ch: char| ch.is_whitespace())(input)?;
    let tag = (result.0, Token::Space);
    Ok(tag)
}

fn value<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    let result = take_while(|ch: char| ch.is_alphanumeric())(input)?;
    let result = (result.0, Token::Value(result.1));
    Ok(result)
}

mod test {
    use super::*;

    #[test]
    fn aws_access_key_id_test() {
        let token = Ok(("", Token::Key(Key::AwsAccessKeyId)));
        assert_eq!(aws_access_key_id("aws_access_key_id"), token);
    }

    #[test]
    fn aws_secret_access_key_test() {
        let token = Ok(("", Token::Key(Key::AwsSecretAccessKey)));
        assert_eq!(aws_secret_access_key("aws_secret_access_key"), token);
    }

    #[test]
    fn equals_test() {
        let token = Ok(("", Token::Equals));
        assert_eq!(equals("="), token);
    }

    #[test]
    fn value_test() {
        let token = Ok(("", Token::Value("6KSUI28SEVTXB63GLSLU")));
        assert_eq!(value("6KSUI28SEVTXB63GLSLU"), token);
    }

    #[test]
    fn access_key_value_test() -> Result<(), anyhow::Error> {
        let token = (
            Token::Key(Key::AwsAccessKeyId),
            Token::Value("6KSUI28SEVTXB63GLSLU"),
        );

        let (_, (key, _, _, _, value)) =
            key_value_tokens("aws_access_key_id=6KSUI28SEVTXB63GLSLU")?;

        assert_eq!((key, value), token);

        Ok(())
    }

    #[test]
    fn secret_key_value_test() -> Result<(), anyhow::Error> {
        let token = (
            Token::Key(Key::AwsSecretAccessKey),
            Token::Value("NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG"),
        );

        let (_, (key, _, _, _, value)) = key_value_tokens(
            "aws_secret_access_key   =    NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG",
        )?;

        assert_eq!((key, value), token);

        Ok(())
    }
}
