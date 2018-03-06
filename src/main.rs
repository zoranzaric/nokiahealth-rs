extern crate chrono;
extern crate csv;
extern crate influent;

use std::error::Error;
use std::io;
use std::process;

use chrono::prelude::*;

use csv::StringRecord;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};

fn example() -> Result<(), Box<Error>> {
    let credentials = Credentials {
        username: "root",
        password: "root",
        database: "test"
    };
    let hosts = vec!["http://localhost:8086"];
    // let client = create_client(credentials, hosts);
    let client = connect();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quote(b'"')
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);

        let m = convert_weight_record_to_measurement(record);
        println!("{:?}", m);
        client.write_one(m, None);

    }
    Ok(())
}

fn convert_weight_record_to_measurement(record: StringRecord) -> Measurement<'static> {
        println!("{:?}", record);

        let weight: Option<f64> = match record.get(1) {
            Option::None => None,
            Option::Some(s) =>
                match s.parse() {
                    Result::Err(_) => None,
                    Result::Ok(weight) => Some(weight),
                }
        };
        let fat: Option<f64> = match record.get(2) {
            Option::None => None,
            Option::Some(s) =>
                match s.parse() {
                    Result::Err(_) => None,
                    Result::Ok(fat) => Some(fat),
                }
        };


        let local: NaiveDateTime = NaiveDateTime::parse_from_str(record.get(0).unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

        let mut m = Measurement::new("weight");
        m.set_timestamp(local.timestamp());
        m.add_tag("name", "Zoran");
        if (weight.is_some()) {
            m.add_field("weight", Value::Float(weight.unwrap()));
        };
        if (fat.is_some()) {
            m.add_field("fat", Value::Float(fat.unwrap()));
        };
        return m;
}


fn connect<'a>() -> HttpClient<'a> {
    let credentials = Credentials {
        username: "root",
        password: "root",
        database: "test"
    };
    let hosts = vec!["http://localhost:8086"];
    let client = create_client(credentials, hosts);
    client
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
