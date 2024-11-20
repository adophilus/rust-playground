mod http;
mod migrate;
mod model;
mod utils;

use model::{Config, Context};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let matches = clap::command!()
        .subcommand_required(true)
        .subcommand(clap::Command::new("serve"))
        .subcommand(clap::Command::new("migrate"))
        .get_matches();

    let config = Config::new();
    let db = utils::init_db(&config).await;
    let ctx = Context::from(config, db);

    match matches.subcommand() {
        Some(("serve", _)) => http::start_server(ctx).await,
        Some(("migrate", _)) => migrate::migrate(ctx).await,
        _ => unreachable!("Clap would ensure that we cannot get here"),
    };
}
