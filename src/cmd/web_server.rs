use crate::common::web::Router;
use crate::modules::echo::web::router::EchoRouter;
use crate::modules::version::web::router::VersionRouter;
use crate::CliArgs;
use actix_web::{web, App, HttpServer};

pub async fn start_server(cli_args: CliArgs) -> std::io::Result<()> {
    let socket = format!("{}:{}", cli_args.host, cli_args.port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cli_args.clone()))
            .configure(|app_cfg| VersionRouter::register_route(app_cfg, &cli_args))
            .configure(|app_cfg| EchoRouter::register_route(app_cfg, &cli_args))
    })
    .bind(socket)?
    .run()
    .await
}
