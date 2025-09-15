use crate::CliArgs;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::{json, Value};

pub async fn static_handler(config: web::Data<CliArgs>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext()) // 👈 agrega Content-Type: text/plain
        .body(config.static_text.to_string())
}

pub async fn echo_handler(req: HttpRequest) -> impl Responder {
    let headers: serde_json::Map<String, Value> = req
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                Value::String(v.to_str().unwrap_or("").to_string()),
            )
        })
        .collect();

    let response = json!({
        "path": req.path(),
        "host": req.connection_info().host(),
        "scheme": req.connection_info().scheme(),
        "method": req.method().to_string(),
        "headers": headers
    });

    HttpResponse::Ok().json(response)
}
