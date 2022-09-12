use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 5353));
    println!("Server running on http://{addr}");
    axum::Server::bind(&addr)
        .serve(axum_boilerplate::app().into_make_service())
        .await?;
    Ok(())
}
