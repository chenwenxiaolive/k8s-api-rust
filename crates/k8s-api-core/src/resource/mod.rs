//! Resource types for Kubernetes API
//!
//! Provides types like Quantity (for resource amounts) and IntOrString.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Quantity is a representation of a numeric value with an optional SI suffix.
///
/// Examples: "100m", "1Gi", "500Mi", "1.5"
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Quantity(pub String);

impl Quantity {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Quantity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Quantity(s))
    }
}

/// IntOrString is a type that can hold either an integer or a string.
///
/// Used for fields like port (which can be a number or a named port).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntOrString {
    Int(i32),
    String(String),
}

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
}

impl From<i32> for IntOrString {
    fn from(i: i32) -> Self {
        IntOrString::Int(i)
    }
}

impl From<String> for IntOrString {
    fn from(s: String) -> Self {
        IntOrString::String(s)
    }
}

impl From<&str> for IntOrString {
    fn from(s: &str) -> Self {
        IntOrString::String(s.to_string())
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

        impl<'de> de::Visitor<'de> for IntOrStringVisitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or a string")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntOrString::Int(v as i32))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntOrString::Int(v as i32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntOrString::String(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntOrString::String(v))
            }
        }

        deserializer.deserialize_any(IntOrStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_serialize() {
        let q = Quantity::new("100Mi");
        let json = serde_json::to_string(&q).unwrap();
        assert_eq!(json, "\"100Mi\"");
    }

    #[test]
    fn test_int_or_string() {
        let int_val: IntOrString = 8080.into();
        let str_val: IntOrString = "http".into();

        assert_eq!(serde_json::to_string(&int_val).unwrap(), "8080");
        assert_eq!(serde_json::to_string(&str_val).unwrap(), "\"http\"");

        let parsed_int: IntOrString = serde_json::from_str("8080").unwrap();
        let parsed_str: IntOrString = serde_json::from_str("\"http\"").unwrap();

        assert_eq!(parsed_int, IntOrString::Int(8080));
        assert_eq!(parsed_str, IntOrString::String("http".to_string()));
    }
}
