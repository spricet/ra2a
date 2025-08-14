use chrono::{DateTime, TimeZone, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Deserializer, Serializer};

use time::OffsetDateTime;
use time::format_description::well_known::Iso8601;

pub mod iso8601_timestamp {
    use super::*;
    pub fn serialize<S>(ts: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt = Utc
            .timestamp_opt(ts.seconds, ts.nanos as u32)
            .single()
            .ok_or_else(|| serde::ser::Error::custom("invalid timestamp"))?;
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        iso8601_to_timestamp(&s).map_err(serde::de::Error::custom)
    }
}

pub mod iso8601_timestamp_opt {
    use super::*;
    pub fn serialize<S>(ts: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match ts {
            Some(ts) => iso8601_timestamp::serialize(ts, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => iso8601_to_timestamp(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

fn iso8601_to_timestamp(s: &str) -> Result<Timestamp, String> {
    // 1️⃣ Try RFC 3339 (chrono)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        let dt_utc = dt.with_timezone(&Utc);
        return Ok(Timestamp {
            seconds: dt_utc.timestamp(),
            nanos: dt_utc.timestamp_subsec_nanos() as i32,
        });
    }

    // 2️⃣ Try full ISO 8601 (time crate)
    if let Ok(dt) = OffsetDateTime::parse(s, &Iso8601::PARSING) {
        return Ok(Timestamp {
            seconds: dt.unix_timestamp(),
            nanos: dt.nanosecond() as i32,
        });
    }

    Err(format!("Invalid ISO 8601 timestamp: {}", s))
}
