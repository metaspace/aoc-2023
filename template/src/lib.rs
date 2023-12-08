#[allow(unused_imports)]
use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, multispace0, multispace1, newline, one_of, space0, space1,
        u64 as nom_u64,
        i64 as nom_i64,
    },
    combinator::{map, verify},
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[allow(dead_code)]
fn parse(input: &str) -> Result<()> {

    let parser = alpha1::<_, Error<_>>;

    match parser(input) {
        Ok((remaining, input)) if remaining == "" => Ok(input),
        Ok((input, _)) => {
            println!("Remaining input:");
            println!("{input:?}");
            Err(anyhow!("Failed to parse"))
        }
        Err(e) => {
            println!("Failed to parse");
            println!("{e:?}");
            Err(anyhow!("Failed to parse"))
        }
    }?;

    todo!()
}

pub fn solve1(_input: &str) -> Result<u64> {
    todo!()
}

pub fn solve2(_input: &str) -> Result<u64> {
    todo!()
}

#[test]
fn test_part1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 6440);
}

#[test]
fn test_part2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 5905);
}
