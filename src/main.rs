use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_html))
        .route("/clicked", get(clicked_html))
        .route("/about", get(about_html));

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Error binding to socket address");

    println!("Listening on port 3001");
    axum::serve(listener, app).await.expect("Error serving");
}

fn page_template(title: String, body: String) -> String {
    let global_css = include_str!("./pico.min.css");
    let htmx = include_str!("./htmx.min.js");
    format!(
        r#"
        <!doctype html>
        <html>
          <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <style>
              {global_css}
            </style>
            <script>
              {htmx}
            </script>
            <title>{title}</title>
          </head>
          <main class="container">
            <body hx-boost="true">
              {body}
            </body>
          </main>
        </html>
        "#,
    )
}

// basic handler that responds with a static string
async fn root_html() -> Html<String> {
    Html(root().to_string())
}

// basic handler that responds with a static string
fn root() -> String {
    page_template(
        "Root".to_string(),
        r#"
      <h1>Hello, World!</h1>
      <button hx-get="/clicked" hx-swap="outerHTML">
        Click Me
      </button>
      <a href="/" >home page</a>
      <a href="/about" >about page</a>
    "#
        .to_string(),
    )
}

async fn clicked_html() -> Html<&'static str> {
    Html("<p>You clicked the button!</p>")
}

async fn about_html() -> Html<String> {
    Html(about().to_string())
}

fn about() -> String {
    page_template(
        "About".to_string(),
        r#"
      <h1>About</h1>
      <button hx-get="/clicked" hx-swap="outerHTML">
        Click Me
      </button>
      <a href="/" >home page</a>
      <a href="/about" >about page</a>
    "#
        .to_string(),
    )
}
