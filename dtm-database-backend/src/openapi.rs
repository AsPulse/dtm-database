use std::env;

use crate::routes::version::{__path_hello, __path_version};
use utoipa::OpenApi;

const OPENAPI_SCHEMA_IN_FILE: &str = include_str!("../../openapi.json");

/// Check whether the generated schema and the schema in file are equal or not.
/// If they are equal, returns true. Otherwise, returns false.
/// If `frozen` is false, the generated schema will be written to the file.
pub fn is_schema_valid(frozen: bool) -> bool {
  let true_schema = ApiDoc::openapi()
    .to_json()
    .expect("Failed to convert OpenAPI to JSON.");

  if openapi_schema_equal(&true_schema, OPENAPI_SCHEMA_IN_FILE) {
    true
  } else {
    if !frozen {
      let schema_file = env::current_dir()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("openapi.json"));
      if let Some(schema_file) = schema_file {
        match std::fs::write(&schema_file, true_schema) {
          Ok(_) => {
            println!(
              "Successfully update OpenAPI schema written in a file: {:?}",
              &schema_file
            );
          }
          Err(e) => {
            eprintln!("Failed to write OpenAPI schema to file: {:?}", e);
          }
        }
      } else {
        eprintln!("Failed to access directory that OpenAPI schema should be in.");
      }
    }
    false
  }
}

/// Returns `true` if two OpenAPI schemas given as a JSON string are equal.
fn openapi_schema_equal(a: impl Into<String>, b: impl Into<String>) -> bool {
  a.into() == b.into()
}

#[derive(OpenApi)]
#[openapi(paths(hello, version))]
pub struct ApiDoc;
