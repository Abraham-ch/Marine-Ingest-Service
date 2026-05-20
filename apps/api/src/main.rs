use std::env;
use dotenv::dotenv;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
  dotenv().ok();

  let port = match env::var("API_PORT") {
    Ok(val) => val,
    Err(_) => {
      eprintln!("No api_port enviroment provided.");
      return;
    }
  };

  let app = Router::new().route("/", get(|| async { "Hello, World!" }));
  let listener = tokio::net::TcpListener::bind(port.clone()).await.unwrap();
  println!("Server started at http://{}", port);
  
  axum::serve(listener,app).await.unwrap();
}
