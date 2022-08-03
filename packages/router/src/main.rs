use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use usecase::UserUsecase;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    let user = UserUsecase::get_user_by_id(1);
    Html("<h1>Hello, World!</h1>")
}
