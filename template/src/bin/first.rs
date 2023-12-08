use aoc::solve1 as solve;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = solve(&input)?;
    println!("Result: {}", sum);
    Ok(())
}
