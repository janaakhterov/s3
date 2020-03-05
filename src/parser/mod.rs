#![allow(dead_code)]
use nom::{
    alt,
    character::complete::{
        line_ending,
        space0,
    },
    eof,
    many_till,
    named,
    tag,
    take,
    tuple,
};

pub(crate) mod config;
pub(crate) mod credentials;
mod key;
mod profile;
mod service;

#[derive(Debug, PartialEq)]
enum Token {
    Equals,
    Space,
    Comment,
}

named!(pub(super) comment(&str) -> (&str, &str, (Vec<&str>, &str)),
    tuple!(
        space0,
        pound,
        many_till!(take!(1), alt!(line_ending | eof))
    )
);

named!(pub(super) pound(&str) -> &str, tag!("#"));
named!(pub(super) equals(&str) -> &str, tag!("="));
named!(pub(super) eof(&str) -> &str, eof!());

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equals_test() {
        let token = Ok(("", "="));
        assert_eq!(equals("="), token);
    }

    #[test]
    fn comment_test() {
        assert!(comment("# adsflasdflkasdfjlakdjfkdsfj\n").is_ok());
    }

    #[test]
    fn comment_no_newline_test() {
        assert!(comment("# adsflasdflkasdfjlakdjfkdsfj").is_ok());
    }
}
