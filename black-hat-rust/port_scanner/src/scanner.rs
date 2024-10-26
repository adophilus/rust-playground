use crate::model::PORTS_TO_SCAN;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Thread error: {0}")]
    ThreadError(#[from] rayon::ThreadPoolBuildError),

    #[error("Http error occurred: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Usage: port_scanner <target>")]
    CliError,
}

#[derive(Debug)]
pub struct Subdomain {
    pub subdomain: String,
}

#[derive(Debug)]
pub struct ScannedSubdomain {
    pub subdomain: String,
    pub ports: Vec<ScannedSubdomainPort>,
}

#[derive(Debug)]
pub enum ScannedPortStatus {
    Opened,
    Closed,
    Timedout,
    ResolutionFailed,
}

#[derive(Debug)]
pub struct ScannedSubdomainPort {
    pub port: u16,
    pub status: ScannedPortStatus,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
struct ApiEntryResponse {
    common_name: String,
    name_value: String,
}

pub fn enumerate_subdomains(target: String) -> Result<Vec<Subdomain>> {
    let entries: Vec<ApiEntryResponse> =
        // reqwest::blocking::get(format!("https://crt.sh/?q%25.{}&output=json", target))?.json()?;
        reqwest::blocking::get("http://localhost:3000")?.json()?;

    let subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            let mut subdomains = entry
                .name_value
                .split('\n')
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();
            subdomains.push(entry.common_name);
            subdomains
        })
        .flatten()
        .filter(|entry| *entry != target || entry.contains('*'))
        .collect();

    let subdomain_objects: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|s| Subdomain { subdomain: s })
        .collect();

    Ok(subdomain_objects)
}

pub fn scan_subdomains(subdomains: Vec<Subdomain>) -> Vec<ScannedSubdomain> {
    subdomains
        .into_par_iter()
        .map(
            |subdomain| match format!("{}:1024", subdomain.subdomain).to_socket_addrs() {
                Ok(socket_addrs) => {
                    let socket_addrs: Vec<SocketAddr> = socket_addrs.collect();
                    ScannedSubdomain {
                        subdomain: subdomain.subdomain.clone(),
                        ports: PORTS_TO_SCAN
                            .into_par_iter()
                            .map(|p| {
                                let socket_addr = SocketAddr::new(socket_addrs[0].ip(), p);
                                scan_port(subdomain.subdomain.clone(), &socket_addr)
                            })
                            .collect::<Vec<ScannedSubdomainPort>>(),
                    }
                }
                _ => ScannedSubdomain {
                    subdomain: subdomain.subdomain,
                    ports: PORTS_TO_SCAN
                        .into_iter()
                        .map(|p| ScannedSubdomainPort {
                            status: ScannedPortStatus::ResolutionFailed,
                            port: p,
                        })
                        .collect::<Vec<ScannedSubdomainPort>>(),
                },
            },
        )
        .collect()
}

fn scan_port(domain: String, addr: &SocketAddr) -> ScannedSubdomainPort {
    println!("->> Scanning {}:{} on {}", domain, addr.port(), addr);
    match TcpStream::connect_timeout(addr, Duration::from_secs(10)) {
        Ok(_) => {
            println!("<<- Scanned successfully!");
            ScannedSubdomainPort {
                status: ScannedPortStatus::Opened,
                port: addr.port(),
            }
        }
        Err(e) => {
            println!("<<- Failed to scan for some reason: {}", e);
            ScannedSubdomainPort {
                status: ScannedPortStatus::Closed,
                port: addr.port(),
            }
        }
    }
}
