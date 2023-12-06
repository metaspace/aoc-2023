use anyhow::anyhow;
use anyhow::Result;
use nom::character::complete::digit1;
use nom::character::complete::{multispace0, multispace1, u64 as nom_u64};
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{bytes::complete::tag, character::complete::space1, multi::separated_list1};

fn parse(input: &str) -> Result<Vec<(u64, u64)>> {
    let time = preceded(
        tag("Time:"),
        preceded(space1, separated_list1(space1::<_, Error<_>>, nom_u64)),
    );
    let distance = preceded(
        tag("Distance:"),
        preceded(space1, separated_list1(space1, nom_u64)),
    );

    let mut parser = terminated(separated_pair(time, multispace1, distance), multispace0);

    let (times, distances) = match parser(input) {
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

    Ok(times.into_iter().zip(distances.into_iter()).collect())
}

fn parse2(input: &str) -> Result<(u64, u64)> {
    let time = map(
        preceded(
            tag("Time:"),
            preceded(space1, separated_list1(space1::<_, Error<_>>, digit1)),
        ),
        |digits| digits.join("").parse::<u64>().unwrap(),
    );

    let distance = map(
        preceded(
            tag("Distance:"),
            preceded(space1, separated_list1(space1, digit1)),
        ),
        |digits| digits.join("").parse::<u64>().unwrap(),
    );

    let mut parser = terminated(separated_pair(time, multispace1, distance), multispace0);

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

fn do_one(length: u64, record: u64) -> Result<u64> {
    // s * t-s  > r
    // st * ss > r
    let rt = length as f64;
    let record = record as f64;

    let lower = 0.5f64 * (rt - f64::sqrt(rt * rt - 4f64 * record));
    let upper = 0.5f64 * (rt + f64::sqrt(rt * rt - 4f64 * record));

    let lower: u64 = if lower.ceil() - lower == 0f64 {
        (lower + 1f64) as u64
    } else {
        lower.ceil() as u64
    };

    let upper: u64 = if upper - upper.floor() == 0f64 {
        (upper - 1f64) as u64
    } else {
        upper.floor() as u64
    };

    Ok(upper - lower + 1)
}

pub fn solve1(input: &str) -> Result<u64> {
    let a: Vec<u64> = parse(input)?
        .into_iter()
        .map(|(a, b)| do_one(a, b))
        .collect::<Result<Vec<u64>>>()?;
    Ok(a.into_iter().reduce(|acc, v| acc * v).unwrap_or(0))
}

pub fn solve2(input: &str) -> Result<u64> {
    let (time, record) = parse2(input)?;
    do_one(time, record)
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 288);
}

#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 71503);
}
