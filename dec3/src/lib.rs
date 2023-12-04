use std::collections::{BTreeMap, BTreeSet};
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    line: u32,
    column: usize,
}

impl From<&Span<'_>> for Position {
    fn from(token: &Span<'_>) -> Self {
        Position {
            line: token.location_line(),
            column: token.get_column(),
        }
    }
}

impl Position {
    fn up(&self) -> Option<Self> {
        if self.line == 0 {
            return None;
        }

        Some(Position {
            line: self.line - 1,
            column: self.column,
        })
    }

    fn down(&self) -> Option<Self> {
        if self.line == u32::MAX {
            return None;
        }

        Some(Position {
            line: self.line + 1,
            column: self.column,
        })
    }

    fn left(&self) -> Option<Self> {
        if self.column == 0 {
            return None;
        }

        Some(Position {
            line: self.line,
            column: self.column - 1,
        })
    }

    fn right(&self) -> Option<Self> {
        if self.column == usize::MAX {
            return None;
        }

        Some(Position {
            line: self.line,
            column: self.column + 1,
        })
    }
}

enum Element {
    Discard,
    Symbol(Position),
    Gear(Position),
    Number(Position, usize, u32),
}

fn dot<'a>(input: Span<'a>) -> IResult<Span<'a>, Element> {
    map(tag("."), |_| Element::Discard)(input)
}

fn part_number<'a>(input: Span<'a>) -> IResult<Span<'a>, Element> {
    map(digit1, |token: Span<'_>| {
        Element::Number(
            (&token).into(),
            token.fragment().len(),
            token.fragment().parse().unwrap(),
        )
    })(input)
}

fn symbol<'a>(input: Span<'a>) -> IResult<Span<'a>, Element> {
    map(
        take_while1(|c| "0123456789.\n".chars().all(|n| n != c)),
        |token: Span<'_>| Element::Symbol((&token).into()),
    )(input)
}


fn gear<'a>(input: Span<'a>) -> IResult<Span<'a>, Element> {
    map(
        tag("*"),
        |token: Span<'_>| Element::Gear((&token).into()),
    )(input)
}

fn line<'a>(input: Span<'a>) -> IResult<Span<'a>, Vec<Element>> {
    many1(alt((dot, part_number, gear, symbol)))(input)
}

fn parse<'a>(input: Span<'a>) -> Result<Vec<Vec<Element>>> {
    match tuple((separated_list1(newline, line), opt(newline)))(input) {
        Ok((input, elements)) if input.fragment() == &"" => Ok(elements.0),
        Ok((input, _elements)) => {
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

fn get_locations(pos: Position) -> Vec<Position> {
    let mut positions = Vec::new();
    if let Some(pos) = pos.up().and_then(|pos| pos.left()) {
        positions.push(pos);
    }
    if let Some(pos) = pos.up() {
        positions.push(pos);
    }
    if let Some(pos) = pos.up().and_then(|pos| pos.right()) {
        positions.push(pos);
    }

    if let Some(pos) = pos.left() {
        positions.push(pos);
    }
    if let Some(pos) = pos.right() {
        positions.push(pos);
    }

    if let Some(pos) = pos.down().and_then(|pos| pos.left()) {
        positions.push(pos);
    }
    if let Some(pos) = pos.down() {
        positions.push(pos);
    }
    if let Some(pos) = pos.down().and_then(|pos| pos.right()) {
        positions.push(pos);
    }

    positions
}

pub fn dec3_1(input: &str) -> Result<u32> {
    let input = Span::new(input);

    let elements = parse(input)?;
    let mut tree = BTreeMap::new();
    let mut symbols = Vec::new();

    for v in elements {
        for e in v {
            match e {
                Element::Number(pos, len, num) => {
                    let pos0 = Position {
                        line: pos.line,
                        column: pos.column,
                    };
                    for i in 0..len {

                        let pos_i = Position {
                            line: pos.line,
                            column: pos.column + i,
                        };
                        tree.insert(pos_i, (pos0.clone(), num));
                    }
                }
                Element::Symbol(pos)|Element::Gear(pos) => symbols.push(pos),
                _ => (),
            }
        }
    }

    let mut numbers = Vec::new();
    for pos in symbols {
        for pos in get_locations(pos) {
            if let Some(number) = tree.get(&pos) {
                numbers.push(number.clone());
            }
        }
    }

    let numbers = numbers.into_iter().collect::<BTreeSet<_>>().into_iter().map(|x| x.1).collect::<Vec<_>>();

    Ok(numbers.into_iter().reduce(|acc, v| acc + v).unwrap_or(0))
}

pub fn dec3_2(input: &str) -> Result<u32> {
    let input = Span::new(input);

    let elements = parse(input)?;
    let mut tree = BTreeMap::new();
    let mut symbols = Vec::new();

    for v in elements {
        for e in v {
            match e {
                Element::Number(pos, len, num) => {
                    let pos0 = Position {
                        line: pos.line,
                        column: pos.column,
                    };
                    for i in 0..len {

                        let pos_i = Position {
                            line: pos.line,
                            column: pos.column + i,
                        };
                        tree.insert(pos_i, (pos0.clone(), num));
                    }
                }
                Element::Gear(pos) => symbols.push(pos),
                _ => (),
            }
        }
    }

    let mut numbers = Vec::new();
    for pos in symbols {
        let mut gear_numbers = Vec::new();
        for pos in get_locations(pos) {
            if let Some(number) = tree.get(&pos) {
                gear_numbers.push(number.clone());
            }
        }
        let gear_numbers = gear_numbers.into_iter().collect::<BTreeSet<_>>().into_iter().map(|x| x.1).collect::<Vec<_>>();
        if gear_numbers.len() == 2 {
            numbers.push(gear_numbers[0] * gear_numbers[1]);
        }
    }


    Ok(numbers.into_iter().reduce(|acc, v| acc + v).unwrap_or(0))
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = dec3_1(s);
    assert_eq!(r.unwrap(), 4361);
}

#[test]
fn test_part_1() {
    let s = include_str!("../input.txt");
    let r = dec3_1(s);
    assert_eq!(r.unwrap(), 533784);
}


#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = dec3_2(s);
    assert_eq!(r.unwrap(), 467835);
}

#[test]
fn test_part_2() {
    let s = include_str!("../input.txt");
    let r = dec3_2(s);
    assert_eq!(r.unwrap(), 78826761);
}
