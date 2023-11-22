use crate::routes::version::{__path_hello, __path_version};
use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(paths(hello, version))]
pub struct ApiDoc;
