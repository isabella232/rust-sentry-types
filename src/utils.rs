use chrono::{DateTime, TimeZone, Utc};

/// Converts a datetime object into a float timestamp.
pub fn datetime_to_timestamp(dt: &DateTime<Utc>) -> f64 {
    if dt.timestamp_subsec_nanos() == 0 {
        dt.timestamp() as f64
    } else {
        (dt.timestamp() as f64) + ((dt.timestamp_subsec_micros() as f64) / 1_000_000f64)
    }
}

pub fn timestamp_to_datetime(ts: f64) -> DateTime<Utc> {
    let secs = ts as i64;
    let micros = (ts.fract() * 1_000_000f64) as u32;
    Utc.timestamp_opt(secs, micros * 1000).unwrap()
}

pub mod ts_seconds_float {
    use std::fmt;
    use serde::{de, ser};
    use chrono::{DateTime, TimeZone, Utc};

    use super::timestamp_to_datetime;

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_any(SecondsTimestampVisitor)
            .map(|dt| dt.with_timezone(&Utc))?)
    }

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if dt.timestamp_subsec_nanos() == 0 {
            serializer.serialize_i64(dt.timestamp())
        } else {
            serializer.serialize_f64(
                (dt.timestamp() as f64) + ((dt.timestamp_subsec_micros() as f64) / 1_000_000f64),
            )
        }
    }

    struct SecondsTimestampVisitor;

    impl<'de> de::Visitor<'de> for SecondsTimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a unix timestamp")
        }

        fn visit_f64<E>(self, value: f64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(timestamp_to_datetime(value))
        }

        fn visit_i64<E>(self, value: i64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(Utc.timestamp_opt(value, 0).unwrap())
        }

        fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(Utc.timestamp_opt(value as i64, 0).unwrap())
        }
    }
}