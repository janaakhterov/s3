use nom::{
    branch::alt,
    bytes::complete::{
        tag,
        take,
        take_till,
        take_while,
    },
    character::complete::{
        newline,
        space0,
    },
    error::ErrorKind,
    number::streaming::be_u16,
    sequence::tuple,
    IResult,
    Needed,
};
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Key(Key<'a>),
    Equals,
    Space,
    Value(&'a str),
    Comment,
    Profile(&'a str),
}

#[derive(Debug, PartialEq)]
enum Key<'a> {
    AwsAccessKeyId,
    AwsSecretAccessKey,
    Unknown(&'a str),
}

fn key_value<'a>(input: &'a str) -> IResult<&'a str, (Token<'a>, Token<'a>)> {
    let result = key(input)?;
    let key = result.1;

    let result = space0(result.0)?;
    let result = equals(result.0)?;
    let result = space0(result.0)?;

    let result = value(result.0)?;
    let value = result.1;

    Ok((result.0, (key, value)))
}

fn key<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    let result = take_while(|ch: char| ch.is_lowercase() || ch == '_')(input)?;
    let key = Token::Key(match result.1 {
        "aws_secret_access_key" => Key::AwsSecretAccessKey,
        "aws_access_key_id" => Key::AwsAccessKeyId,
        _ => Key::Unknown(result.1),
    });

    Ok((result.0, key))
}

fn comment<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let tag = tag("#")(input)?;
    let result = take_till(|ch: char| ch == '\n')(tag.0)?;

    let result = if result.0.len() > 0 {
        take(1usize)(result.0)?
    } else {
        result
    };

    let result = (result.0, Token::Comment);
    Ok(result)
}

fn profile<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let result = tag("[")(input)?;
    let result = take_while(|ch: char| ch.is_lowercase())(result.0)?;
    let end = tag("]")(result.0)?;
    let result = (end.0, Token::Profile(result.1));
    Ok(result)
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
        assert_eq!(key("aws_access_key_id"), token);
    }

    #[test]
    fn aws_secret_access_key_test() {
        let token = Ok(("", Token::Key(Key::AwsSecretAccessKey)));
        assert_eq!(key("aws_secret_access_key"), token);
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

        let tokens = key_value("aws_access_key_id=6KSUI28SEVTXB63GLSLU")?.1;

        assert_eq!(tokens, token);

        Ok(())
    }

    #[test]
    fn secret_key_value_test() -> Result<(), anyhow::Error> {
        let token = (
            Token::Key(Key::AwsSecretAccessKey),
            Token::Value("NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG"),
        );

        let tokens =
            key_value("aws_secret_access_key   =    NQMJwbNv0qjBBtAIPbV47JOnqrGCveuqVvO8XwuG")?.1;

        assert_eq!(tokens, token);

        Ok(())
    }

    #[test]
    fn comment_test() {
        let token = Ok(("", Token::Comment));
        assert_eq!(comment("# adsflasdflkasdfjlakdjfkdsfj\n"), token);
    }

    #[test]
    fn comment_no_newline_test() {
        let token = Ok(("", Token::Comment));
        assert_eq!(comment("# adsflasdflkasdfjlakdjfkdsfj"), token);
    }

    #[test]
    fn profile_test() {
        let token = Ok(("", Token::Profile("default")));
        assert_eq!(profile("[default]"), token);
    }
}
