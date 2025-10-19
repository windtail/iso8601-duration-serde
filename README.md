# iso8601-duration-serde

[![Crates.io](https://img.shields.io/crates/v/iso8601-duration-serde)](https://crates.io/crates/iso8601-duration-serde)
[![License](https://img.shields.io/crates/l/iso8601-duration-serde)](LICENSE)

A Rust library for serializing and deserializing [`time::Duration`](https://crates.io/crates/time) using the ISO 8601 format.

## Features

This library provides a simple way to serialize and deserialize [`time::Duration`](https://docs.rs/time/latest/time/struct.Duration.html) types using the widely supported [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).

Examples:
- `P1D` represents 1 day
- `PT1H` represents 1 hour
- `PT30M` represents 30 minutes
- `PT45S` represents 45 seconds
- `P2DT3H30M15S` represents 2 days, 3 hours, 30 minutes, and 15 seconds

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
iso8601-duration-serde = "0.1"
```

## Example

```rust
use serde::{Serialize, Deserialize};
use time::Duration;

#[derive(Serialize, Deserialize)]
struct Example {
    #[serde(with = "iso8601_duration_serde")]
    duration: Duration,
}

// Serialization
let example = Example {
    duration: Duration::days(1) + Duration::hours(2) + Duration::minutes(30),
};

let json = serde_json::to_string(&example)?;
// json will be: {"duration":"P1DT2H30M"}

// Deserialization
let json = r#"{"duration":"PT45S"}"#;
let example: Example = serde_json::from_str(json)?;
// example.duration will be Duration::seconds(45)
```

## Limitations

- Year and month durations are not supported as they are not fixed-length in `time::Duration`
- Attempting to deserialize a duration with years or months will result in an error

## Dependencies

- [time](https://crates.io/crates/time) - Time handling library
- [serde](https://crates.io/crates/serde) - Serialization framework
- [iso8601-duration](https://crates.io/crates/iso8601-duration) - ISO 8601 duration parsing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.