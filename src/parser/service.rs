use super::equals;
use nom::{
    bytes::complete::take_while,
    character::complete::{
        line_ending,
        space0,
    },
    IResult,
};

use super::key::{
    key_value,
    Pair,
};

#[derive(Debug, PartialEq)]
pub(super) struct Service<'a> {
    pub(super) name: ServiceName<'a>,
    pub(super) options: Vec<Pair<'a>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct ServiceName<'a>(&'a str);

pub(super) fn service<'a>(input: &'a str) -> IResult<&'a str, Service> {
    let name = service_name(input)?;

    let result = space0(name.0)?;
    let result = equals(result.0)?;
    let result = space0(result.0)?;
    let result = line_ending(result.0)?;

    let name = name.1;

    let mut options = Vec::new();
    let mut input = result.0;

    loop {
        input = space0(input)?.0;

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

    Ok((input, Service { name, options }))
}

fn service_name<'a>(input: &'a str) -> IResult<&'a str, ServiceName<'a>> {
    let result = take_while(|ch: char| ch.is_alphanumeric() || ch == '_')(input)?;

    Ok((result.0, ServiceName(result.1)))
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
    fn service_test() {
        let token = Ok((
            "",
            Service {
                name: ServiceName("s3"),
                options: vec![Pair {
                    key: Key("max_queries"),
                    value: Value("20"),
                }],
            },
        ));
        assert_eq!(
            service(
                r#"s3 = 
                    max_queries = 20"#
            ),
            token
        );
    }

    #[test]
    fn service_name_test() {
        let token = Ok((" = \n", ServiceName("s3")));
        assert_eq!(service_name("s3 = \n"), token);
    }
}
