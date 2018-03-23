#[macro_use]
extern crate serde_derive;

extern crate chrono;

extern crate csv;

extern crate serde;
extern crate serde_json;

extern crate stringreader;

mod weight {
    use chrono::prelude::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Weight {
        #[serde(with = "my_date_format", rename = "Date")] pub date: NaiveDateTime,
        #[serde(rename = "Gewicht")] pub weight: Option<f64>,
        #[serde(rename = "Fettmasse")] pub fat: Option<f64>,
    }

    mod my_date_format {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod weight_tests {
    use csv;
    use stringreader::StringReader;
    use weight;

    #[test]
    fn simple_example_should_be_parsed() {
        let example = r#"Date,Gewicht,Fettmasse,Knochenmasse,Muskelmasse,Wasseranteil,Kommentare
"2018-03-03 08:47:03",80.03,19.54,,,,
"#;
        let string_reader = StringReader::new(example);

        let mut rdr = csv::Reader::from_reader(string_reader);

        let mut weights: Vec<weight::Weight> = Vec::new();

        for result in rdr.deserialize() {
            let record: weight::Weight = result.unwrap();
            weights.push(record);
        }

        assert_eq!(1, weights.len());
        assert_eq!(Some(80.03), weights[0].weight)
    }
}
