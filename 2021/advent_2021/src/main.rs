use anyhow::Result;
mod puzzles;

#[tokio::main]
async fn main() -> Result<()> {
    puzzles::day_1::execute()?;

    Ok(())
}
