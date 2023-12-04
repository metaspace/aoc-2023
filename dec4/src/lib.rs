use anyhow::anyhow;
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::{map, opt},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

type Cards = Vec<(Vec<u64>, Vec<u64>)>;

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, map(digit1, |s: &str| s.parse::<u64>().unwrap()))(input)
}

fn values(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(space1, numbers, space0)(input)
}

fn parse(input: &str) -> Result<Cards> {
    let header = map(
        tuple((tag::<_, _, Error<_>>("Card"), space1, digit1, tag(":"))),
        |(_, _, d, _): (_, _, &str, _)| d.parse::<u64>().unwrap(),
    );
    let line = map(tuple((values, tag("|"), values)), |(a, _, c)| (a, c));
    let full_line = preceded(header, line);
    let mut cards = terminated(separated_list1(newline, full_line), opt(newline));
    match cards(input) {
        Ok((remaining, cards)) if remaining == "" => Ok(cards),
        Ok((remain,_)) => {
            println!("Remaining input:");
            println!("{remain:?}");
            Err(anyhow!("Failed to parse"))
        }
        Err(_) => Err(anyhow!("Failed to parse")),
    }
}

pub fn solve1(input: &str) -> Result<u64> {
    let cards = parse(input)?;
    let result = cards
        .into_iter()
        .map(|(win, draw)| {
            let count: u32 = draw
                .into_iter()
                .filter(|number| win.contains(number))
                .count()
                .try_into()
                .unwrap();
            if count > 0 {
                2u64.pow(count - 1)
            } else {
                0
            }
        })
        .reduce(|acc, v| acc + v)
        .unwrap_or(0);

    Ok(result)
}

pub fn solve2(input: &str) -> Result<usize> {
    let cards = parse(input)?;
    let mut stack = (0..cards.len()).collect::<Vec<_>>();
    let mut count = 0;
    while !stack.is_empty() {
        let mut next = Vec::new();
        for i in &mut stack.drain(..) {
            count += 1;
            let (win, draw) = &cards[i];
            let wins = draw
                .into_iter()
                .filter(|number| win.contains(number))
                .count();
            next.append(&mut (i + 1..i + wins + 1).collect::<Vec<_>>());
        }
        stack.append(&mut next);
    }

    Ok(count)
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 13);
}

#[test]
fn test_part_1() {
    let s = include_str!("../input.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 21919);
}

#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 30);
}

#[test]
fn test_part_2() {
    let s = include_str!("../input.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 9881048);
}
