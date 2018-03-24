//! The `storage` module contains all the storage handling.  Currently only
//! the `influxdb` module.

// The `influxdb` module contains all the code to write to a InfluxDB database.
pub mod influxdb {
    use influent::create_client;
    use influent::client::{Client, Credentials};
    use influent::client::http::HttpClient;
    use influent::measurement::{Measurement, Value};

    use data::weight::Weight;

    pub struct ConnectionData {
        username: String,
        password: String,
        database: String,
        host: String,
        port: String,
    }

    impl<'a> From<Weight> for Measurement<'a> {
        fn from(weight: Weight) -> Self {
            // TODO don't hardcode measurement
            let mut m = Measurement::new("weight");
            m.set_timestamp(weight.date.timestamp());
            // TODO don't hardcode name
            m.add_tag("name", "Zoran");

            m.add_field("weight", Value::Float(weight.weight));

            if let Some(fat) = weight.fat {
                m.add_field("fat", Value::Float(fat));
            };

            m
        }
    }

    pub fn connect<'a>() -> HttpClient<'a> {
        let credentials = Credentials {
            username: "root",
            password: "root",
            database: "health",
        };
        let hosts = vec!["http://localhost:8086"];
        create_client(credentials, hosts)
    }

    pub fn write_weights<C>(client: &C, weights: Vec<Weight>)
    where
        C: Client,
    {
        for weight in weights {
            let m: Measurement = weight.into();
        
        match client.write_one(m, None) {
                Ok(_) => {}
                Err(e) => println!("{:?}", e),
            };
        }
    }

    #[cfg(test)]
    mod tests {
        use data::weight::Weight;

        use chrono::prelude::*;
        use influent::measurement::{Measurement, Value};

        #[test]
        fn converting_a_weight_should_work() {
            let weight = Weight {
                date: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                weight: 42.0,
                fat: Some(6.3),
            };

            let m: Measurement = weight.into();

            if let Some(&Value::Float(actual_weight)) = m.fields.get("weight") {
                assert_eq!(42.0, actual_weight);
            } else {
                // TODO
            };
        }
    }
}
