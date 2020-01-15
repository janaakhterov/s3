#![allow(dead_code)]
use nom::{
    bytes::complete::{
        tag,
        take,
        take_till,
        take_while,
    },
    IResult,
};

mod credentials;
mod key;
mod profile;
mod service;

#[derive(Debug, PartialEq)]
enum Token {
    Equals,
    Space,
    Comment,
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

fn equals<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let tag = tag("=")(input)?;
    Ok((tag.0, Token::Equals))
}

fn space<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let result = take_while(|ch: char| ch.is_whitespace())(input)?;
    let tag = (result.0, Token::Space);
    Ok(tag)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equals_test() {
        let token = Ok(("", Token::Equals));
        assert_eq!(equals("="), token);
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
}
