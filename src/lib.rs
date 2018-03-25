//! `nokiahealth` is a crate that helps manage
//! [NOKIA Health](https://health.nokia.com) data.
//! 
//! Currently the supported data is `weight` only.  The only supported storage
//! is [`InfluxDB`](https://www.influxdata.com/time-series-platform/influxdb/).
#[macro_use]
extern crate serde_derive;

extern crate chrono;

extern crate csv;

extern crate serde;
extern crate serde_json;

extern crate stringreader;

extern crate influent;

pub mod storage;
pub mod data;
