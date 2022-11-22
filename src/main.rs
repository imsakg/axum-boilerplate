use axum_boilerplate::{config::Config, db::Database, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let address = config.address;
    let state = AppState::new(config.clone(), Database::open(&config.database_path)?);
    println!("Server running on http://{address}");
    axum::Server::bind(&address)
        .serve(axum_boilerplate::app(state).into_make_service())
        .await?;
    Ok(())
}
