use anyhow::anyhow;
use anyhow::Result;
use nom::branch::alt;
use nom::branch::permutation;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::Finish;
use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple, IResult};
use nom_permutation::permutation_opt;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

type GameList = Vec<(u32, Vec<Draw>)>;

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn tag(&self) -> &'static str {
        match self {
            Color::Red => " red",
            Color::Green => " green",
            Color::Blue => " blue",
        }
    }

    fn parse<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        map(
            alt((tag(" red"), tag(" green"), tag(" blue"))),
            |color: &str| color.into(),
        )
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            " red" => Self::Red,
            " green" => Self::Green,
            " blue" => Self::Blue,
            _ => unreachable!(),
        }
    }
}

fn parse_game(input: &str) -> IResult<&str, GameList> {
    let color = tuple((
        map(digit1, |s: &str| s.parse::<u32>().unwrap()),
        Color::parse(),
    ));
    let game = map(
        tuple((
            map(
                tuple((tag("Game "), digit1, tag(": "))),
                |(_, id, _): (_, &str, _)| id.parse::<u32>().unwrap(),
            ),
            separated_list1(
                tag("; "),
                map(
                    separated_list1(tag(", "), color),
                    |v: Vec<(u32, Color)>| -> Draw {
                        let red = if let Some((count, _)) =
                            v.iter().find(|(_count, color)| color == &Color::Red)
                        {
                            *count
                        } else {
                            0
                        };

                        let green = if let Some((count, _)) =
                            v.iter().find(|(_count, color)| color == &Color::Green)
                        {
                            *count
                        } else {
                            0
                        };

                        let blue = if let Some((count, _)) =
                            v.iter().find(|(_count, color)| color == &Color::Blue)
                        {
                            *count
                        } else {
                            0
                        };

                        Draw { red, green, blue }
                    },
                ),
            ),
        )),
        |(id, game)| (id, game),
    );

    terminated(separated_list1(newline, game), many0(newline))(input)
}

pub fn dec2_1(input: &str, bag: (u32, u32, u32)) -> Result<u32> {
    let (red, green, blue) = bag;
    let games = match parse_game(input).finish() {
        Ok(("", games)) => games,
        _ => {
            return Err(anyhow!("Failed to parse"));
        }
    };

    let sum: u32 = games
        .into_iter()
        .filter(|(_, games)| {
            games
                .iter()
                .all(|game| game.red <= red && game.blue <= blue && game.green <= green)
        })
        .map(|(id, _)| id)
        .reduce(|acc, id| acc + id)
        .unwrap();

    Ok(sum)
}

pub fn dec2_2(input: &str) -> Result<u32> {
    let games = match parse_game(input).finish() {
        Ok(("", games)) => games,
        _ => {
            return Err(anyhow!("Failed to parse"));
        }
    };

    let powers: u32 = games
        .into_iter()
        .map(|(_, games)| {
            let red = games.iter().map(|game| game.red).max().unwrap();
            let green = games.iter().map(|game| game.green).max().unwrap();
            let blue = games.iter().map(|game| game.blue).max().unwrap();
            (red, green, blue)
        }).map(|(red, green, blue)| red * green * blue).reduce(|acc, power| acc + power).unwrap();




    Ok(powers)
}

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = dec2_1(s, (12, 13, 14));
    assert_eq!(r.unwrap(), 8);
}

#[test]
fn test_part_1() {
    let s = include_str!("../input.txt");
    let r = dec2_1(s, (12, 13, 14));
    assert_eq!(r.unwrap(), 2369);
}


#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = dec2_2(s);
    assert_eq!(r.unwrap(), 2286);
}

#[test]
fn test_part_2() {
    let s = include_str!("../input.txt");
    let r = dec2_2(s);
    assert_eq!(r.unwrap(), 66363);
}
