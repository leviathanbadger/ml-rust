mod perceptron;

use anyhow::{Result};
use perceptron::{test_perceptron};

fn main() -> Result<()> {
    test_perceptron()?;

    Ok(())
}
