# Itelem

Provides parsing of iRacing's `.ibt` telemetry files. It's based on the excellent javascript library [ibt-telemetry](https://github.com/SkippyZA/ibt-telemetry).

# Usage

To use `itelem`, add the following to your `Cargo.toml`:

```toml
[dependencies]
itelem = "0.1"
```

or run this command in your project root:

```
$ cargo add itelem@0.1
```

Pass in a file or anything that implements `Read + Seek` and you can access header information as well as `weekend_info` and the samples that contain the most interesting information.
```rust
let file = File::open("./sting.ibt").unwrap();
let mut reader = IbtReader::new(Box::new(file));
assert_eq!(reader.header.tick_rate, 60);

let weekend_info = &reader.session_info.weekend_info;
assert_eq!(weekend_info.track_name, "spielberg gp");

let rpm = reader.find_var("RPM".to_string()).unwrap();
let samples: Vec<Sample> = reader.samples().collect();

// There are 3371 samples and with a 60 tick tick_rate
// meaning that the telemetry file contains 56 seconds of data
assert_eq!(samples.len(), 3371);
let first_sample = samples[1001].get_by_header(&rpm).unwrap();
assert_eq!(first_sample, SampleValue::Float32(991.8974));
```
