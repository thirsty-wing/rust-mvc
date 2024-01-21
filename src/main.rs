use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/clicked", get(clicked));

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Error binding to socket address");

    println!("Listening on port 3001");
    axum::serve(listener, app).await.expect("Error serving");
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    Html(
        r#"
      <script src="https://unpkg.com/htmx.org@1.9.10"></script>
      <h1>Hello, World!</h1>
      <button hx-get="/clicked" hx-swap="outerHTML">
        Click Me
      </button>
    "#,
    )
}

async fn clicked() -> Html<&'static str> {
    Html("<p>You clicked the button!</p>")
}
