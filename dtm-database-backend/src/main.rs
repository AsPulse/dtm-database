use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_PORT: u16 = 3001;
const PORT_ENV: &str = "PORT";

#[tokio::main]
async fn main() {
  // set port, ipv4, and socket.
  let port: u16 = set_port(PORT_ENV);
  let ipv4: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
  let socket_v4: SocketAddr = SocketAddr::new(IpAddr::V4(ipv4), port);

  // build our application with a single route
  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run it with hyper on localhost:3000
  axum::Server::bind(&socket_v4)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

/// A port setter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
fn set_port(port_env: &str) -> u16 {
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

#[cfg(test)]
mod test {
  use crate::{set_port, DEFAULT_PORT, PORT_ENV};

  /// set_port function's test. this test check whether set_port function's return value is equivalent DEFAULT_PORT or not.
  #[test]
  fn test_set_default_port() {
    let port: u16 = set_port(PORT_ENV);
    assert_eq!(port, DEFAULT_PORT);
  }
}
