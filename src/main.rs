#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = notes_demo::create();

    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
