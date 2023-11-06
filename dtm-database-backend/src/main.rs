use axum::{routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const DEFAULT_PORT: u16 = 3001;
const PORT_ENV: &str = "PORT";
const DEFAULT_MODE: BootingModes = BootingModes::Debug;
const MODE_ENV: &str = "ENV";
const DEBUG: &str = "DEBUG";
const RELEASE: &str = "RELEASE";
const VERSION: &str = "0.0.1";

#[derive(Debug, PartialEq)]
enum BootingModes {
  Debug,
  Release,
}

impl std::fmt::Display for BootingModes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BootingModes::Debug => write!(f, "{}", DEBUG),
      BootingModes::Release => write!(f, "{}", RELEASE),
    }
  }
}

impl std::str::FromStr for BootingModes {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      DEBUG => Ok(BootingModes::Debug),
      RELEASE => Ok(BootingModes::Release),
      _ => Err("Unknown mode found"),
    }
  }
}

#[tokio::main]
async fn main() {
  // set port, ipv4, and socket.
  let socket_v4: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, get_port(PORT_ENV)));
  // set booting mode.
  let booting_mode: BootingModes = get_booting_mode(MODE_ENV);

  // build our application with a single route
  let app = match booting_mode {
    BootingModes::Debug => Router::new()
      .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())),
    BootingModes::Release => Router::new(),
  }
  .route("/", get(hello))
  .route("/version", get(version));

  // run it with hyper on localhost:3000
  axum::Server::bind(&socket_v4)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

/// A booting mode getter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
fn get_booting_mode(mode_env: &str) -> BootingModes {
  match std::env::var(mode_env) {
    Ok(value) => value.parse::<BootingModes>().unwrap(),
    Err(_) => DEFAULT_MODE,
  }
}

/// A port getter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
fn get_port(port_env: &str) -> u16 {
  match std::env::var(port_env) {
    Ok(value) => value
      .parse::<u16>()
      .expect("PORT environment variable should be u16, between 0 and 65535"),
    Err(_) => DEFAULT_PORT,
  }
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "correctly accessed")))]
async fn hello() -> &'static str {
  "Hello, World!"
}

#[utoipa::path(get, path = "/version", responses((status = 200, description = "correctly accessed")))]
async fn version() -> &'static str {
  VERSION
}

#[derive(OpenApi)]
#[openapi(paths(hello, version))]
struct ApiDoc;

#[cfg(test)]
mod test {
  use crate::{
    get_booting_mode, get_port, BootingModes, DEFAULT_MODE, DEFAULT_PORT, MODE_ENV, PORT_ENV,
  };

  /// get_port function's test. this test check whether get_port function's return value is equivalent DEFAULT_PORT or not.
  #[test]
  fn test_get_default_port() {
    let port: u16 = get_port(PORT_ENV);
    assert_eq!(port, DEFAULT_PORT);
  }

  #[test]
  fn test_get_default_mode() {
    let booting_mode: BootingModes = get_booting_mode(MODE_ENV);
    assert_eq!(booting_mode, DEFAULT_MODE);
  }
}
