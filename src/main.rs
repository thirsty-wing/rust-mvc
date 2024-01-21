use axum::{routing::get, Router, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Error binding to socket address");

    println!("Listening on port 3001");
    axum::serve(listener, app).await.expect("Error serving");
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
