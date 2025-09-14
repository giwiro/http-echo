use crate::modules::version::use_case::get_version;
use actix_web::{http, HttpResponse, Responder};

pub async fn version_handler() -> impl Responder {
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, "text/plain"))
        .body(get_version())
}
