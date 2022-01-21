extern crate dotenv;
use dotenv::dotenv;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let server = notes_demo::create();
    let port = dotenv::var("PORT")?;
    server.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
