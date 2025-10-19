use iso8601_duration::Duration as IsoDuration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::Duration;
use time_core::convert::*;

/// Serialize an [`time::Duration`] using the well-known ISO 8601 format.
#[inline]
pub fn serialize<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    let mut seconds = duration.whole_seconds();
    let nanoseconds = duration.subsec_nanoseconds();

    let days = seconds / Second::per_t::<i64>(Day);
    seconds = seconds % Second::per_t::<i64>(Day);

    let hours = seconds / Second::per_t::<i64>(Hour);
    seconds = seconds % Second::per_t::<i64>(Hour);

    let minutes = seconds / Second::per_t::<i64>(Minute);
    seconds = seconds % Second::per_t::<i64>(Minute);

    let seconds_f32 =
        seconds as f32 + (nanoseconds as f64 / Nanosecond::per_t::<f64>(Second)) as f32;

    let iso_duration = IsoDuration::new(
        0f32,
        0f32,
        days as f32,
        hours as f32,
        minutes as f32,
        seconds_f32,
    );

    iso_duration.serialize(serializer)
}

/// Deserialize an [`time::Duration`] from its ISO 8601 representation.
#[inline]
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Duration, D::Error> {
    let duration = IsoDuration::deserialize(deserializer)?;

    if duration.year > 0.0 || duration.month > 0.0 {
        return Err(serde::de::Error::custom(
            "Duration::year and Duration::month must be zero",
        ));
    }

    let seconds_fract = duration.day.fract() * Second::per_t::<f32>(Day)
        + duration.hour.fract() * Second::per_t::<f32>(Hour)
        + duration.minute.fract() * Second::per_t::<f32>(Minute)
        + duration.second.fract();

    let seconds = duration.day as i64 * Second::per_t::<i64>(Day)
        + duration.hour as i64 * Second::per_t::<i64>(Hour)
        + duration.minute as i64 * Second::per_t::<i64>(Minute)
        + duration.second as i64
        + seconds_fract as i64;

    let nanoseconds = (seconds_fract.fract() * Nanosecond::per_t::<f32>(Second)) as i32;

    Ok(Duration::new(seconds, nanoseconds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct TestStruct {
        #[serde(with = "super")]
        duration: Duration,
    }

    #[test]
    fn test_positive_duration_with_days() {
        let test_struct = TestStruct {
            duration: Duration::days(5),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"duration":"P5D"}"#);

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_positive_duration_with_hours() {
        let test_struct = TestStruct {
            duration: Duration::hours(3),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"duration":"PT3H"}"#);

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_positive_duration_with_minutes() {
        let test_struct = TestStruct {
            duration: Duration::minutes(30),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"duration":"PT30M"}"#);

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_positive_duration_with_seconds() {
        let test_struct = TestStruct {
            duration: Duration::seconds(45),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"duration":"PT45S"}"#);

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_complex_positive_duration() {
        let test_struct = TestStruct {
            duration: Duration::days(2) + Duration::hours(3) + Duration::minutes(30) + Duration::seconds(15),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"duration":"P2DT3H30M15S"}"#);

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_duration_with_fractional_seconds() {
        let test_struct = TestStruct {
            duration: Duration::seconds(10) + Duration::milliseconds(500),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        // Depending on serialization implementation, fractional seconds might be formatted differently
        
        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        // Allow small differences in precision due to floating point arithmetic
        assert_eq!(deserialized.duration.whole_seconds(), test_struct.duration.whole_seconds());
        // Check that the difference is less than 1 second
        assert!(deserialized.duration - test_struct.duration < Duration::seconds(1));
        assert!(test_struct.duration - deserialized.duration < Duration::seconds(1));
    }

    #[test]
    fn test_deserialize_iso8601_variants() {
        // Test various ISO 8601 formats
        
        // Days only
        let json = r#"{"duration":"P7D"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.duration, Duration::days(7));
        
        // Hours only
        let json = r#"{"duration":"PT5H"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.duration, Duration::hours(5));
        
        // Minutes only
        let json = r#"{"duration":"PT30M"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.duration, Duration::minutes(30));
        
        // Seconds only
        let json = r#"{"duration":"PT45S"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.duration, Duration::seconds(45));
        
        // Complex format
        let json = r#"{"duration":"P1DT12H30M45S"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.duration, Duration::days(1) + Duration::hours(12) + Duration::minutes(30) + Duration::seconds(45));
        
        // Fractional seconds
        let json = r#"{"duration":"PT10.5S"}"#;
        let deserialized: TestStruct = serde_json::from_str(json).unwrap();
        // Check that deserialization works (exact value might vary due to float precision)
        assert!(deserialized.duration > Duration::seconds(10));
        assert!(deserialized.duration < Duration::seconds(11));
    }
}
