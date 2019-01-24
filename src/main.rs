extern crate ieql;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate ron;
extern crate reqwest;
extern crate rusoto_s3;
extern crate flate2;
extern crate warc_parser;
extern crate nom;
extern crate httparse;
extern crate env_logger;
extern crate sys_info;

use std::net::SocketAddr;

use clap::{App, Arg, SubCommand};

use ieql::query::query::{Query, QueryGroup};
use std::path::Path;

mod client;
mod master;
mod util;

fn main() {
    env_logger::init();

    let matches = App::new("MIEQL Command Line Interface")
        .version(crate_version!())
        .about("S3 and container bindings for IEQL.")
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("client")
                .about("Act as the client")
                .args_from_usage("-t, --threads=[# of threads] 'The number of threads to use (default 8)'")
                .args_from_usage("-m, --master=[master url] 'The url of the master (default <http://0.0.0.0:3000>)'")
                .args_from_usage("-q, --queue=[max queue size] 'Maximum number of items in the queue at any given time (default 256)'")
        )
        .subcommand(
            SubCommand::with_name("master")
                .about("Act as the master")
                .args_from_usage("-b, --bind=[bind address] 'The network address to bind to (0.0.0.0:3000 by default)'")
                .args_from_usage("-d, --debug 'Always return a single url to be scanned'"),
        )
        .get_matches();
    run(matches);
}

fn run(matches: clap::ArgMatches) {
    match matches.subcommand() {
        ("client", Some(m)) => {
            let master_url = m.value_of("master").unwrap_or("http://0.0.0.0:3000");
            let threads: u8 = match m.value_of("threads").unwrap_or("8").parse() {
                Ok(value) => value,
                Err(error) => {
                    error!("invalid number of threads `{}` (`{}`)!", m.value_of("threads").unwrap(), error);
                    return;
                }
            };
            let queue_size: usize = match m.value_of("queue").unwrap_or("256").parse() {
                Ok(value) => value,
                Err(error) => {
                    error!("invalid max queue size `{}` (`{}`)!", m.value_of("queue").unwrap(), error);
                    return;
                }
            };
            client::main(String::from(master_url), threads, queue_size);
        }
        ("master", Some(m)) => {
            let bind_address: SocketAddr = m
                .value_of("bind")
                .unwrap_or("0.0.0.0:3000")
                .parse()
                .expect("invalid bind address provided");
            let debug = m.is_present("debug");
            master::main(bind_address, debug);
        }
        _ => error!("no valid command specified; try running with `--help`."),
    }
}
