use dec2::dec2_1;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let sum = dec2_1(&input, (12,13,14))?;
    println!("Result: {}", sum);
    Ok(())
}
