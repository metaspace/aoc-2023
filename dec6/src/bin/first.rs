use dec6::solve1;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = solve1(&input)?;
    println!("Result: {}", sum);
    Ok(())
}
