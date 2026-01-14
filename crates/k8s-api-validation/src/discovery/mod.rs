//! Discovery API validation
//!
//! This module provides validation for discovery API types including:
//! - EndpointSlice

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};
use k8s_api::discovery::v1::{Endpoint, EndpointPort, EndpointSlice};

/// Valid address types for EndpointSlice
const VALID_ADDRESS_TYPES: &[&str] = &["IPv4", "IPv6", "FQDN"];

/// Valid protocols for EndpointPort
const VALID_PROTOCOLS: &[&str] = &["TCP", "UDP", "SCTP"];

/// Maximum number of endpoints per slice
const MAX_ENDPOINTS_PER_SLICE: usize = 100;

/// Maximum number of ports per slice
const MAX_PORTS_PER_SLICE: usize = 100;

/// Maximum number of addresses per endpoint
const MAX_ADDRESSES_PER_ENDPOINT: usize = 100;

/// Maximum length for zone name
const MAX_ZONE_NAME_LENGTH: usize = 253;

// =============================================================================
// EndpointSlice Validation
// =============================================================================

/// Validates an EndpointSlice resource.
pub fn validate_endpoint_slice(es: &EndpointSlice) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&es.metadata, "metadata", true));

    // Validate addressType is required and valid
    if es.address_type.is_empty() {
        errors.push(ValidationError::required(
            "addressType",
            "addressType is required",
        ));
    } else if !VALID_ADDRESS_TYPES.contains(&es.address_type.as_str()) {
        errors.push(ValidationError::not_supported(
            "addressType",
            &es.address_type,
            VALID_ADDRESS_TYPES,
        ));
    }

    // Validate endpoints
    if es.endpoints.len() > MAX_ENDPOINTS_PER_SLICE {
        errors.push(ValidationError::too_long(
            "endpoints",
            MAX_ENDPOINTS_PER_SLICE,
            es.endpoints.len(),
        ));
    }

    for (i, endpoint) in es.endpoints.iter().enumerate() {
        errors.extend(validate_endpoint(
            endpoint,
            &format!("endpoints[{}]", i),
            &es.address_type,
        ));
    }

    // Validate ports
    if es.ports.len() > MAX_PORTS_PER_SLICE {
        errors.push(ValidationError::too_long(
            "ports",
            MAX_PORTS_PER_SLICE,
            es.ports.len(),
        ));
    }

    // Check for duplicate port names
    let mut seen_port_names = std::collections::HashSet::new();
    for (i, port) in es.ports.iter().enumerate() {
        errors.extend(validate_endpoint_port(port, &format!("ports[{}]", i)));

        if let Some(name) = &port.name {
            if !name.is_empty() {
                if !seen_port_names.insert(name.clone()) {
                    errors.push(ValidationError::duplicate(
                        format!("ports[{}].name", i),
                        name,
                    ));
                }
            }
        }
    }

    errors
}

/// Validates an Endpoint.
fn validate_endpoint(endpoint: &Endpoint, field: &str, address_type: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Addresses is required and must be non-empty
    if endpoint.addresses.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.addresses", field),
            "at least one address is required",
        ));
    }

    // Validate addresses count
    if endpoint.addresses.len() > MAX_ADDRESSES_PER_ENDPOINT {
        errors.push(ValidationError::too_long(
            format!("{}.addresses", field),
            MAX_ADDRESSES_PER_ENDPOINT,
            endpoint.addresses.len(),
        ));
    }

    // Validate addresses based on address type
    for (i, addr) in endpoint.addresses.iter().enumerate() {
        errors.extend(validate_address(
            addr,
            &format!("{}.addresses[{}]", field, i),
            address_type,
        ));
    }

    // Validate hostname (DNS subdomain if specified)
    if let Some(hostname) = &endpoint.hostname {
        if hostname.len() > 253 {
            errors.push(ValidationError::too_long(
                format!("{}.hostname", field),
                253,
                hostname.len(),
            ));
        }
    }

    // Validate nodeName length
    if let Some(node_name) = &endpoint.node_name {
        if node_name.len() > 253 {
            errors.push(ValidationError::too_long(
                format!("{}.nodeName", field),
                253,
                node_name.len(),
            ));
        }
    }

    // Validate zone length
    if let Some(zone) = &endpoint.zone {
        if zone.len() > MAX_ZONE_NAME_LENGTH {
            errors.push(ValidationError::too_long(
                format!("{}.zone", field),
                MAX_ZONE_NAME_LENGTH,
                zone.len(),
            ));
        }
    }

    // Validate hints
    if let Some(hints) = &endpoint.hints {
        for (i, for_zone) in hints.for_zones.iter().enumerate() {
            if for_zone.name.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.hints.forZones[{}].name", field, i),
                    "zone name is required",
                ));
            } else if for_zone.name.len() > MAX_ZONE_NAME_LENGTH {
                errors.push(ValidationError::too_long(
                    format!("{}.hints.forZones[{}].name", field, i),
                    MAX_ZONE_NAME_LENGTH,
                    for_zone.name.len(),
                ));
            }
        }
    }

    errors
}

/// Validates an address based on address type.
fn validate_address(addr: &str, field: &str, address_type: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if addr.is_empty() {
        errors.push(ValidationError::required(field, "address cannot be empty"));
        return errors;
    }

    match address_type {
        "IPv4" => {
            if !is_valid_ipv4(addr) {
                errors.push(ValidationError::invalid(
                    field,
                    format!("must be a valid IPv4 address, got: {}", addr),
                ));
            }
        }
        "IPv6" => {
            if !is_valid_ipv6(addr) {
                errors.push(ValidationError::invalid(
                    field,
                    format!("must be a valid IPv6 address, got: {}", addr),
                ));
            }
        }
        "FQDN" => {
            if !is_valid_fqdn(addr) {
                errors.push(ValidationError::invalid(
                    field,
                    format!("must be a valid FQDN, got: {}", addr),
                ));
            }
        }
        _ => {
            // Unknown address type, already validated above
        }
    }

    errors
}

/// Validates an EndpointPort.
fn validate_endpoint_port(port: &EndpointPort, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate port name (must be DNS label if specified)
    if let Some(name) = &port.name {
        if !name.is_empty() {
            if name.len() > 63 {
                errors.push(ValidationError::too_long(
                    format!("{}.name", field),
                    63,
                    name.len(),
                ));
            }
            if !is_valid_dns_label(name) {
                errors.push(ValidationError::invalid(
                    format!("{}.name", field),
                    format!("must be a valid DNS label: {}", name),
                ));
            }
        }
    }

    // Validate protocol
    if let Some(protocol) = &port.protocol {
        if !VALID_PROTOCOLS.contains(&protocol.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.protocol", field),
                protocol,
                VALID_PROTOCOLS,
            ));
        }
    }

    // Validate port number
    if let Some(port_num) = port.port {
        if port_num < 1 || port_num > 65535 {
            errors.push(ValidationError::out_of_range(
                format!("{}.port", field),
                1,
                65535,
                port_num as i64,
            ));
        }
    }

    // Validate appProtocol length
    if let Some(app_protocol) = &port.app_protocol {
        if app_protocol.len() > 255 {
            errors.push(ValidationError::too_long(
                format!("{}.appProtocol", field),
                255,
                app_protocol.len(),
            ));
        }
    }

    errors
}

// =============================================================================
// Helper validation functions
// =============================================================================

fn is_valid_ipv4(addr: &str) -> bool {
    addr.parse::<std::net::Ipv4Addr>().is_ok()
}

fn is_valid_ipv6(addr: &str) -> bool {
    addr.parse::<std::net::Ipv6Addr>().is_ok()
}

fn is_valid_fqdn(fqdn: &str) -> bool {
    if fqdn.is_empty() || fqdn.len() > 253 {
        return false;
    }

    // Each label must be valid
    for label in fqdn.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        // Must start with alphanumeric
        if !label.chars().next().map(|c| c.is_ascii_alphanumeric()).unwrap_or(false) {
            return false;
        }
        // Must end with alphanumeric
        if !label.chars().last().map(|c| c.is_ascii_alphanumeric()).unwrap_or(false) {
            return false;
        }
        // Can only contain alphanumeric and hyphens
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

fn is_valid_dns_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }

    // Must start with lowercase alphanumeric
    if !label.chars().next().map(|c| c.is_ascii_lowercase() || c.is_ascii_digit()).unwrap_or(false) {
        return false;
    }

    // Must end with lowercase alphanumeric
    if !label.chars().last().map(|c| c.is_ascii_lowercase() || c.is_ascii_digit()).unwrap_or(false) {
        return false;
    }

    // Can only contain lowercase alphanumeric and hyphens
    label.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::discovery::v1::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_endpoint_slice_valid_ipv4() {
        let es = EndpointSlice {
            metadata: ObjectMeta {
                name: "my-service-abc12".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string(), "10.0.0.2".to_string()],
                conditions: Some(EndpointConditions {
                    ready: Some(true),
                    serving: Some(true),
                    terminating: Some(false),
                }),
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                name: Some("http".to_string()),
                protocol: Some("TCP".to_string()),
                port: Some(80),
                app_protocol: Some("http".to_string()),
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_endpoint_slice_valid_ipv6() {
        let es = EndpointSlice {
            metadata: ObjectMeta {
                name: "my-service-def34".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            address_type: "IPv6".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["::1".to_string(), "2001:db8::1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                port: Some(8080),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_endpoint_slice_valid_fqdn() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("external-service"),
            address_type: "FQDN".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["api.example.com".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_endpoint_slice_missing_address_type() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: String::new(),
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "addressType"));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_address_type() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "InvalidType".to_string(),
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "addressType"));
    }

    #[test]
    fn test_validate_endpoint_slice_empty_addresses() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec![],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("addresses")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_ipv4() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["not-an-ip".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("IPv4")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_ipv6() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv6".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["192.168.1.1".to_string()], // IPv4 in IPv6 slice
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("IPv6")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_fqdn() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "FQDN".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["-invalid.example.com".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("FQDN")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_port_range() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                port: Some(0), // Invalid port
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("port")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_port_too_high() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                port: Some(70000), // Too high
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("port")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_protocol() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                protocol: Some("HTTP".to_string()), // Not a valid transport protocol
                port: Some(80),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("protocol")));
    }

    #[test]
    fn test_validate_endpoint_slice_duplicate_port_names() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![
                EndpointPort {
                    name: Some("http".to_string()),
                    port: Some(80),
                    protocol: Some("TCP".to_string()),
                    ..Default::default()
                },
                EndpointPort {
                    name: Some("http".to_string()), // Duplicate name
                    port: Some(8080),
                    protocol: Some("TCP".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn test_validate_endpoint_slice_invalid_port_name() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                name: Some("Invalid-Port-Name".to_string()), // Uppercase not allowed
                port: Some(80),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("name")));
    }

    #[test]
    fn test_validate_endpoint_slice_with_hints() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                hints: Some(EndpointHints {
                    for_zones: vec![ForZone {
                        name: "us-west-2a".to_string(),
                    }],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_endpoint_slice_empty_zone_name() {
        let es = EndpointSlice {
            metadata: ObjectMeta::named("test-slice"),
            address_type: "IPv4".to_string(),
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                hints: Some(EndpointHints {
                    for_zones: vec![ForZone {
                        name: String::new(), // Empty zone name
                    }],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_endpoint_slice(&es);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("forZones")));
    }

    #[test]
    fn test_is_valid_dns_label() {
        assert!(is_valid_dns_label("http"));
        assert!(is_valid_dns_label("my-port"));
        assert!(is_valid_dns_label("port80"));
        assert!(is_valid_dns_label("a"));

        assert!(!is_valid_dns_label(""));
        assert!(!is_valid_dns_label("-invalid"));
        assert!(!is_valid_dns_label("invalid-"));
        assert!(!is_valid_dns_label("UPPERCASE"));
        assert!(!is_valid_dns_label("with.dot"));
        assert!(!is_valid_dns_label("with_underscore"));
    }

    #[test]
    fn test_is_valid_fqdn() {
        assert!(is_valid_fqdn("example.com"));
        assert!(is_valid_fqdn("api.example.com"));
        assert!(is_valid_fqdn("my-service.namespace.svc.cluster.local"));

        assert!(!is_valid_fqdn(""));
        assert!(!is_valid_fqdn("-invalid.com"));
        assert!(!is_valid_fqdn("invalid-.com"));
        assert!(!is_valid_fqdn("with spaces.com"));
    }
}
