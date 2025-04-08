use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let ignore_case = env::var("APP_IGNORE_CASE").is_ok();

    let config = Config::build(&args, ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Problem running application: {err}");
        process::exit(1);
    })
}
