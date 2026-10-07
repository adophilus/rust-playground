mod http;
mod model;
mod repo;
#[cfg(test)]
mod test;

use model::{Config, Context};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let matches = clap::command!()
        .subcommand_required(true)
        .subcommand(clap::Command::new("serve"))
        .get_matches();

    let ctx = Context::init(Config::new()).await;

    match matches.subcommand() {
        Some(("serve", _)) => http::start_server(ctx).await,
        _ => unreachable!("Clap would ensure that we cannot get here"),
    };
}
