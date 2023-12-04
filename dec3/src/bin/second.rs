use dec3::dec3_2;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = dec3_2(&input)?;
    println!("Result: {}", sum);
    Ok(())
}
