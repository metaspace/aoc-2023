use dec4::solve2;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = solve2(&input)?;
    println!("Result: {}", sum);
    Ok(())
}
