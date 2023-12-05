use anyhow::anyhow;
use anyhow::Result;
use nom::character::complete::u64 as nom_u64;
use nom::sequence::delimited;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated, tuple},
};
use rust_lapper::Interval;
use rust_lapper::Lapper;
use std::collections::BTreeMap;

fn parse(input: &str) -> Result<(Vec<u64>, Vec<((&str, &str), Vec<(u64, u64, u64)>)>)> {
    let number_list = separated_list1(space1::<_, nom::error::Error<_>>, nom_u64);
    let seeds = delimited(tag("seeds: "), number_list, multispace1);

    let map_name = separated_pair(alpha1, tag("-to-"), alpha1);
    let map_header = terminated(map_name, tag(" map:"));
    let map_line = map(
        tuple((nom_u64, space1, nom_u64, space1, nom_u64)),
        |(a, _, b, _, c)| (a, b, c),
    );
    let map_lines = separated_list1(newline, map_line);
    let full_map = tuple((terminated(map_header, newline), map_lines));
    let all_maps = separated_list1(multispace1, full_map);

    let mut full_input = terminated(pair(seeds, all_maps), multispace0);

    match full_input(input) {
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

fn parse2(input: &str) -> Result<(Vec<(u64, u64)>, Vec<((&str, &str), Vec<(u64, u64, u64)>)>)> {
    let number_pair = separated_pair(nom_u64, space1::<_, nom::error::Error<_>>, nom_u64);
    let number_list = separated_list1(space1, number_pair);
    let seeds = delimited(tag("seeds: "), number_list, multispace1);

    let map_name = separated_pair(alpha1, tag("-to-"), alpha1);
    let map_header = terminated(map_name, tag(" map:"));
    let map_line = map(
        tuple((nom_u64, space1, nom_u64, space1, nom_u64)),
        |(a, _, b, _, c)| (a, b, c),
    );
    let map_lines = separated_list1(newline, map_line);
    let full_map = tuple((terminated(map_header, newline), map_lines));
    let all_maps = separated_list1(multispace1, full_map);

    let mut full_input = terminated(pair(seeds, all_maps), multispace0);

    match full_input(input) {
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

#[derive(Clone, PartialEq, Eq)]
struct Mapping {
    from: u64,
    to: u64,
    length: u64,
}

impl Mapping {
    fn new(value: (u64, u64, u64)) -> Self {
        Self {
            from: value.1,
            to: value.0,
            length: value.2,
        }
    }

    fn offset(&self) -> i64 {
        self.to as i64 - self.from as i64
    }
}

pub fn solve1(input: &str) -> Result<u64> {
    let (seeds, maps) = parse(input)?;
    let mut trees: BTreeMap<String, _> = BTreeMap::new();
    let mut pairs: BTreeMap<String, String> = BTreeMap::new();

    for ((from, to), mappings) in maps {
        pairs.insert(from.into(), to.into());
        let mut range_tree: Lapper<_, Mapping> = Lapper::new(Vec::new());

        for mapping in mappings {
            let mapping: Mapping = Mapping::new(mapping);
            let interval = Interval {
                start: mapping.from,
                stop: mapping.from + mapping.length,
                val: mapping,
            };
            range_tree.insert(interval);
        }

        trees.insert(from.into(), range_tree);
    }

    let mut work: Vec<(String, u64)> = seeds
        .into_iter()
        .map(|seed| ("seed".into(), seed))
        .collect();

    let mut locations = Vec::new();

    while !work.is_empty() {
        let mut new_work = Vec::new();
        for (name, value) in work.drain(..) {
            if name == "location" {
                locations.push(value);
            } else {
                let target = pairs.get(&name).ok_or(anyhow!("Could not map"))?;
                if let Some(range) = trees
                    .get(&name)
                    .ok_or(anyhow!("Failed to find map for {name}"))?
                    .find(value, value + 1)
                    .next()
                {
                    let mapping = &range.val;
                    let new_value = ((value as i64) + mapping.offset()) as u64;
                    new_work.push((target.clone(), new_value));
                } else {
                    new_work.push((target.clone(), value));
                }
            }
        }
        work.append(&mut new_work);
    }

    let result = locations.into_iter().min().unwrap_or(0);

    Ok(result)
}

/// TODO: This should probably be optimized to process ranges instead of
/// individual points. It takes forever to run.
pub fn solve2(input: &str) -> Result<u64> {
    let (seeds, maps) = parse2(input)?;
    let mut trees: BTreeMap<String, Lapper<u64, Mapping>> = BTreeMap::new();
    let mut pairs: BTreeMap<String, String> = BTreeMap::new();

    for ((from, to), mappings) in maps {
        pairs.insert(from.into(), to.into());
        let mut range_tree: Lapper<_, Mapping> = Lapper::new(Vec::new());

        for mapping in mappings {
            let mapping: Mapping = Mapping::new(mapping);
            let interval = Interval {
                start: mapping.from,
                stop: mapping.from + mapping.length,
                val: mapping,
            };
            range_tree.insert(interval);
        }

        trees.insert(from.into(), range_tree);
    }

    let mut work = seeds
        .into_iter()
        .map(|(start, length)| (start..start + length).map(|seed| ("seed".into(), seed)))
        .flatten();

    let mut min_location = u64::MAX;

    let mut next_work = None;

    loop {
        if next_work.is_none() {
            next_work = work.next();
        }

        let Some((name, value)) = next_work.take() else {
            break;
        };

        if name == "location" {
            min_location = std::cmp::min(min_location, value);
        } else {
            let target = pairs.get(&name).ok_or(anyhow!("Could not map"))?;
            if let Some(range) = trees
                .get(&name)
                .ok_or(anyhow!("Failed to find map for {name}"))?
                .find(value, value + 1)
                .next()
            {
                let mapping = &range.val;
                let new_value = ((value as i64) + mapping.offset()) as u64;
                next_work = Some((target.clone(), new_value));
            } else {
                next_work = Some((target.clone(), value));
            }
        }
    }

    Ok(min_location)
}

//fn translate(seed: (String, u64), trees:

#[test]
fn test_example_1() {
    let s = include_str!("input1.txt");
    let r = solve1(s);
    assert_eq!(r.unwrap(), 35);
}

#[test]
fn test_example_2() {
    let s = include_str!("input1.txt");
    let r = solve2(s);
    assert_eq!(r.unwrap(), 46);
}
