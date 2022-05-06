use mongodb::options::FindOneOptions;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let opts = FindOneOptions::builder().sort(sort)
    Ok(())
}