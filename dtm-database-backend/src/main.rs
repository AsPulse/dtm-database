use axum::{routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};
use utoipa::OpenApi;

const DEFAULT_PORT: u16 = 3001;
const PORT_ENV: &str = "PORT";
const VERSION: &str = "0.0.1";

#[tokio::main]
async fn main() {
  // set port, ipv4, and socket.
  let socket_v4: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, get_port(PORT_ENV)));

  // build our application with a single route
  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/version", get(version));

  // run it with hyper on localhost:3000
  axum::Server::bind(&socket_v4)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

/// A port getter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
fn get_port(port_env: &str) -> u16 {
  match std::env::var(port_env) {
    Ok(value) => value
      .parse::<u16>()
      .expect("PORT environment variable should be u16, between 0 and 65535"),
    Err(error) => {
      println!("WARNING: {}. Using default port, {}", error, DEFAULT_PORT);
      DEFAULT_PORT
    }
  }
}

#[utoipa::path(
  get,
  path = "/version",
)]
async fn version() -> &'static str {
  VERSION
}

#[derive(OpenApi)]
#[openapi(
)]
struct ApiDoc;

#[cfg(test)]
mod test {
  use crate::{get_port, DEFAULT_PORT, PORT_ENV};

  /// get_port function's test. this test check whether get_port function's return value is equivalent DEFAULT_PORT or not.
  #[test]
  fn test_get_default_port() {
    let port: u16 = get_port(PORT_ENV);
    assert_eq!(port, DEFAULT_PORT);
  }
}
