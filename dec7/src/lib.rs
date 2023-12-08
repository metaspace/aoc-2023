use std::cmp::max;
use std::cmp::Ordering::{Equal, Greater, Less};

use anyhow::{anyhow, Result};
use nom::character::complete::u64 as nom_u64;
use nom::combinator::map;
use nom::{
    character::complete::{multispace0, newline, one_of, space1},
    combinator::verify,
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

fn parse(input: &str) -> Result<Vec<(Vec<u8>, u64)>> {
    let cards = verify(
        many1(map(
            one_of::<_, _, (&str, ErrorKind)>("AKQJT987654321"),
            |c| match c {
                '1' => 0u8,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                '9' => 8,
                'T' => 9,
                'J' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => unreachable!(),
            },
        )),
        |cards: &Vec<u8>| cards.len() == 5,
    );
    let line = separated_pair(cards, space1, nom_u64);
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

fn rank(hand: &Vec<u8>) -> Result<u8> {
    let mut hand = hand.clone();
    hand.sort();
    let mut unique = hand.clone();
    unique.dedup();

    let mut counts = unique
        .iter()
        .map(|c1| hand.iter().filter(|c2| c1 == *c2).count())
        .collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    let rank = if counts[0] == 5 {
        6
    } else if counts[0] == 4 {
        5
    } else if counts[0] == 3 && counts[1] == 2 {
        4
    } else if counts[0] == 3 {
        3
    } else if counts[0] == 2 && counts[1] == 2 {
        2
    } else if counts[0] == 2 {
        1
    } else {
        0
    };

    Ok(rank)
}

pub fn solve1(input: &str) -> Result<u64> {
    let mut hands = parse(input)?
        .into_iter()
        .map(|(hand, bet)| (rank(&hand).unwrap(), hand, bet))
        .collect::<Vec<_>>();

    hands.sort_by(
        |(rank1, hand1, _), (rank2, hand2, _)| match rank1.cmp(rank2) {
            Less => Less,
            Equal => hand1.cmp(hand2),
            Greater => Greater,
        },
    );

    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_rank, _hand, bet))| bet as u64 * (i as u64 + 1))
        .reduce(|acc, val| acc + val)
        .unwrap_or(0);

    Ok(result)
}

fn parse2(input: &str) -> Result<Vec<(Vec<u8>, u64)>> {
    let cards = verify(
        many1(map(
            one_of::<_, _, (&str, ErrorKind)>("AKQJT987654321"),
            |c| match c {
                'J' => 0u8,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => unreachable!(),
            },
        )),
        |cards: &Vec<u8>| cards.len() == 5,
    );
    let line = separated_pair(cards, space1, nom_u64);
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

// A better solution is to just make the joker the same of the most frequent
// card in the hand
fn rank2(hand: &Vec<u8>) -> Result<u8> {
    let hand = hand.clone();
    let (mut jokers, rest) = hand.into_iter().partition::<Vec<_>, _>(|c| *c == 0);

    if rest.len() == 5 {
        return rank(&rest);
    }

    let mut max_rank = 0;

    loop {

        let mut hand = rest.clone();
        hand.append(&mut jokers.clone());
        max_rank = max(max_rank, rank(&hand)?);

        if jokers.iter().all(|c| *c == 13) {
            break;
        }

        for idx in 0..jokers.len() {
            if jokers[idx] < 13 {
                jokers[idx] += 1;
                break;
            }
            else {
                jokers[idx] = 0;
            }
        }

    }

    Ok(max_rank)
}

pub fn solve2(input: &str) -> Result<u64> {
    let mut hands = parse2(input)?
        .into_iter()
        .map(|(hand, bet)| (rank2(&hand).unwrap(), hand, bet))
        .collect::<Vec<_>>();

    hands.sort_by(
        |(rank1, hand1, _), (rank2, hand2, _)| match rank1.cmp(rank2) {
            Less => Less,
            Equal => hand1.cmp(hand2),
            Greater => Greater,
        },
    );

    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_rank, _hand, bet))| bet as u64 * (i as u64 + 1))
        .reduce(|acc, val| acc + val)
        .unwrap_or(0);

    Ok(result)
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 6440);
}

#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 5905);
}
