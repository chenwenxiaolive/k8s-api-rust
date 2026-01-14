//! Common validation utilities for Kubernetes names, labels, and annotations
//!
//! This module provides validation functions that are reused across different API types.

use crate::{ValidationError, ValidationResult};
use once_cell::sync::Lazy;
use regex::Regex;

// =============================================================================
// Constants
// =============================================================================

/// Maximum length of a Kubernetes name (DNS subdomain)
pub const DNS_SUBDOMAIN_MAX_LENGTH: usize = 253;

/// Maximum length of a DNS label (RFC 1123)
pub const DNS_LABEL_MAX_LENGTH: usize = 63;

/// Maximum length of a qualified name (e.g., label keys)
pub const QUALIFIED_NAME_MAX_LENGTH: usize = 63;

/// Maximum total length of a label key with prefix
pub const LABEL_KEY_MAX_LENGTH: usize = 253 + 1 + 63; // prefix/name

/// Maximum length of a label value
pub const LABEL_VALUE_MAX_LENGTH: usize = 63;

/// Maximum length of an annotation key
pub const ANNOTATION_KEY_MAX_LENGTH: usize = 253 + 1 + 63;

/// Maximum length of annotation value (256KB)
pub const ANNOTATION_VALUE_MAX_LENGTH: usize = 256 * 1024;

// =============================================================================
// Regex patterns
// =============================================================================

static DNS_SUBDOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*$").unwrap()
});

static DNS_LABEL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9]([-a-z0-9]*[a-z0-9])?$").unwrap()
});

static QUALIFIED_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]$").unwrap()
});

static LABEL_VALUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?$").unwrap()
});

static ENV_VAR_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap()
});

static PORT_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9]([-a-z0-9]*[a-z0-9])?$").unwrap()
});

// =============================================================================
// Name Validation
// =============================================================================

/// Validates a Kubernetes object name (DNS subdomain format).
pub fn validate_dns_subdomain_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "name is required"));
        return errors;
    }

    if name.len() > DNS_SUBDOMAIN_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, DNS_SUBDOMAIN_MAX_LENGTH, name.len()));
    }

    if !DNS_SUBDOMAIN_REGEX.is_match(name) {
        errors.push(ValidationError::invalid(
            field,
            "must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character",
        ));
    }

    errors
}

/// Validates a DNS label (RFC 1123).
pub fn validate_dns_label(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "name is required"));
        return errors;
    }

    if name.len() > DNS_LABEL_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, DNS_LABEL_MAX_LENGTH, name.len()));
    }

    if !DNS_LABEL_REGEX.is_match(name) {
        errors.push(ValidationError::invalid(
            field,
            "must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character",
        ));
    }

    errors
}

/// Validates a qualified name (used for label keys, annotation keys without prefix).
pub fn validate_qualified_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "name is required"));
        return errors;
    }

    if name.len() > QUALIFIED_NAME_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, QUALIFIED_NAME_MAX_LENGTH, name.len()));
    }

    if !QUALIFIED_NAME_REGEX.is_match(name) {
        errors.push(ValidationError::invalid(
            field,
            "must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character",
        ));
    }

    errors
}

// =============================================================================
// Label Validation
// =============================================================================

/// Validates a label key (with optional prefix).
pub fn validate_label_key(key: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if key.is_empty() {
        errors.push(ValidationError::required(field, "label key is required"));
        return errors;
    }

    if key.len() > LABEL_KEY_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, LABEL_KEY_MAX_LENGTH, key.len()));
    }

    // Check for prefix/name format
    if let Some(slash_idx) = key.find('/') {
        let prefix = &key[..slash_idx];
        let name = &key[slash_idx + 1..];

        // Validate prefix as DNS subdomain
        errors.extend(validate_dns_subdomain_name(prefix, &format!("{} (prefix)", field)));

        // Validate name as qualified name
        errors.extend(validate_qualified_name(name, &format!("{} (name)", field)));
    } else {
        // No prefix, validate as qualified name
        errors.extend(validate_qualified_name(key, field));
    }

    errors
}

/// Validates a label value.
pub fn validate_label_value(value: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Empty value is allowed
    if value.is_empty() {
        return errors;
    }

    if value.len() > LABEL_VALUE_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, LABEL_VALUE_MAX_LENGTH, value.len()));
    }

    if !LABEL_VALUE_REGEX.is_match(value) {
        errors.push(ValidationError::invalid(
            field,
            "must be empty or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character",
        ));
    }

    errors
}

/// Validates labels map.
pub fn validate_labels(labels: &std::collections::BTreeMap<String, String>, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    for (key, value) in labels {
        errors.extend(validate_label_key(key, &format!("{}[{}] (key)", field, key)));
        errors.extend(validate_label_value(value, &format!("{}[{}] (value)", field, key)));
    }

    errors
}

// =============================================================================
// Annotation Validation
// =============================================================================

/// Validates an annotation key.
pub fn validate_annotation_key(key: &str, field: &str) -> ValidationResult {
    // Annotation keys follow the same rules as label keys
    validate_label_key(key, field)
}

/// Validates an annotation value.
pub fn validate_annotation_value(value: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if value.len() > ANNOTATION_VALUE_MAX_LENGTH {
        errors.push(ValidationError::too_long(field, ANNOTATION_VALUE_MAX_LENGTH, value.len()));
    }

    errors
}

/// Validates annotations map.
pub fn validate_annotations(annotations: &std::collections::BTreeMap<String, String>, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    for (key, value) in annotations {
        errors.extend(validate_annotation_key(key, &format!("{}[{}] (key)", field, key)));
        errors.extend(validate_annotation_value(value, &format!("{}[{}] (value)", field, key)));
    }

    errors
}

// =============================================================================
// Port Validation
// =============================================================================

/// Validates a port number.
pub fn validate_port_number(port: i32, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if port < 1 || port > 65535 {
        errors.push(ValidationError::out_of_range(field, 1, 65535, port as i64));
    }

    errors
}

/// Validates a port name.
pub fn validate_port_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        return errors; // Empty port name is allowed
    }

    if name.len() > 15 {
        errors.push(ValidationError::too_long(field, 15, name.len()));
    }

    if !PORT_NAME_REGEX.is_match(name) {
        errors.push(ValidationError::invalid(
            field,
            "must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character",
        ));
    }

    // Port names cannot be all digits
    if name.chars().all(|c| c.is_ascii_digit()) {
        errors.push(ValidationError::invalid(field, "port name cannot be all digits"));
    }

    errors
}

// =============================================================================
// Environment Variable Validation
// =============================================================================

/// Validates an environment variable name.
pub fn validate_env_var_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "environment variable name is required"));
        return errors;
    }

    if !ENV_VAR_NAME_REGEX.is_match(name) {
        errors.push(ValidationError::invalid(
            field,
            "must consist of alphanumeric characters and underscores, and cannot start with a digit",
        ));
    }

    errors
}

// =============================================================================
// Protocol Validation
// =============================================================================

/// Valid protocols for container ports and services.
pub const VALID_PROTOCOLS: &[&str] = &["TCP", "UDP", "SCTP"];

/// Validates a protocol.
pub fn validate_protocol(protocol: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if protocol.is_empty() {
        return errors; // Empty defaults to TCP
    }

    if !VALID_PROTOCOLS.contains(&protocol) {
        errors.push(ValidationError::not_supported(field, protocol, VALID_PROTOCOLS));
    }

    errors
}

// =============================================================================
// ObjectMeta Validation
// =============================================================================

/// Validates ObjectMeta.
pub fn validate_object_meta(
    meta: &k8s_apimachinery::apis::meta::v1::ObjectMeta,
    field: &str,
    require_name: bool,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate name
    if require_name && meta.name.is_empty() && meta.generate_name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "name or generateName is required",
        ));
    }

    if !meta.name.is_empty() {
        errors.extend(validate_dns_subdomain_name(&meta.name, &format!("{}.name", field)));
    }

    if !meta.generate_name.is_empty() {
        errors.extend(validate_dns_subdomain_name(
            &meta.generate_name,
            &format!("{}.generateName", field),
        ));
    }

    // Validate namespace
    if !meta.namespace.is_empty() {
        errors.extend(validate_dns_label(&meta.namespace, &format!("{}.namespace", field)));
    }

    // Validate labels
    errors.extend(validate_labels(&meta.labels, &format!("{}.labels", field)));

    // Validate annotations
    errors.extend(validate_annotations(&meta.annotations, &format!("{}.annotations", field)));

    errors
}

// =============================================================================
// Resource Quantity Validation
// =============================================================================

/// SI suffixes for decimal quantities
const SI_SUFFIXES: &[(&str, i64)] = &[
    ("E", 1_000_000_000_000_000_000),  // Exa
    ("P", 1_000_000_000_000_000),       // Peta
    ("T", 1_000_000_000_000),           // Tera
    ("G", 1_000_000_000),               // Giga
    ("M", 1_000_000),                   // Mega
    ("k", 1_000),                       // Kilo
    ("m", 1),                           // Milli (stored as milli for precision)
    ("", 1000),                         // Base (converted to milli)
];

/// Binary suffixes for memory quantities
const BINARY_SUFFIXES: &[(&str, i64)] = &[
    ("Ei", 1 << 60), // Exbi
    ("Pi", 1 << 50), // Pebi
    ("Ti", 1 << 40), // Tebi
    ("Gi", 1 << 30), // Gibi
    ("Mi", 1 << 20), // Mebi
    ("Ki", 1 << 10), // Kibi
];

/// Regex pattern for quantity validation
static QUANTITY_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Matches: optional sign, decimal number, optional exponent, optional suffix
    // Examples: "100m", "1.5Gi", "1e6", "-500M", "0.1"
    Regex::new(r"^([+-]?[0-9]+\.?[0-9]*([eE][+-]?[0-9]+)?)(([EPTGMK]i?)|m|k)?$").unwrap()
});

/// Validates a Kubernetes resource quantity string.
///
/// Quantities can be expressed as:
/// - Plain integers: "100", "1000"
/// - Decimals: "0.5", "1.5"
/// - Scientific notation: "1e6", "1.5e3"
/// - SI suffixes: "100m" (milli), "1k" (kilo), "1M" (mega), "1G" (giga)
/// - Binary suffixes: "1Ki" (kibi), "1Mi" (mebi), "1Gi" (gibi)
pub fn validate_quantity(quantity: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if quantity.is_empty() {
        errors.push(ValidationError::required(field, "quantity is required"));
        return errors;
    }

    if !QUANTITY_REGEX.is_match(quantity) {
        errors.push(ValidationError::invalid(
            field,
            format!(
                "must be a valid quantity (e.g., '100m', '1Gi', '500M', '1.5'): got '{}'",
                quantity
            ),
        ));
    }

    errors
}

/// Validates a CPU resource quantity.
///
/// CPU quantities are typically expressed as:
/// - Millicores: "100m", "500m", "2000m"
/// - Cores: "0.1", "0.5", "1", "2.5"
///
/// Returns an error if the quantity is invalid or negative.
pub fn validate_cpu_quantity(quantity: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // First validate general quantity format
    errors.extend(validate_quantity(quantity, field));
    if !errors.is_empty() {
        return errors;
    }

    // Parse and validate CPU-specific constraints
    match parse_cpu_quantity(quantity) {
        Ok(millicores) => {
            if millicores < 0 {
                errors.push(ValidationError::invalid(
                    field,
                    "CPU quantity cannot be negative",
                ));
            }
        }
        Err(e) => {
            errors.push(ValidationError::invalid(field, e));
        }
    }

    errors
}

/// Validates a memory resource quantity.
///
/// Memory quantities are typically expressed as:
/// - Bytes: "134217728"
/// - Binary suffixes: "128Mi", "1Gi", "2Ti"
/// - SI suffixes: "128M", "1G" (note: 1G = 1000M, 1Gi = 1024Mi)
///
/// Returns an error if the quantity is invalid or negative.
pub fn validate_memory_quantity(quantity: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // First validate general quantity format
    errors.extend(validate_quantity(quantity, field));
    if !errors.is_empty() {
        return errors;
    }

    // Parse and validate memory-specific constraints
    match parse_memory_quantity(quantity) {
        Ok(bytes) => {
            if bytes < 0 {
                errors.push(ValidationError::invalid(
                    field,
                    "memory quantity cannot be negative",
                ));
            }
        }
        Err(e) => {
            errors.push(ValidationError::invalid(field, e));
        }
    }

    errors
}

/// Validates resource requirements (requests and limits).
///
/// Validates that:
/// - All resource names are valid
/// - All quantities are valid
/// - Requests don't exceed limits (if both specified)
pub fn validate_resource_requirements(
    requests: &std::collections::BTreeMap<String, String>,
    limits: &std::collections::BTreeMap<String, String>,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate requests
    for (resource_name, quantity) in requests {
        errors.extend(validate_resource_name(
            resource_name,
            &format!("{}.requests[{}]", field, resource_name),
        ));

        let quantity_field = format!("{}.requests[{}]", field, resource_name);
        if resource_name == "cpu" {
            errors.extend(validate_cpu_quantity(quantity, &quantity_field));
        } else if resource_name == "memory" || resource_name.ends_with("-memory") {
            errors.extend(validate_memory_quantity(quantity, &quantity_field));
        } else {
            errors.extend(validate_quantity(quantity, &quantity_field));
        }
    }

    // Validate limits
    for (resource_name, quantity) in limits {
        errors.extend(validate_resource_name(
            resource_name,
            &format!("{}.limits[{}]", field, resource_name),
        ));

        let quantity_field = format!("{}.limits[{}]", field, resource_name);
        if resource_name == "cpu" {
            errors.extend(validate_cpu_quantity(quantity, &quantity_field));
        } else if resource_name == "memory" || resource_name.ends_with("-memory") {
            errors.extend(validate_memory_quantity(quantity, &quantity_field));
        } else {
            errors.extend(validate_quantity(quantity, &quantity_field));
        }
    }

    // Validate that requests don't exceed limits
    for (resource_name, request_quantity) in requests {
        if let Some(limit_quantity) = limits.get(resource_name) {
            if resource_name == "cpu" {
                if let (Ok(req), Ok(lim)) = (
                    parse_cpu_quantity(request_quantity),
                    parse_cpu_quantity(limit_quantity),
                ) {
                    if req > lim {
                        errors.push(ValidationError::invalid(
                            format!("{}.requests[{}]", field, resource_name),
                            format!(
                                "request ({}) cannot exceed limit ({})",
                                request_quantity,
                                limit_quantity
                            ),
                        ));
                    }
                }
            } else if resource_name == "memory" || resource_name.ends_with("-memory") {
                if let (Ok(req), Ok(lim)) = (
                    parse_memory_quantity(request_quantity),
                    parse_memory_quantity(limit_quantity),
                ) {
                    if req > lim {
                        errors.push(ValidationError::invalid(
                            format!("{}.requests[{}]", field, resource_name),
                            format!(
                                "request ({}) cannot exceed limit ({})",
                                request_quantity,
                                limit_quantity
                            ),
                        ));
                    }
                }
            }
        }
    }

    errors
}

/// Validates a resource name (cpu, memory, or extended resource).
pub fn validate_resource_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "resource name is required"));
        return errors;
    }

    // Standard resource names
    let standard_resources = ["cpu", "memory", "storage", "ephemeral-storage"];
    if standard_resources.contains(&name) {
        return errors;
    }

    // ResourceQuota prefixed names (requests.*, limits.*)
    let quota_prefixes = ["requests.", "limits."];
    for prefix in quota_prefixes {
        if let Some(suffix) = name.strip_prefix(prefix) {
            // Validate the suffix part
            return validate_resource_name(suffix, field);
        }
    }

    // Extended resource names must be qualified names
    // e.g., "nvidia.com/gpu", "kubernetes.io/some-resource"
    if name.contains('/') {
        errors.extend(validate_label_key(name, field));
    } else {
        // Unqualified extended resources must be DNS labels
        errors.extend(validate_dns_label(name, field));
    }

    errors
}

/// Parses a CPU quantity string and returns the value in millicores.
fn parse_cpu_quantity(quantity: &str) -> Result<i64, String> {
    let quantity = quantity.trim();

    // Handle millicores suffix
    if let Some(num_str) = quantity.strip_suffix('m') {
        return num_str
            .parse::<f64>()
            .map(|v| v as i64)
            .map_err(|_| format!("invalid CPU quantity: {}", quantity));
    }

    // Handle plain number (cores)
    quantity
        .parse::<f64>()
        .map(|v| (v * 1000.0) as i64) // Convert to millicores
        .map_err(|_| format!("invalid CPU quantity: {}", quantity))
}

/// Parses a memory quantity string and returns the value in bytes.
fn parse_memory_quantity(quantity: &str) -> Result<i64, String> {
    let quantity = quantity.trim();

    // Try binary suffixes first (Gi, Mi, Ki, etc.)
    for (suffix, multiplier) in BINARY_SUFFIXES {
        if let Some(num_str) = quantity.strip_suffix(suffix) {
            return num_str
                .parse::<f64>()
                .map(|v| (v * (*multiplier as f64)) as i64)
                .map_err(|_| format!("invalid memory quantity: {}", quantity));
        }
    }

    // Try SI suffixes (G, M, K, etc.)
    for (suffix, multiplier) in SI_SUFFIXES {
        if suffix.is_empty() {
            continue;
        }
        if let Some(num_str) = quantity.strip_suffix(suffix) {
            // For memory, SI suffixes represent 10^n, not milli
            let actual_multiplier = if *suffix == "m" {
                // milli doesn't make sense for memory, but handle anyway
                1
            } else {
                *multiplier
            };
            return num_str
                .parse::<f64>()
                .map(|v| (v * (actual_multiplier as f64)) as i64)
                .map_err(|_| format!("invalid memory quantity: {}", quantity));
        }
    }

    // Handle scientific notation and plain numbers
    quantity
        .parse::<f64>()
        .map(|v| v as i64)
        .map_err(|_| format!("invalid memory quantity: {}", quantity))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dns_subdomain_name() {
        // Valid names
        assert!(validate_dns_subdomain_name("my-app", "test").is_empty());
        assert!(validate_dns_subdomain_name("my.app.v1", "test").is_empty());
        assert!(validate_dns_subdomain_name("a", "test").is_empty());
        assert!(validate_dns_subdomain_name("1a", "test").is_empty());

        // Invalid names
        assert!(!validate_dns_subdomain_name("", "test").is_empty());
        assert!(!validate_dns_subdomain_name("My-App", "test").is_empty()); // uppercase
        assert!(!validate_dns_subdomain_name("-my-app", "test").is_empty()); // starts with dash
        assert!(!validate_dns_subdomain_name("my-app-", "test").is_empty()); // ends with dash
    }

    #[test]
    fn test_validate_dns_label() {
        // Valid labels
        assert!(validate_dns_label("my-app", "test").is_empty());
        assert!(validate_dns_label("a", "test").is_empty());

        // Invalid labels
        assert!(!validate_dns_label("", "test").is_empty());
        assert!(!validate_dns_label("my.app", "test").is_empty()); // contains dot
        assert!(!validate_dns_label("MY-APP", "test").is_empty()); // uppercase
    }

    #[test]
    fn test_validate_label_key() {
        // Valid keys
        assert!(validate_label_key("app", "test").is_empty());
        assert!(validate_label_key("app.kubernetes.io/name", "test").is_empty());
        assert!(validate_label_key("my-label", "test").is_empty());

        // Invalid keys
        assert!(!validate_label_key("", "test").is_empty());
    }

    #[test]
    fn test_validate_label_value() {
        // Valid values
        assert!(validate_label_value("", "test").is_empty()); // empty is valid
        assert!(validate_label_value("my-value", "test").is_empty());
        assert!(validate_label_value("v1.0.0", "test").is_empty());

        // Invalid values
        assert!(!validate_label_value("-value", "test").is_empty()); // starts with dash
    }

    #[test]
    fn test_validate_port_number() {
        assert!(validate_port_number(80, "test").is_empty());
        assert!(validate_port_number(1, "test").is_empty());
        assert!(validate_port_number(65535, "test").is_empty());

        assert!(!validate_port_number(0, "test").is_empty());
        assert!(!validate_port_number(-1, "test").is_empty());
        assert!(!validate_port_number(65536, "test").is_empty());
    }

    #[test]
    fn test_validate_env_var_name() {
        assert!(validate_env_var_name("MY_VAR", "test").is_empty());
        assert!(validate_env_var_name("_PRIVATE", "test").is_empty());
        assert!(validate_env_var_name("VAR123", "test").is_empty());

        assert!(!validate_env_var_name("", "test").is_empty());
        assert!(!validate_env_var_name("123VAR", "test").is_empty()); // starts with digit
        assert!(!validate_env_var_name("my-var", "test").is_empty()); // contains dash
    }

    #[test]
    fn test_validate_quantity() {
        // Valid quantities
        assert!(validate_quantity("100", "test").is_empty());
        assert!(validate_quantity("100m", "test").is_empty());
        assert!(validate_quantity("1Ki", "test").is_empty());
        assert!(validate_quantity("1Gi", "test").is_empty());
        assert!(validate_quantity("0.5", "test").is_empty());
        assert!(validate_quantity("500M", "test").is_empty());
        // Note: negative quantities are syntactically valid, context-specific validation
        // (like CPU/memory) rejects negative values
        assert!(validate_quantity("-100", "test").is_empty());

        // Invalid quantities
        assert!(!validate_quantity("", "test").is_empty());
        assert!(!validate_quantity("abc", "test").is_empty());
    }

    #[test]
    fn test_validate_cpu_quantity() {
        // Valid CPU quantities
        assert!(validate_cpu_quantity("100m", "test").is_empty());
        assert!(validate_cpu_quantity("1", "test").is_empty());
        assert!(validate_cpu_quantity("0.5", "test").is_empty());
        assert!(validate_cpu_quantity("1000m", "test").is_empty());
        assert!(validate_cpu_quantity("2500m", "test").is_empty());

        // Invalid CPU quantities
        assert!(!validate_cpu_quantity("", "test").is_empty());
        assert!(!validate_cpu_quantity("-100m", "test").is_empty());
        assert!(!validate_cpu_quantity("abc", "test").is_empty());
    }

    #[test]
    fn test_validate_memory_quantity() {
        // Valid memory quantities
        assert!(validate_memory_quantity("128Mi", "test").is_empty());
        assert!(validate_memory_quantity("1Gi", "test").is_empty());
        assert!(validate_memory_quantity("512M", "test").is_empty());
        assert!(validate_memory_quantity("1000000", "test").is_empty());
        assert!(validate_memory_quantity("1Ti", "test").is_empty());

        // Invalid memory quantities
        assert!(!validate_memory_quantity("", "test").is_empty());
        assert!(!validate_memory_quantity("-128Mi", "test").is_empty());
        assert!(!validate_memory_quantity("abc", "test").is_empty());
    }

    #[test]
    fn test_validate_resource_name() {
        // Valid resource names
        assert!(validate_resource_name("cpu", "test").is_empty());
        assert!(validate_resource_name("memory", "test").is_empty());
        assert!(validate_resource_name("nvidia.com/gpu", "test").is_empty());
        assert!(validate_resource_name("requests.storage", "test").is_empty());

        // Invalid resource names
        assert!(!validate_resource_name("", "test").is_empty());
    }

    #[test]
    fn test_validate_resource_requirements() {
        use std::collections::BTreeMap;

        // Valid: requests <= limits
        let mut requests = BTreeMap::new();
        let mut limits = BTreeMap::new();
        requests.insert("cpu".to_string(), "100m".to_string());
        requests.insert("memory".to_string(), "128Mi".to_string());
        limits.insert("cpu".to_string(), "500m".to_string());
        limits.insert("memory".to_string(), "256Mi".to_string());
        assert!(validate_resource_requirements(&requests, &limits, "test").is_empty());

        // Invalid: request > limit for CPU
        let mut requests_exceed = BTreeMap::new();
        let mut limits_low = BTreeMap::new();
        requests_exceed.insert("cpu".to_string(), "1000m".to_string());
        limits_low.insert("cpu".to_string(), "500m".to_string());
        assert!(!validate_resource_requirements(&requests_exceed, &limits_low, "test").is_empty());

        // Invalid: request > limit for memory
        let mut mem_requests = BTreeMap::new();
        let mut mem_limits = BTreeMap::new();
        mem_requests.insert("memory".to_string(), "1Gi".to_string());
        mem_limits.insert("memory".to_string(), "512Mi".to_string());
        assert!(!validate_resource_requirements(&mem_requests, &mem_limits, "test").is_empty());
    }
}
