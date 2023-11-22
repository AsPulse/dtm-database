extern crate dtm_database_backend;

use axum::{routing::get, Router};
use dtm_database_backend::{env::{BootingMode, ENV, PORT}, openapi::ApiDoc, routes::version::{hello, version}};
use utoipa::OpenApi;
use std::net::{Ipv4Addr, SocketAddr};
use utoipa_swagger_ui::SwaggerUi;

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
  let server = axum::Server::bind(&socket_v4).serve(app.into_make_service());

  println!("Listening on {}", socket_v4);

  server.await.unwrap();
}
