use crate::common::web::Router;
use crate::modules::version::web::controller::version_handler;
use crate::CliArgs;
use actix_web::web;

pub struct VersionRouter {}

impl Router for VersionRouter {
    fn register_route(app_cfg: &mut web::ServiceConfig, _: &CliArgs) {
        app_cfg.route("/version", web::get().to(version_handler));
    }
}
