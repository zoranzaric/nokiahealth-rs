- Pass database connection data to `nokiahealth::storage::influxdb::create_client`  
  Getting this right is one of those "Learing Rust brickwalls"...
- Add an optional name/person field to `Weight`
- Add the measurement to the `Weight`  
  My problem with this is having an `InfluxDB` implementation detail leak into `nokiahealth::data`.
- Add a `README.md` and `README.tpl` and find out how jonhoo's
  [tsunami](https://github.com/jonhoo/tsunami) does it.  
  jonhoo did it on stream.