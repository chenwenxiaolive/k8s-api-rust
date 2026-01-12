//! Primitive types for Kubernetes API
//!
//! This module contains primitive types that are used across multiple API groups.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// IntOrString is a type that can hold either an integer or a string.
///
/// This is used in Kubernetes for fields that can accept either a numeric value
/// or a string (e.g., port specifications that can be a number or a named port).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntOrString {
    /// An integer value
    Int(i32),
    /// A string value
    String(String),
}

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
}

impl From<i32> for IntOrString {
    fn from(value: i32) -> Self {
        IntOrString::Int(value)
    }
}

impl From<String> for IntOrString {
    fn from(value: String) -> Self {
        IntOrString::String(value)
    }
}

impl From<&str> for IntOrString {
    fn from(value: &str) -> Self {
        IntOrString::String(value.to_owned())
    }
}

impl fmt::Display for IntOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntOrString::Int(i) => write!(f, "{}", i),
            IntOrString::String(s) => write!(f, "{}", s),
        }
    }
}

impl Serialize for IntOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IntOrString::Int(i) => serializer.serialize_i32(*i),
            IntOrString::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for IntOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntOrStringVisitor;

        impl<'de> serde::de::Visitor<'de> for IntOrStringVisitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or a string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<IntOrString, E>
            where
                E: serde::de::Error,
            {
                Ok(IntOrString::Int(value as i32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<IntOrString, E>
            where
                E: serde::de::Error,
            {
                Ok(IntOrString::Int(value as i32))
            }

            fn visit_str<E>(self, value: &str) -> Result<IntOrString, E>
            where
                E: serde::de::Error,
            {
                Ok(IntOrString::String(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<IntOrString, E>
            where
                E: serde::de::Error,
            {
                Ok(IntOrString::String(value))
            }
        }

        deserializer.deserialize_any(IntOrStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_or_string_serialize_int() {
        let value = IntOrString::Int(8080);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "8080");
    }

    #[test]
    fn test_int_or_string_serialize_string() {
        let value = IntOrString::String("http".to_string());
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"http\"");
    }

    #[test]
    fn test_int_or_string_deserialize_int() {
        let value: IntOrString = serde_json::from_str("8080").unwrap();
        assert_eq!(value, IntOrString::Int(8080));
    }

    #[test]
    fn test_int_or_string_deserialize_string() {
        let value: IntOrString = serde_json::from_str("\"http\"").unwrap();
        assert_eq!(value, IntOrString::String("http".to_string()));
    }
}
