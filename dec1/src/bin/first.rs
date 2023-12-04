use dec1::*;
use anyhow::Result;

fn main() -> Result<()> {
    let value = dec1(std::io::stdin().lock())?;
    println!("Value: {value}");
    Ok(())
}

