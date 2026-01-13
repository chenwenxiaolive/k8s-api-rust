//! Time types for Kubernetes API.
//!
//! This module contains time-related types that match the Kubernetes apimachinery
//! time types: `Time`, `MicroTime`, and `Duration`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::Duration as StdDuration;

/// Time is a wrapper around DateTime<Utc> that serializes to RFC 3339 format.
///
/// This corresponds to `metav1.Time` in Kubernetes which represents timestamps
/// in API objects. The format is RFC 3339 with second-level precision.
///
/// # Example
///
/// ```
/// use k8s_api_meta::Time;
///
/// let time = Time::now();
/// let json = serde_json::to_string(&time).unwrap();
/// // Produces: "2024-01-15T10:30:00Z"
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(pub DateTime<Utc>);

impl Time {
    /// Creates a new Time with the current UTC time.
    pub fn now() -> Self {
        Time(Utc::now())
    }

    /// Creates a Time from a DateTime<Utc>.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Time(dt)
    }

    /// Returns the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    /// Returns a reference to the inner DateTime<Utc>.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Returns true if this time is zero (Unix epoch).
    pub fn is_zero(&self) -> bool {
        self.0.timestamp() == 0 && self.0.timestamp_subsec_nanos() == 0
    }
}

impl Default for Time {
    fn default() -> Self {
        Time(DateTime::UNIX_EPOCH)
    }
}

impl From<DateTime<Utc>> for Time {
    fn from(dt: DateTime<Utc>) -> Self {
        Time(dt)
    }
}

impl From<Time> for DateTime<Utc> {
    fn from(t: Time) -> Self {
        t.0
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Kubernetes Time uses RFC 3339 format with second precision
        serializer.serialize_str(&self.0.format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Parse RFC 3339 format
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| Time(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom)
    }
}

/// MicroTime is a wrapper around DateTime<Utc> with microsecond precision.
///
/// This corresponds to `metav1.MicroTime` in Kubernetes which represents
/// timestamps with microsecond precision. The serialization format is
/// RFC 3339 with 6 decimal places for fractional seconds.
///
/// # Example
///
/// ```
/// use k8s_api_meta::MicroTime;
///
/// let time = MicroTime::now();
/// let json = serde_json::to_string(&time).unwrap();
/// // Produces: "2024-01-15T10:30:00.123456Z"
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicroTime(pub DateTime<Utc>);

impl MicroTime {
    /// Creates a new MicroTime with the current UTC time.
    pub fn now() -> Self {
        MicroTime(Utc::now())
    }

    /// Creates a MicroTime from a DateTime<Utc>.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        MicroTime(dt)
    }

    /// Returns the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    /// Returns a reference to the inner DateTime<Utc>.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Returns true if this time is zero (Unix epoch).
    pub fn is_zero(&self) -> bool {
        self.0.timestamp() == 0 && self.0.timestamp_subsec_nanos() == 0
    }
}

impl Default for MicroTime {
    fn default() -> Self {
        MicroTime(DateTime::UNIX_EPOCH)
    }
}

impl From<DateTime<Utc>> for MicroTime {
    fn from(dt: DateTime<Utc>) -> Self {
        MicroTime(dt)
    }
}

impl From<MicroTime> for DateTime<Utc> {
    fn from(t: MicroTime) -> Self {
        t.0
    }
}

impl From<Time> for MicroTime {
    fn from(t: Time) -> Self {
        MicroTime(t.0)
    }
}

impl fmt::Display for MicroTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
    }
}

impl Serialize for MicroTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Kubernetes MicroTime uses RFC 3339 format with microsecond precision
        serializer.serialize_str(&self.0.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string())
    }
}

impl<'de> Deserialize<'de> for MicroTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Parse RFC 3339 format with fractional seconds
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| MicroTime(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom)
    }
}

/// Duration is a wrapper around std::time::Duration that serializes to Go duration format.
///
/// This corresponds to `metav1.Duration` in Kubernetes. It serializes to strings
/// like "1h30m", "500ms", "2s".
///
/// # Format
///
/// The duration string is a sequence of decimal numbers, each with optional
/// fraction and a unit suffix:
/// - `ns` - nanoseconds
/// - `us` or `µs` - microseconds
/// - `ms` - milliseconds
/// - `s` - seconds
/// - `m` - minutes
/// - `h` - hours
///
/// # Example
///
/// ```
/// use k8s_api_meta::Duration;
/// use std::time::Duration as StdDuration;
///
/// let dur = Duration::from_secs(3661); // 1h1m1s
/// let json = serde_json::to_string(&dur).unwrap();
/// assert_eq!(json, "\"1h1m1s\"");
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub StdDuration);

impl Duration {
    /// Creates a new Duration from seconds.
    pub const fn from_secs(secs: u64) -> Self {
        Duration(StdDuration::from_secs(secs))
    }

    /// Creates a new Duration from milliseconds.
    pub const fn from_millis(millis: u64) -> Self {
        Duration(StdDuration::from_millis(millis))
    }

    /// Creates a new Duration from a std::time::Duration.
    pub const fn from_std(d: StdDuration) -> Self {
        Duration(d)
    }

    /// Returns the duration as seconds.
    pub const fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Returns the duration as milliseconds.
    pub const fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Returns the inner std::time::Duration.
    pub const fn into_inner(self) -> StdDuration {
        self.0
    }

    /// Returns true if this duration is zero.
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Formats the duration as a Go duration string.
    fn format_go_duration(&self) -> String {
        if self.0.is_zero() {
            return "0s".to_string();
        }

        let total_nanos = self.0.as_nanos();
        let mut result = String::new();

        let hours = total_nanos / 3_600_000_000_000;
        let remaining = total_nanos % 3_600_000_000_000;
        let minutes = remaining / 60_000_000_000;
        let remaining = remaining % 60_000_000_000;
        let seconds = remaining / 1_000_000_000;
        let remaining = remaining % 1_000_000_000;
        let millis = remaining / 1_000_000;
        let remaining = remaining % 1_000_000;
        let micros = remaining / 1_000;
        let nanos = remaining % 1_000;

        if hours > 0 {
            result.push_str(&format!("{}h", hours));
        }
        if minutes > 0 {
            result.push_str(&format!("{}m", minutes));
        }
        if seconds > 0 {
            result.push_str(&format!("{}s", seconds));
        }
        if millis > 0 {
            result.push_str(&format!("{}ms", millis));
        }
        if micros > 0 {
            result.push_str(&format!("{}µs", micros));
        }
        if nanos > 0 {
            result.push_str(&format!("{}ns", nanos));
        }

        if result.is_empty() {
            "0s".to_string()
        } else {
            result
        }
    }
}

impl From<StdDuration> for Duration {
    fn from(d: StdDuration) -> Self {
        Duration(d)
    }
}

impl From<Duration> for StdDuration {
    fn from(d: Duration) -> Self {
        d.0
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_go_duration())
    }
}

/// Error when parsing a duration string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDurationError(String);

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid duration: {}", self.0)
    }
}

impl std::error::Error for ParseDurationError {}

impl FromStr for Duration {
    type Err = ParseDurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_go_duration(s)
    }
}

/// Parses a Go duration string.
fn parse_go_duration(s: &str) -> Result<Duration, ParseDurationError> {
    let s = s.trim();
    if s.is_empty() || s == "0" {
        return Ok(Duration::default());
    }

    let mut total_nanos: u128 = 0;
    let mut chars = s.chars().peekable();

    // Handle negative durations (Go supports them, but they're uncommon in K8s)
    let negative = chars.peek() == Some(&'-');
    if negative {
        chars.next();
    }

    while chars.peek().is_some() {
        // Parse the number (including fractional part)
        let mut num_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if num_str.is_empty() {
            return Err(ParseDurationError(format!("invalid duration: {}", s)));
        }

        let num: f64 = num_str
            .parse()
            .map_err(|_| ParseDurationError(format!("invalid number: {}", num_str)))?;

        // Parse the unit
        let mut unit = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphabetic() || c == 'µ' {
                unit.push(c);
                chars.next();
            } else {
                break;
            }
        }

        let multiplier: u128 = match unit.as_str() {
            "ns" => 1,
            "us" | "µs" => 1_000,
            "ms" => 1_000_000,
            "s" => 1_000_000_000,
            "m" => 60_000_000_000,
            "h" => 3_600_000_000_000,
            "" => {
                // If no unit, assume seconds (Go behavior)
                1_000_000_000
            }
            _ => return Err(ParseDurationError(format!("unknown unit: {}", unit))),
        };

        total_nanos += (num * multiplier as f64) as u128;
    }

    Ok(Duration(StdDuration::from_nanos(total_nanos as u64)))
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.format_go_duration())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_go_duration(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_serialize() {
        let time = Time(
            DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let json = serde_json::to_string(&time).unwrap();
        assert_eq!(json, "\"2024-01-15T10:30:00Z\"");
    }

    #[test]
    fn test_time_deserialize() {
        let json = "\"2024-01-15T10:30:00Z\"";
        let time: Time = serde_json::from_str(json).unwrap();
        assert_eq!(time.to_string(), "2024-01-15T10:30:00Z");
    }

    #[test]
    fn test_time_roundtrip() {
        let original = Time::now();
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Time = serde_json::from_str(&json).unwrap();
        // Compare at second precision (Time loses sub-second precision)
        assert_eq!(original.0.timestamp(), parsed.0.timestamp());
    }

    #[test]
    fn test_microtime_serialize() {
        let time = MicroTime(
            DateTime::parse_from_rfc3339("2024-01-15T10:30:00.123456Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let json = serde_json::to_string(&time).unwrap();
        assert_eq!(json, "\"2024-01-15T10:30:00.123456Z\"");
    }

    #[test]
    fn test_microtime_deserialize() {
        let json = "\"2024-01-15T10:30:00.123456Z\"";
        let time: MicroTime = serde_json::from_str(json).unwrap();
        assert_eq!(time.to_string(), "2024-01-15T10:30:00.123456Z");
    }

    #[test]
    fn test_duration_format() {
        assert_eq!(Duration::from_secs(0).to_string(), "0s");
        assert_eq!(Duration::from_secs(1).to_string(), "1s");
        assert_eq!(Duration::from_secs(60).to_string(), "1m");
        assert_eq!(Duration::from_secs(3600).to_string(), "1h");
        assert_eq!(Duration::from_secs(3661).to_string(), "1h1m1s");
        assert_eq!(Duration::from_millis(500).to_string(), "500ms");
    }

    #[test]
    fn test_duration_parse() {
        assert_eq!(Duration::from_str("0s").unwrap(), Duration::from_secs(0));
        assert_eq!(Duration::from_str("1s").unwrap(), Duration::from_secs(1));
        assert_eq!(Duration::from_str("1m").unwrap(), Duration::from_secs(60));
        assert_eq!(Duration::from_str("1h").unwrap(), Duration::from_secs(3600));
        assert_eq!(
            Duration::from_str("1h1m1s").unwrap(),
            Duration::from_secs(3661)
        );
        assert_eq!(
            Duration::from_str("500ms").unwrap(),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn test_duration_serialize() {
        let dur = Duration::from_secs(3661);
        let json = serde_json::to_string(&dur).unwrap();
        assert_eq!(json, "\"1h1m1s\"");
    }

    #[test]
    fn test_duration_deserialize() {
        let json = "\"1h30m\"";
        let dur: Duration = serde_json::from_str(json).unwrap();
        assert_eq!(dur.as_secs(), 5400);
    }

    #[test]
    fn test_duration_roundtrip() {
        let original = Duration::from_secs(7265); // 2h1m5s
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Duration = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_time_is_zero() {
        let zero = Time::default();
        assert!(zero.is_zero());

        let non_zero = Time::now();
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_duration_is_zero() {
        let zero = Duration::default();
        assert!(zero.is_zero());

        let non_zero = Duration::from_secs(1);
        assert!(!non_zero.is_zero());
    }
}
