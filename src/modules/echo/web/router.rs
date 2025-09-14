use crate::common::web::Router;
use crate::modules::echo::web::controller::{echo_handler, static_handler};
use crate::CliArgs;
use actix_web::web;

pub struct EchoRouter {}

impl Router for EchoRouter {
    fn register_route(app_cfg: &mut web::ServiceConfig, cli_args: &CliArgs) {
        match cli_args.mode.as_str() {
            "static" => app_cfg.route("/{tail:.*}", web::get().to(static_handler)),
            "echo" => app_cfg.route("/{tail:.*}", web::get().to(echo_handler)),
            _ => app_cfg.route("/{tail:.*}", web::get().to(echo_handler)),
        };
    }
}
