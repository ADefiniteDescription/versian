use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, multispace0},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Version<'a> {
    epoch: Option<&'a str>,
    upstream_version: &'a str,
    debian_revision: Option<&'a str>,
    architecture: Option<&'a str>,
}

fn parse_epoch(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_digit())(input)
}

fn parse_upstream_version(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '+' || c == '.' || c == '~')(input)
}

fn parse_debian_revision(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '+' || c == '.' || c == '~' || c == '-')(
        input,
    )
}

fn parse_architecture(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '+')(input)
}

fn parse_version(input: &str) -> IResult<&str, Version> {
    let (input, epoch) = opt(preceded(tag(":"), parse_epoch))(input)?;
    let (input, upstream_version) = parse_upstream_version(input)?;
    let (input, debian_revision) = opt(preceded(tag("-"), parse_debian_revision))(input)?;
    let (input, architecture) = opt(preceded(tag("."), parse_architecture))(input)?;
    Ok((
        input,
        Version {
            epoch,
            upstream_version,
            debian_revision,
            architecture,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_parser() {
        let version = "5.10.104-tegra-35.2.1-20230124153320";
        let parsed = parse_version(version);
        println!("{:?}", parsed);
    }
}
