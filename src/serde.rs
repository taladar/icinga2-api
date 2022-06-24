//! Custom deserializers for various parts of the Icinga API results

use serde::Deserialize;

/// deserializes a unix timestamp with sub second accuracy
/// (usually 6 digits after the decimal point for icinga)
///
/// # Errors
///
/// returns an error if the value can not be parsed as an f64
/// or if it can not be converted from a unix timestamp to a
/// [time::OffsetDateTime]
pub fn deserialize_icinga_timestamp<'de, D>(
    deserializer: D,
) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f: f64 = f64::deserialize(deserializer)?;

    let i = (f * 1_000_000_000f64) as i128;

    time::OffsetDateTime::from_unix_timestamp_nanos(i).map_err(serde::de::Error::custom)
}

/// deserializes an optional unix timestamp with sub second accuracy
/// (usually 6 digits after the decimal point for icinga)
/// if the value is 0 return None
///
/// # Errors
///
/// returns an error if the value can not be parsed as an f64
/// or if it can not be converted from a unix timestamp to a
/// [time::OffsetDateTime]
pub fn deserialize_optional_icinga_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<time::OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f: f64 = f64::deserialize(deserializer)?;

    if f == 0.0f64 {
        Ok(None)
    } else {
        let i = (f * 1_000_000_000f64) as i128;

        Ok(Some(
            time::OffsetDateTime::from_unix_timestamp_nanos(i).map_err(serde::de::Error::custom)?,
        ))
    }
}

/// deserialize an optional String where None is represented as
/// an empty string
///
/// # Errors
///
/// returns an error if the value can not be interpreted as null or a String
pub fn deserialize_empty_string_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    } else {
        Ok(None)
    }
}

/// deserialize an optional value with a FromStr implementation where None is represented as
/// an empty string
///
/// # Errors
///
/// returns an error if the value can not be interpreted as null or a String
pub fn deserialize_empty_string_or_parse<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.parse().map_err(serde::de::Error::custom)?))
        }
    } else {
        Ok(None)
    }
}

/// deserialize an integer as a time::Duration where the integer represents seconds
///
/// # Errors
///
/// returns an error if the value can not be parsed as an integer
pub fn deserialize_seconds_as_duration<'de, D>(deserializer: D) -> Result<time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i: i64 = i64::deserialize(deserializer)?;
    Ok(time::Duration::seconds(i))
}

/// deserialize an integer as a time::Duration where the integer represents seconds
///
/// # Errors
///
/// returns an error if the value can not be interpreted as null or an integer
pub fn deserialize_optional_seconds_as_duration<'de, D>(
    deserializer: D,
) -> Result<Option<time::Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i: Option<i64> = Option::deserialize(deserializer)?;
    if let Some(i) = i {
        Ok(Some(time::Duration::seconds(i)))
    } else {
        Ok(None)
    }
}
