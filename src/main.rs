extern crate chrono;
extern crate csv;
extern crate influent;

use std::error::Error;
use std::path::Path;
use std::process;

use chrono::prelude::*;

use csv::StringRecord;

#[macro_use]
extern crate log;
extern crate loggerv;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};

fn read_weight(path: &Path) -> Result<(), Box<Error>> {
    let client = connect();

    let mut rdr = match csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quote(b'"')
        .from_path(path)
    {
        Ok(rdr) => rdr,
        Err(e) => {
            panic!("{}", e);
        }
    };
    for result in rdr.records() {
        let record = result?;

        let m = convert_weight_record_to_measurement(record);
        debug!("{:?}", m);
        match client.write_one(m, None) {
            Ok(_) => {}
            Err(e) => error!("{:?}", e),
        }
    }
    Ok(())
}

fn convert_weight_record_to_measurement(record: StringRecord) -> Measurement<'static> {
    debug!("{:?}", record);

    let weight: Option<f64> = match record.get(1) {
        Option::None => None,
        Option::Some(s) => match s.parse() {
            Result::Err(_) => None,
            Result::Ok(weight) => Some(weight),
        },
    };
    let fat: Option<f64> = match record.get(2) {
        Option::None => None,
        Option::Some(s) => match s.parse() {
            Result::Err(_) => None,
            Result::Ok(fat) => Some(fat),
        },
    };

    let local: NaiveDateTime =
        NaiveDateTime::parse_from_str(record.get(0).unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

    let mut m = Measurement::new("weight");
    m.set_timestamp(local.timestamp());
    m.add_tag("name", "Zoran");
    match weight {
        Some(weight) => {
            m.add_field("weight", Value::Float(weight));
        }
        None => {}
    };
    match fat {
        Some(fat) => {
            m.add_field("fat", Value::Float(fat));
        }
        None => {}
    };
    return m;
}

fn connect<'a>() -> HttpClient<'a> {
    let credentials = Credentials {
        username: "root",
        password: "root",
        database: "health",
    };
    let hosts = vec!["http://localhost:8086"];
    let client = create_client(credentials, hosts);
    client
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
