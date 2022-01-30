extern crate dotenv;
use dotenv::dotenv;
use notes_demo::{AppState};

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let mut state = AppState::new();
    state.environment.insert(String::from("API_ORIGIN"), dotenv::var("API_ORIGIN")?);

    let server = notes_demo::create(state);
    let port = dotenv::var("PORT")?;
    server.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
