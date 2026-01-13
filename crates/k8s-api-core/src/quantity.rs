//! Quantity type for Kubernetes resource quantities.
//!
//! Quantity is a fixed-point representation of a number used for resource
//! specifications in Kubernetes. It provides convenient marshaling/unmarshaling
//! in JSON and YAML, and supports suffixes like Ki, Mi, Gi (binary) and k, M, G (decimal).
//!
//! # Examples
//!
//! ```
//! use k8s_api_core::Quantity;
//!
//! // Parse from string
//! let q = Quantity::from("100m");
//! assert_eq!(q.to_string(), "100m");
//!
//! // Create from numeric values
//! let q = Quantity::from_milli(1500); // 1500m = 1.5
//! let q = Quantity::from_value(1024 * 1024 * 1024, "Gi"); // 1Gi
//! ```

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Errors that can occur when parsing a Quantity.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    /// The quantity string format is invalid.
    #[error("invalid quantity format: {0}")]
    InvalidFormat(String),
    /// The numeric part of the quantity is invalid.
    #[error("invalid numeric value: {0}")]
    InvalidNumber(String),
    /// The suffix is not recognized.
    #[error("unknown suffix: {0}")]
    UnknownSuffix(String),
}

/// The format used for representing the quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// Decimal SI suffixes (k, M, G, T, P, E) - powers of 10^3.
    DecimalSI,
    /// Binary SI suffixes (Ki, Mi, Gi, Ti, Pi, Ei) - powers of 2^10.
    BinarySI,
    /// Decimal exponent notation (e.g., 12e6).
    DecimalExponent,
}

/// Scale represents the scale of a quantity (power of 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scale(pub i32);

impl Scale {
    /// Scale for milli (10^-3).
    pub const MILLI: Scale = Scale(-3);
    /// Scale for micro (10^-6).
    pub const MICRO: Scale = Scale(-6);
    /// Scale for nano (10^-9).
    pub const NANO: Scale = Scale(-9);
    /// Scale for one (10^0).
    pub const ONE: Scale = Scale(0);
    /// Scale for kilo (10^3).
    pub const KILO: Scale = Scale(3);
    /// Scale for mega (10^6).
    pub const MEGA: Scale = Scale(6);
    /// Scale for giga (10^9).
    pub const GIGA: Scale = Scale(9);
    /// Scale for tera (10^12).
    pub const TERA: Scale = Scale(12);
    /// Scale for peta (10^15).
    pub const PETA: Scale = Scale(15);
    /// Scale for exa (10^18).
    pub const EXA: Scale = Scale(18);
}

/// Quantity is a fixed-point representation of a number.
///
/// It provides convenient marshaling/unmarshaling in JSON and YAML,
/// and string conversion. The quantity will NEVER be internally represented
/// by a floating point number to maintain precision.
///
/// # Serialization
///
/// Quantities serialize to strings in JSON:
/// - `"100m"` - 100 milli (0.1)
/// - `"1"` - one
/// - `"1Ki"` - 1024 (binary kibi)
/// - `"1k"` - 1000 (decimal kilo)
/// - `"1.5Gi"` - 1.5 * 2^30 bytes
///
/// # Supported Suffixes
///
/// Binary (powers of 2^10): Ki, Mi, Gi, Ti, Pi, Ei
/// Decimal (powers of 10^3): m, k, M, G, T, P, E
/// Decimal exponent: e or E followed by an integer
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quantity {
    /// The raw string representation.
    value: String,
}

impl Quantity {
    /// Creates a new Quantity from a string value.
    ///
    /// This does not validate the format. Use `parse` for validation.
    pub fn new(value: impl Into<String>) -> Self {
        Quantity {
            value: value.into(),
        }
    }

    /// Returns the string representation of the quantity.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns true if the quantity is zero.
    pub fn is_zero(&self) -> bool {
        self.value == "0" || self.value.is_empty()
    }

    /// Creates a Quantity representing zero.
    pub fn zero() -> Self {
        Quantity {
            value: "0".to_string(),
        }
    }

    /// Creates a Quantity from milli units (e.g., 1500 = "1500m").
    pub fn from_milli(value: i64) -> Self {
        if value == 0 {
            return Self::zero();
        }
        if value % 1000 == 0 {
            Quantity {
                value: (value / 1000).to_string(),
            }
        } else {
            Quantity {
                value: format!("{}m", value),
            }
        }
    }

    /// Creates a Quantity with a specific suffix.
    pub fn from_value(value: i64, suffix: &str) -> Self {
        if value == 0 {
            return Self::zero();
        }
        Quantity {
            value: format!("{}{}", value, suffix),
        }
    }

    /// Attempts to parse the quantity and return the value in milli units.
    ///
    /// Returns None if the quantity cannot be represented in milli units
    /// or if parsing fails.
    pub fn to_milli(&self) -> Option<i64> {
        self.parse_to_scale(Scale::MILLI)
    }

    /// Attempts to parse and return the value at the given scale.
    fn parse_to_scale(&self, target_scale: Scale) -> Option<i64> {
        let (value, scale) = self.parse_internal().ok()?;

        let scale_diff = scale.0 - target_scale.0;
        if scale_diff >= 0 {
            let multiplier = 10i64.checked_pow(scale_diff as u32)?;
            value.checked_mul(multiplier)
        } else {
            let divisor = 10i64.checked_pow((-scale_diff) as u32)?;
            if value % divisor == 0 {
                Some(value / divisor)
            } else {
                None // Would lose precision
            }
        }
    }

    /// Parses the quantity string into a value and scale.
    fn parse_internal(&self) -> Result<(i64, Scale), QuantityError> {
        let s = self.value.trim();
        if s.is_empty() {
            return Ok((0, Scale::ONE));
        }

        // Find where the number ends and suffix begins
        let mut num_end = s.len();
        for (i, c) in s.char_indices().rev() {
            if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
                num_end = i + c.len_utf8();
                break;
            }
        }

        // Handle exponent notation (e.g., "1e3", "1E6")
        if let Some(e_pos) = s.find(['e', 'E']) {
            // Check if what follows 'e' is a number (exponent notation)
            // vs a suffix like "Ei"
            let after_e = &s[e_pos + 1..];
            if after_e
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit() || c == '-' || c == '+')
            {
                // This is exponent notation
                let num_part = &s[..e_pos];
                let exp_part = &s[e_pos + 1..];

                let base: f64 = num_part
                    .parse()
                    .map_err(|_| QuantityError::InvalidNumber(num_part.to_string()))?;
                let exp: i32 = exp_part
                    .parse()
                    .map_err(|_| QuantityError::InvalidNumber(exp_part.to_string()))?;

                // Convert to integer at scale 0
                let multiplier = 10f64.powi(exp);
                let result = base * multiplier;
                if result.fract() != 0.0 {
                    // Has fractional part, need to adjust scale
                    let milli_value = (result * 1000.0).round() as i64;
                    return Ok((milli_value, Scale::MILLI));
                }
                return Ok((result as i64, Scale::ONE));
            }
        }

        // Parse numeric part
        let num_str = &s[..num_end];
        let suffix = &s[num_end..];

        // Handle decimal numbers by converting to milli
        if num_str.contains('.') {
            let num: f64 = num_str
                .parse()
                .map_err(|_| QuantityError::InvalidNumber(num_str.to_string()))?;

            let (base_scale, multiplier) = suffix_to_scale_and_multiplier(suffix)?;

            // Calculate value in milli
            let base_value = num * multiplier as f64 * 1000.0;
            return Ok((base_value.round() as i64, Scale(base_scale.0 - 3)));
        }

        let num: i64 = if num_str.is_empty() {
            0
        } else {
            num_str
                .parse()
                .map_err(|_| QuantityError::InvalidNumber(num_str.to_string()))?
        };

        let (scale, multiplier) = suffix_to_scale_and_multiplier(suffix)?;
        Ok((num * multiplier, scale))
    }
}

/// Converts a suffix to its scale and base multiplier.
fn suffix_to_scale_and_multiplier(suffix: &str) -> Result<(Scale, i64), QuantityError> {
    match suffix {
        "" => Ok((Scale::ONE, 1)),
        "m" => Ok((Scale::MILLI, 1)),
        "k" => Ok((Scale::KILO, 1)),
        "M" => Ok((Scale::MEGA, 1)),
        "G" => Ok((Scale::GIGA, 1)),
        "T" => Ok((Scale::TERA, 1)),
        "P" => Ok((Scale::PETA, 1)),
        "E" => Ok((Scale::EXA, 1)),
        "Ki" => Ok((Scale::ONE, 1024)),
        "Mi" => Ok((Scale::ONE, 1024 * 1024)),
        "Gi" => Ok((Scale::ONE, 1024 * 1024 * 1024)),
        "Ti" => Ok((Scale::ONE, 1024i64 * 1024 * 1024 * 1024)),
        "Pi" => Ok((Scale::ONE, 1024i64 * 1024 * 1024 * 1024 * 1024)),
        "Ei" => Ok((Scale::ONE, 1024i64 * 1024 * 1024 * 1024 * 1024 * 1024)),
        _ => Err(QuantityError::UnknownSuffix(suffix.to_string())),
    }
}

impl Default for Quantity {
    fn default() -> Self {
        Quantity::zero()
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for Quantity {
    type Err = QuantityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let q = Quantity::new(s);
        // Validate by attempting to parse
        q.parse_internal()?;
        Ok(q)
    }
}

impl From<&str> for Quantity {
    fn from(s: &str) -> Self {
        Quantity::new(s)
    }
}

impl From<String> for Quantity {
    fn from(s: String) -> Self {
        Quantity::new(s)
    }
}

impl From<i64> for Quantity {
    fn from(v: i64) -> Self {
        Quantity::new(v.to_string())
    }
}

impl From<i32> for Quantity {
    fn from(v: i32) -> Self {
        Quantity::new(v.to_string())
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Quantity {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by milli value for ordering
        match (self.to_milli(), other.to_milli()) {
            (Some(a), Some(b)) => a.cmp(&b),
            // Fall back to string comparison if parsing fails
            _ => self.value.cmp(&other.value),
        }
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QuantityVisitor;

        impl serde::de::Visitor<'_> for QuantityVisitor {
            type Value = Quantity;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a quantity string like '100m', '1Gi', or '500'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Quantity, E>
            where
                E: serde::de::Error,
            {
                Ok(Quantity::new(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Quantity, E>
            where
                E: serde::de::Error,
            {
                Ok(Quantity::new(value))
            }

            // Also handle numeric values
            fn visit_i64<E>(self, value: i64) -> Result<Quantity, E>
            where
                E: serde::de::Error,
            {
                Ok(Quantity::from(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Quantity, E>
            where
                E: serde::de::Error,
            {
                Ok(Quantity::new(value.to_string()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Quantity, E>
            where
                E: serde::de::Error,
            {
                // Convert to string representation
                Ok(Quantity::new(value.to_string()))
            }
        }

        deserializer.deserialize_any(QuantityVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_from_string() {
        let q = Quantity::from("100m");
        assert_eq!(q.as_str(), "100m");
    }

    #[test]
    fn test_quantity_zero() {
        let q = Quantity::zero();
        assert!(q.is_zero());
        assert_eq!(q.to_string(), "0");
    }

    #[test]
    fn test_quantity_from_milli() {
        assert_eq!(Quantity::from_milli(1500).to_string(), "1500m");
        assert_eq!(Quantity::from_milli(1000).to_string(), "1");
        assert_eq!(Quantity::from_milli(0).to_string(), "0");
    }

    #[test]
    fn test_quantity_serialize() {
        let q = Quantity::from("100Mi");
        let json = serde_json::to_string(&q).unwrap();
        assert_eq!(json, "\"100Mi\"");
    }

    #[test]
    fn test_quantity_deserialize() {
        let q: Quantity = serde_json::from_str("\"100Mi\"").unwrap();
        assert_eq!(q.as_str(), "100Mi");
    }

    #[test]
    fn test_quantity_deserialize_number() {
        let q: Quantity = serde_json::from_str("1024").unwrap();
        assert_eq!(q.as_str(), "1024");
    }

    #[test]
    fn test_quantity_to_milli() {
        assert_eq!(Quantity::from("1").to_milli(), Some(1000));
        assert_eq!(Quantity::from("100m").to_milli(), Some(100));
        assert_eq!(Quantity::from("1k").to_milli(), Some(1_000_000));
        assert_eq!(Quantity::from("1M").to_milli(), Some(1_000_000_000));
    }

    #[test]
    fn test_quantity_binary_suffixes() {
        let q = Quantity::from("1Ki");
        assert_eq!(q.to_milli(), Some(1024 * 1000));

        let q = Quantity::from("1Mi");
        assert_eq!(q.to_milli(), Some(1024 * 1024 * 1000));
    }

    #[test]
    fn test_quantity_ordering() {
        let q1 = Quantity::from("100m");
        let q2 = Quantity::from("1");
        let q3 = Quantity::from("1k");

        assert!(q1 < q2);
        assert!(q2 < q3);
        assert!(q1 < q3);
    }

    #[test]
    fn test_quantity_roundtrip() {
        let original = Quantity::from("500Mi");
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Quantity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_quantity_parse_decimal() {
        let q = Quantity::from("1.5");
        assert_eq!(q.to_milli(), Some(1500));
    }

    #[test]
    fn test_quantity_parse_exponent() {
        let q = Quantity::from("1e3");
        assert_eq!(q.to_milli(), Some(1_000_000)); // 1000 * 1000 milli
    }

    #[test]
    fn test_quantity_default() {
        let q = Quantity::default();
        assert!(q.is_zero());
    }

    #[test]
    fn test_quantity_from_value() {
        let q = Quantity::from_value(100, "Mi");
        assert_eq!(q.to_string(), "100Mi");
    }
}
