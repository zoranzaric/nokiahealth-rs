use chrono::prelude::*;
use csv;
use std::io::Read;
use std::path::Path;

/// A weight [kg] datapoint with optional fat mass [kg].
#[derive(Serialize, Deserialize, Debug)]
pub struct Weight {
    #[serde(with = "my_date_format", rename = "Date")] pub date: NaiveDateTime,
    #[serde(rename = "Gewicht")] pub weight: f64,
    #[serde(rename = "Fettmasse")] pub fat: Option<f64>,
}

/// A function to read a weight vector from a given path.
pub fn read_weights_from_path(path: &Path) -> Vec<Weight>
{
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

    rdr.deserialize()
        .map(|record| record.unwrap())
        .map(Weight::from)
        .collect()
}

/// A function to read a weight vector from a given `std::io::Read`.
/// 
/// # Example
/// 
///```rust,ignore
///let example = r#"Date,Gewicht,Fettmasse,Knochenmasse,Muskelmasse,Wasseranteil,Kommentare
///"2018-03-03 08:47:03",80.03,19.54,,,,
///"#;
///let string_reader = StringReader::new(example);
///
///let weights = nokiahealth::data::weight::read_weights_from_reader(string_reader);
///```
pub fn read_weights_from_reader<R>(r: R) -> Vec<Weight>
where
    R: Read,
{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quote(b'"')
        .from_reader(r);
    rdr.deserialize()
        .map(|record| record.unwrap())
        .map(Weight::from)
        .collect()
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

#[cfg(test)]
mod tests {
    use stringreader::StringReader;
    use data::weight;

    #[test]
    fn simple_example_should_be_parsed() {
        let example = r#"Date,Gewicht,Fettmasse,Knochenmasse,Muskelmasse,Wasseranteil,Kommentare
"2018-03-03 08:47:03",80.03,19.54,,,,
"#;
        let string_reader = StringReader::new(example);

        let weights = weight::read_weights_from_reader(string_reader);

        assert_eq!(1, weights.len());
        assert_eq!(80.03, weights[0].weight);
        assert_eq!(Some(19.54), weights[0].fat);
    }
}
