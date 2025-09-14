use crate::CliArgs;
use actix_web::web;

pub trait Router {
    fn register_route(app_cfg: &mut web::ServiceConfig, cli_args: &CliArgs);
}
