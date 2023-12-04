
use anyhow::{anyhow, Result};
use std::io::BufRead;

pub fn dec1(input: impl BufRead) -> Result<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let line = line?;
        let first = line
            .chars()
            .find(|c| c.to_digit(10).is_some())
            .ok_or(anyhow!("No digit on line"))?
            .to_digit(10)
            .ok_or(anyhow!("Failed to parse first digit"))?;

        let last = line
            .chars()
            .rev()
            .find(|c| c.to_digit(10).is_some())
            .ok_or(anyhow!("No digit on line"))?
            .to_digit(10)
            .ok_or(anyhow!("Failed to parse last digit"))?;

        sum += first * 10 + last;
    }

    return Ok(sum);
}

pub fn dec1_2(input: impl BufRead) -> Result<u32> {
    let mapping = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;

    for line in input.lines() {
        let line = line?;
        let first = mapping
            .iter()
            .map(|x| {
                if let Some(pos) = line.find(x.0) {
                    Some((pos, x.1))
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .min()
            .map(|x| x.unwrap().1)
            .ok_or(anyhow!("No input on line"))?;

        let last = mapping
            .iter()
            .map(|x| {
                if let Some(pos) = line.rfind(x.0) {
                    Some((pos, x.1))
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .max()
            .map(|x| x.unwrap().1)
            .ok_or(anyhow!("No input on line"))?;

        sum += first * 10 + last;
    }

    return Ok(sum);
}

#[test]
fn test_calibration_example_1() {
    let s = include_str!("input1.txt");
    let cursor = std::io::Cursor::new(s);
    let reader = std::io::BufReader::new(cursor);
    let r = dec1(reader);

    assert_eq!(r.unwrap(), 142);
}

#[test]
fn test_calibration_example_1_big() {
    let s = include_str!("../input.txt");
    let cursor = std::io::Cursor::new(s);
    let reader = std::io::BufReader::new(cursor);
    let r = dec1(reader);

    assert_eq!(r.unwrap(), 55488);
}

#[test]
fn test_calibration_example_2() {
    let s = include_str!("input2.txt");
    let cursor = std::io::Cursor::new(s);
    let reader = std::io::BufReader::new(cursor);
    let r = dec1_2(reader);

    assert_eq!(r.unwrap(), 281);
}

#[test]
fn test_calibration_example_2_big() {
    let s = include_str!("../input.txt");
    let cursor = std::io::Cursor::new(s);
    let reader = std::io::BufReader::new(cursor);
    let r = dec1_2(reader);

    assert_eq!(r.unwrap(), 55614);
}
