extern crate dtm_database_backend;

use axum::{routing::get, Router};
use dtm_database_backend::{
  env::{BootingMode, ENV, PORT},
  openapi::{schema_validation, ApiDoc},
  routes::version::{hello, version},
};
use std::{
  env,
  net::{Ipv4Addr, SocketAddr},
};
use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
  // set port, ipv4, and socket.
  let socket_v4: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, *PORT));

  let frozen_schema =
    *ENV == BootingMode::Production || env::args().any(|arg| arg == "--frozen-schema");
  let only_schema_checking = env::args().any(|arg| arg == "--only-schema-checking");
  if schema_validation(frozen_schema) {
    println!(
      "{}",
      if frozen_schema {
        "OpenAPI schema validation passed."
      } else {
        "There are no updates to the OpenAPI schema."
      }
    );
  } else if frozen_schema {
    panic!("OpenAPI schema is different from one in the file! Try without --frozen-schema to update schema.");
  }

  if only_schema_checking {
    return;
  }

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
