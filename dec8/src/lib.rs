use std::collections::BTreeMap;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, multispace0, multispace1, newline, one_of, space0, space1,
        u64 as nom_u64,
    },
    combinator::{map, verify},
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Failed to convert: {value}")),
        }
    }
}

fn parse(input: &str) -> Result<(Vec<Direction>, Vec<(&str, (&str, &str))>)> {
    let directions = many1(map(one_of::<_, _, Error<_>>("LR"), |c| {
        c.try_into().unwrap()
    }));

    let key = alphanumeric1;
    let payload = delimited(
        tag("("),
        separated_pair(alphanumeric1, terminated(tag(","), space0), alphanumeric1),
        tag(")"),
    );
    let node = separated_pair(key, tag(" = "), payload);
    let nodes = separated_list1(newline, node);
    let mut parser = terminated(separated_pair(directions, multispace1, nodes), multispace0);

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
    }
}

pub fn solve1(input: &str) -> Result<u64> {
    let (directions, nodes) = parse(input)?;
    let tree: BTreeMap<_, _> = nodes.into_iter().collect();

    let mut path = "AAA";
    let mut node;
    let mut steps = 0;

    for direction in directions.into_iter().cycle() {
        if path == "ZZZ" {
            break;
        }

        node = tree
            .get(path)
            .ok_or(anyhow!("Failed to find node for {path}"))?;

        path = match direction {
            Direction::Left => node.0,
            Direction::Right => node.1,
        };

        steps += 1;
    }

    Ok(steps)
}

fn label(input: &str) -> IResult<&str, [u8; 3]> {
    map(
        verify(alphanumeric1, |parsed: &str| parsed.len() == 3),
        |parsed: &str| {
            parsed
                .chars()
                .map(|c| -> u8 { c.try_into().unwrap() })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        },
    )(input)
}

fn parse2(input: &str) -> Result<(Vec<Direction>, Vec<([u8; 3], ([u8; 3], [u8; 3]))>)> {
    let directions = many1(map(one_of::<_, _, Error<_>>("LR"), |c| {
        c.try_into().unwrap()
    }));

    let payload = delimited(
        tag("("),
        separated_pair(label, terminated(tag(","), space0), label),
        tag(")"),
    );
    let node = separated_pair(label, tag(" = "), payload);
    let nodes = separated_list1(newline, node);
    let mut parser = terminated(separated_pair(directions, multispace1, nodes), multispace0);

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
    }
}

// This should eventually terminate, but did not do so in 30 min for me. Because
// of the shape partitioned and circular shape of the input, the problem can be
// reduced to finding the least common multiple of the length from start to
// finish of each node. This did not occur to me and it is not evident from the
// problem description.
pub fn solve2(input: &str) -> Result<u64> {
    let (directions, nodes) = parse2(input)?;
    let mut paths = nodes
        .iter()
        .filter(|(src, _dst)| src[2] == 'A'.try_into().unwrap())
        .map(|(src, _dst)| *src)
        .collect::<Vec<_>>();

    let tree: BTreeMap<_, _> = nodes.into_iter().collect();

    let mut steps = 0;
    for direction in directions.into_iter().cycle() {
        if paths.iter().all(|path| path[2] == 'Z'.try_into().unwrap()) {
            break;
        }

        for path in paths.iter_mut() {
            let node = tree.get(path).unwrap();
            *path = match direction {
                Direction::Left => node.0,
                Direction::Right => node.1,
            }
        }

        steps += 1;
    }

    Ok(steps)
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 2);
}

#[test]
fn test_example_2() {
    let s = include_str!("input2.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 6);
}
