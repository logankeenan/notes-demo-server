extern crate dotenv;
use dotenv::dotenv;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let server = notes_demo::create();
    let port = dotenv::var("port")?;
    server.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}
