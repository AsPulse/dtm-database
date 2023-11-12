extern crate dtm_database_backend;

use axum::{routing::get, Router};
use dtm_database_backend::env::{PORT, ENV, BootingMode};
use std::net::{Ipv4Addr, SocketAddr};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const VERSION: &str = "0.0.1";

#[tokio::main]
async fn main() {
  // set port, ipv4, and socket.
  let socket_v4: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, *PORT));

  // build our application with a single route
  let app = match *ENV {
    BootingMode::Debug => Router::new()
      .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())),
    BootingMode::Production => Router::new(),
  }
  .route("/", get(hello))
  .route("/version", get(version));

  // run it with hyper on localhost:3000
  axum::Server::bind(&socket_v4)
    .serve(app.into_make_service())
    .await
    .unwrap();
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
