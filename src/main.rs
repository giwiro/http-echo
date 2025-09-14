mod cmd;
mod common;
mod modules;

use clap::{builder::PossibleValuesParser, Parser};
use cmd::web_server::start_server;
use log::LevelFilter;

#[derive(Parser, Clone)]
#[command(version)]
struct CliArgs {
    /// Execution mode (echo, static)
    #[arg(
        long,
        value_parser = PossibleValuesParser::new(["echo", "static"]),
        hide_possible_values = true,
        default_value = "echo",
    )]
    mode: String,
    /// Log level of the server (off, error, warn, info, debug, trace)
    #[arg(long, value_enum, default_value = "info")]
    log_level: LevelFilter,
    /// Text returned by the server if execution mode is static
    #[arg(long, default_value = "Hello World!")]
    static_text: String,
    /// Server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    /// Server port
    #[arg(long, default_value_t = 8080)]
    port: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = <CliArgs as clap::Parser>::parse();
    colog::default_builder()
        .filter_level(args.log_level) // Set a custom default filter level
        .init();
    start_server(args).await
}
