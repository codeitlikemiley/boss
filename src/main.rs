use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/ping", get(ping));

    // Bind to address
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("   Try http://127.0.0.1:3000/ping");

    // Run it
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn ping() -> &'static str {
    "pong"
}
