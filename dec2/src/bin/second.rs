use dec2::dec2_2;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = dec2_2(&input)?;
    println!("Result: {}", sum);
    Ok(())
}
