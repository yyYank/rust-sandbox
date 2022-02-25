use axum::Router;
use std::net::SocketAddr;

mod route;

fn main() {
    if let Err(e) = run() {
        println!("{}", e)
    }
}

#[tokio::main]
async fn run() -> hyper::Result<()> {
    let app = Router::new().nest("/sample", route::routes());
    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
}
