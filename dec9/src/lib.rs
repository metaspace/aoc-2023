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
fn parse(input: &str) -> Result<Vec<Vec<i64>>> {
    let line = separated_list1(space1::<_, Error<_>>, nom_i64);
    let mut parser = terminated(separated_list1(newline, line), multispace0);

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

pub fn solve1(input: &str) -> Result<i64> {
    let input = parse(input)?;
    let mut line_results = Vec::new();

    for line in input {
        let mut current: Vec<_> = line.to_vec();
        let mut last: Vec<_> = Vec::new();
        last.push(*(line.last().unwrap()));

        loop {
            current = current.windows(2).map(|s| s[1] - s[0]).collect();
            let last_nr = *(current.last().unwrap());
            last.push(last_nr);
            if current.iter().all(|v| *v == 0) {
                break;
            }
        }

        line_results.push(last.into_iter().rev().reduce(|acc,v| acc+v).unwrap());
    }

    Ok(line_results.into_iter().reduce(|acc,v| acc+v).unwrap())
}

pub fn solve2(input: &str) -> Result<i64> {
    let input = parse(input)?;
    let mut line_results = Vec::new();

    for line in input {
        let mut current: Vec<_> = line.to_vec();
        let mut last: Vec<_> = Vec::new();
        last.push(*(line.first().unwrap()));

        loop {
            current = current.windows(2).map(|s| s[1] - s[0]).collect();
            let last_nr = *(current.first().unwrap());
            last.push(last_nr);
            if current.iter().all(|v| *v == 0) {
                break;
            }
        }

        line_results.push(last.into_iter().rev().reduce(|acc,v| v - acc).unwrap());
    }

    Ok(line_results.into_iter().reduce(|acc,v| acc+v).unwrap())
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 114);
}

#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 2);
}
