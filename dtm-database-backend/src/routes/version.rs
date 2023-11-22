#[utoipa::path(get, path = "/", responses((status = 200, description = "correctly accessed")))]
pub async fn hello() -> &'static str {
  "Hello, World!"
}

#[utoipa::path(get, path = "/version", responses((status = 200, description = "correctly accessed")))]
pub async fn version() -> &'static str {
  "0.0.3"
}
