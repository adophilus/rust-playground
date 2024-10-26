mod model;
mod scanner;

use rayon::ThreadPoolBuilder;
use scanner::{enumerate_subdomains, scan_subdomains, Error, Result};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        return Err(Error::CliError);
    }

    let target = args[1].clone();

    let threadpool = ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .map_err(Error::ThreadError)?;

    let subdomains = enumerate_subdomains(target.clone())?;
    println!("Found {} subdomains on {}!", subdomains.len(), target);
    println!("Scanning...");

    threadpool.install(|| {
        scan_subdomains(subdomains);
    });

    Ok(())
}
