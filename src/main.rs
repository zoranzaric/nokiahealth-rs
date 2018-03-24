extern crate chrono;
extern crate csv;
extern crate influent;

use std::error::Error;
use std::path::Path;
use std::process;

extern crate nokiahealth;

extern crate log;
extern crate loggerv;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

fn read_weight(path: &Path) -> Result<(), Box<Error>> {
    let client = nokiahealth::storage::influxdb::connect();

    let weights = nokiahealth::data::weight::read_weights_from_path(path);

    nokiahealth::storage::influxdb::write_weights(&client, weights);

    Ok(())
}

fn main() {
    let matches = App::new("nokiahealth")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
                .multiple(true),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .help("Sets the InfluxDB host to use")
                .default_value("localhost")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("Sets the InfluxDB database to use")
                .default_value("8086")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("username")
                .long("username")
                .help("Sets the InfluxDB username to use")
                .default_value("root")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("password")
                .long("password")
                .help("Sets the InfluxDB password to use")
                .default_value("root")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("database")
                .long("database")
                .help("Sets the InfluxDB database to use")
                .default_value("health")
                .takes_value(true)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("weight")
                .about("Imports weights from a Nokia Health CSV file to an InfluxDB")
                .version(crate_version!())
                .author(crate_authors!())
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    loggerv::Logger::new()
        .verbosity(matches.occurrences_of("v"))
        .level(true)
        .line_numbers(true)
        .colors(true)
        .init()
        .unwrap();

    match matches.subcommand_name() {
        Some("weight") => {
            let matches = matches.subcommand_matches("weight").unwrap();

            let input_path = Path::new(matches.value_of("INPUT").unwrap());

            if let Err(err) = read_weight(&input_path) {
                println!("error running read_weight: {}", err);
                process::exit(1);
            }
        }
        None => {
            eprintln!("No subcommand");
        }
        _ => {
            eprintln!("Unknown subcommand");
        }
    }
}
