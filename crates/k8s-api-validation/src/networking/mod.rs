//! Networking API validation
//!
//! This module provides validation for networking API types including:
//! - Ingress
//! - IngressClass
//! - NetworkPolicy

use crate::common::{
    validate_dns_subdomain_name, validate_object_meta, validate_port_number, validate_protocol,
};
use crate::{ValidationError, ValidationResult};
use k8s_api::networking::v1::{
    HTTPIngressPath, Ingress, IngressBackend, IngressClass, IngressRule, IngressSpec,
    NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyPort,
    NetworkPolicySpec,
};

/// Valid path types for Ingress
const VALID_PATH_TYPES: &[&str] = &["Exact", "Prefix", "ImplementationSpecific"];

/// Valid policy types for NetworkPolicy
const VALID_POLICY_TYPES: &[&str] = &["Ingress", "Egress"];

// =============================================================================
// Ingress Validation
// =============================================================================

/// Validates an Ingress resource.
pub fn validate_ingress(ingress: &Ingress) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&ingress.metadata, "metadata", true));

    if let Some(spec) = &ingress.spec {
        errors.extend(validate_ingress_spec(spec, "spec"));
    }

    errors
}

/// Validates an IngressSpec.
pub fn validate_ingress_spec(spec: &IngressSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate default backend if present
    if let Some(backend) = &spec.default_backend {
        errors.extend(validate_ingress_backend(
            backend,
            &format!("{}.defaultBackend", field),
        ));
    }

    // Validate TLS entries
    for (i, tls) in spec.tls.iter().enumerate() {
        let tls_field = format!("{}.tls[{}]", field, i);

        // Validate hosts
        for (j, host) in tls.hosts.iter().enumerate() {
            if !host.is_empty() {
                errors.extend(validate_ingress_host(
                    host,
                    &format!("{}.hosts[{}]", tls_field, j),
                ));
            }
        }
    }

    // Validate rules
    for (i, rule) in spec.rules.iter().enumerate() {
        errors.extend(validate_ingress_rule(rule, &format!("{}.rules[{}]", field, i)));
    }

    errors
}

/// Validates an IngressRule.
pub fn validate_ingress_rule(rule: &IngressRule, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate host if present
    if !rule.host.is_empty() {
        errors.extend(validate_ingress_host(&rule.host, &format!("{}.host", field)));
    }

    // Validate HTTP paths
    if let Some(http) = &rule.ingress_rule_value.http {
        if http.paths.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.http.paths", field),
                "at least one path is required",
            ));
        }

        for (i, path) in http.paths.iter().enumerate() {
            errors.extend(validate_http_ingress_path(
                path,
                &format!("{}.http.paths[{}]", field, i),
            ));
        }
    }

    errors
}

/// Validates an HTTPIngressPath.
pub fn validate_http_ingress_path(path: &HTTPIngressPath, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate pathType
    if let Some(path_type) = &path.path_type {
        if !VALID_PATH_TYPES.contains(&path_type.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.pathType", field),
                path_type,
                VALID_PATH_TYPES,
            ));
        }

        // Validate path format based on pathType
        if !path.path.is_empty() {
            if path_type == "Exact" || path_type == "Prefix" {
                if !path.path.starts_with('/') {
                    errors.push(ValidationError::invalid(
                        format!("{}.path", field),
                        "must start with '/' when pathType is Exact or Prefix",
                    ));
                }
            }
        }
    } else {
        errors.push(ValidationError::required(
            format!("{}.pathType", field),
            "pathType is required",
        ));
    }

    // Validate backend
    errors.extend(validate_ingress_backend(
        &path.backend,
        &format!("{}.backend", field),
    ));

    errors
}

/// Validates an IngressBackend.
pub fn validate_ingress_backend(backend: &IngressBackend, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Must have either service or resource
    if backend.service.is_none() && backend.resource.is_none() {
        errors.push(ValidationError::required(
            field,
            "one of service or resource is required",
        ));
    }

    // Cannot have both
    if backend.service.is_some() && backend.resource.is_some() {
        errors.push(ValidationError::invalid(
            field,
            "cannot specify both service and resource",
        ));
    }

    // Validate service backend
    if let Some(service) = &backend.service {
        if service.name.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.service.name", field),
                "service name is required",
            ));
        } else {
            errors.extend(validate_dns_subdomain_name(
                &service.name,
                &format!("{}.service.name", field),
            ));
        }

        // Validate port if present
        if let Some(port) = &service.port {
            // Must have either name or number
            if port.name.is_empty() && port.number == 0 {
                errors.push(ValidationError::required(
                    format!("{}.service.port", field),
                    "port name or number is required",
                ));
            }

            if port.number != 0 {
                errors.extend(validate_port_number(
                    port.number,
                    &format!("{}.service.port.number", field),
                ));
            }
        }
    }

    // Validate resource backend
    if let Some(resource) = &backend.resource {
        if resource.kind.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.resource.kind", field),
                "resource kind is required",
            ));
        }

        if resource.name.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.resource.name", field),
                "resource name is required",
            ));
        }
    }

    errors
}

/// Validates an Ingress host.
fn validate_ingress_host(host: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Host can be a wildcard like "*.example.com"
    if host.starts_with("*.") {
        let domain = &host[2..];
        if domain.is_empty() {
            errors.push(ValidationError::invalid(
                field,
                "wildcard host must have a domain after '*.'",
            ));
        }
        // Validate the domain part
        for part in domain.split('.') {
            if part.is_empty() {
                errors.push(ValidationError::invalid(
                    field,
                    "host contains empty domain label",
                ));
            }
        }
    } else {
        // Regular host validation - must be valid hostname
        for part in host.split('.') {
            if part.is_empty() {
                errors.push(ValidationError::invalid(
                    field,
                    "host contains empty domain label",
                ));
            }
        }
    }

    errors
}

// =============================================================================
// IngressClass Validation
// =============================================================================

/// Validates an IngressClass resource.
pub fn validate_ingress_class(ingress_class: &IngressClass) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&ingress_class.metadata, "metadata", true));

    if let Some(spec) = &ingress_class.spec {
        if spec.controller.is_empty() {
            errors.push(ValidationError::required(
                "spec.controller",
                "controller is required",
            ));
        }

        // Validate parameters if present
        if let Some(params) = &spec.parameters {
            if params.kind.is_empty() {
                errors.push(ValidationError::required(
                    "spec.parameters.kind",
                    "kind is required",
                ));
            }

            if params.name.is_empty() {
                errors.push(ValidationError::required(
                    "spec.parameters.name",
                    "name is required",
                ));
            }

            // Validate scope
            if let Some(scope) = &params.scope {
                if scope != "Cluster" && scope != "Namespace" {
                    errors.push(ValidationError::not_supported(
                        "spec.parameters.scope",
                        scope,
                        &["Cluster", "Namespace"],
                    ));
                }
            }
        }
    }

    errors
}

// =============================================================================
// NetworkPolicy Validation
// =============================================================================

/// Validates a NetworkPolicy resource.
pub fn validate_network_policy(policy: &NetworkPolicy) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&policy.metadata, "metadata", true));

    if let Some(spec) = &policy.spec {
        errors.extend(validate_network_policy_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a NetworkPolicySpec.
pub fn validate_network_policy_spec(spec: &NetworkPolicySpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate policyTypes
    for (i, policy_type) in spec.policy_types.iter().enumerate() {
        if !VALID_POLICY_TYPES.contains(&policy_type.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.policyTypes[{}]", field, i),
                policy_type,
                VALID_POLICY_TYPES,
            ));
        }
    }

    // Validate ingress rules
    for (i, rule) in spec.ingress.iter().enumerate() {
        errors.extend(validate_network_policy_ingress_rule(
            rule,
            &format!("{}.ingress[{}]", field, i),
        ));
    }

    // Validate egress rules
    for (i, rule) in spec.egress.iter().enumerate() {
        errors.extend(validate_network_policy_egress_rule(
            rule,
            &format!("{}.egress[{}]", field, i),
        ));
    }

    errors
}

pub mod internal {
    use super::*;
    use k8s_api::networking::internal as api;

    pub fn validate_ingress(ingress: &api::Ingress) -> ValidationResult {
        crate::internal::validate_with(ingress, "ingress", super::validate_ingress)
    }

    pub fn validate_ingress_spec(spec: &api::IngressSpec, field: &str) -> ValidationResult {
        crate::internal::validate_with(spec, field, |external_spec| {
            super::validate_ingress_spec(external_spec, field)
        })
    }

    pub fn validate_ingress_rule(rule: &api::IngressRule, field: &str) -> ValidationResult {
        crate::internal::validate_with(rule, field, |external_rule| {
            super::validate_ingress_rule(external_rule, field)
        })
    }

    pub fn validate_http_ingress_path(path: &api::HTTPIngressPath, field: &str) -> ValidationResult {
        crate::internal::validate_with(path, field, |external_path| {
            super::validate_http_ingress_path(external_path, field)
        })
    }

    pub fn validate_ingress_backend(
        backend: &api::IngressBackend,
        field: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(backend, field, |external_backend| {
            super::validate_ingress_backend(external_backend, field)
        })
    }

    pub fn validate_ingress_class(ingress_class: &api::IngressClass) -> ValidationResult {
        crate::internal::validate_with(
            ingress_class,
            "ingressClass",
            super::validate_ingress_class,
        )
    }

    pub fn validate_network_policy(policy: &api::NetworkPolicy) -> ValidationResult {
        crate::internal::validate_with(policy, "networkPolicy", super::validate_network_policy)
    }

    pub fn validate_network_policy_spec(
        spec: &api::NetworkPolicySpec,
        field: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field, |external_spec| {
            super::validate_network_policy_spec(external_spec, field)
        })
    }
}

/// Validates a NetworkPolicyIngressRule.
fn validate_network_policy_ingress_rule(
    rule: &NetworkPolicyIngressRule,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate ports
    for (i, port) in rule.ports.iter().enumerate() {
        errors.extend(validate_network_policy_port(
            port,
            &format!("{}.ports[{}]", field, i),
        ));
    }

    // Validate from peers
    for (i, peer) in rule.from.iter().enumerate() {
        errors.extend(validate_network_policy_peer(
            peer,
            &format!("{}.from[{}]", field, i),
        ));
    }

    errors
}

/// Validates a NetworkPolicyEgressRule.
fn validate_network_policy_egress_rule(
    rule: &NetworkPolicyEgressRule,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate ports
    for (i, port) in rule.ports.iter().enumerate() {
        errors.extend(validate_network_policy_port(
            port,
            &format!("{}.ports[{}]", field, i),
        ));
    }

    // Validate to peers
    for (i, peer) in rule.to.iter().enumerate() {
        errors.extend(validate_network_policy_peer(
            peer,
            &format!("{}.to[{}]", field, i),
        ));
    }

    errors
}

/// Validates a NetworkPolicyPort.
fn validate_network_policy_port(port: &NetworkPolicyPort, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate protocol
    if let Some(protocol) = &port.protocol {
        errors.extend(validate_protocol(protocol, &format!("{}.protocol", field)));
    }

    // Validate port if it's a number
    if let Some(port_value) = &port.port {
        if let Some(port_num) = port_value.as_i64() {
            errors.extend(validate_port_number(
                port_num as i32,
                &format!("{}.port", field),
            ));
        }
    }

    // Validate endPort
    if let Some(end_port) = port.end_port {
        errors.extend(validate_port_number(end_port, &format!("{}.endPort", field)));

        // endPort requires port to be set
        if port.port.is_none() {
            errors.push(ValidationError::forbidden(
                format!("{}.endPort", field),
                "endPort cannot be set without port",
            ));
        }

        // endPort must be >= port
        if let Some(port_value) = &port.port {
            if let Some(port_num) = port_value.as_i64() {
                if end_port < port_num as i32 {
                    errors.push(ValidationError::invalid(
                        format!("{}.endPort", field),
                        "endPort must be greater than or equal to port",
                    ));
                }
            }
        }
    }

    errors
}

/// Validates a NetworkPolicyPeer.
fn validate_network_policy_peer(
    peer: &k8s_api::networking::v1::NetworkPolicyPeer,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate ipBlock if present
    if let Some(ip_block) = &peer.ip_block {
        errors.extend(validate_cidr(&ip_block.cidr, &format!("{}.ipBlock.cidr", field)));

        for (i, except) in ip_block.except.iter().enumerate() {
            errors.extend(validate_cidr(
                except,
                &format!("{}.ipBlock.except[{}]", field, i),
            ));
        }
    }

    errors
}

/// Validates a CIDR notation string.
fn validate_cidr(cidr: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if cidr.is_empty() {
        errors.push(ValidationError::required(field, "CIDR is required"));
        return errors;
    }

    // Split into IP and prefix length
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        errors.push(ValidationError::invalid(
            field,
            "CIDR must be in format IP/prefixLength",
        ));
        return errors;
    }

    let ip = parts[0];
    let prefix = parts[1];

    // Check if it's IPv4 or IPv6
    let is_ipv6 = ip.contains(':');

    // Validate prefix length
    match prefix.parse::<u8>() {
        Ok(prefix_len) => {
            if is_ipv6 {
                if prefix_len > 128 {
                    errors.push(ValidationError::out_of_range(
                        format!("{} (prefix)", field),
                        0,
                        128,
                        prefix_len as i64,
                    ));
                }
            } else if prefix_len > 32 {
                errors.push(ValidationError::out_of_range(
                    format!("{} (prefix)", field),
                    0,
                    32,
                    prefix_len as i64,
                ));
            }
        }
        Err(_) => {
            errors.push(ValidationError::invalid(
                format!("{} (prefix)", field),
                "prefix length must be a valid number",
            ));
        }
    }

    // Basic IP validation
    if is_ipv6 {
        // IPv6 basic check - should have colons
        if ip.matches(':').count() < 2 || ip.matches(':').count() > 7 {
            errors.push(ValidationError::invalid(
                format!("{} (IP)", field),
                "invalid IPv6 address format",
            ));
        }
    } else {
        // IPv4 basic check
        let octets: Vec<&str> = ip.split('.').collect();
        if octets.len() != 4 {
            errors.push(ValidationError::invalid(
                format!("{} (IP)", field),
                "IPv4 address must have 4 octets",
            ));
        } else {
            for (i, octet) in octets.iter().enumerate() {
                match octet.parse::<u8>() {
                    Ok(_) => {}
                    Err(_) => {
                        errors.push(ValidationError::invalid(
                            format!("{} (IP octet {})", field, i),
                            "each octet must be a number between 0 and 255",
                        ));
                    }
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::networking::v1::*;
    use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_ingress_valid() {
        let ingress = Ingress {
            metadata: ObjectMeta::named("test-ingress"),
            spec: Some(IngressSpec {
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    ingress_rule_value: IngressRuleValue {
                        http: Some(HTTPIngressRuleValue {
                            paths: vec![HTTPIngressPath {
                                path: "/".to_string(),
                                path_type: Some("Prefix".to_string()),
                                backend: IngressBackend {
                                    service: Some(IngressServiceBackend {
                                        name: "my-service".to_string(),
                                        port: Some(ServiceBackendPort {
                                            number: 80,
                                            ..Default::default()
                                        }),
                                    }),
                                    ..Default::default()
                                },
                            }],
                        }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_ingress(&ingress);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_ingress_invalid_path_type() {
        let ingress = Ingress {
            metadata: ObjectMeta::named("test-ingress"),
            spec: Some(IngressSpec {
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    ingress_rule_value: IngressRuleValue {
                        http: Some(HTTPIngressRuleValue {
                            paths: vec![HTTPIngressPath {
                                path: "/".to_string(),
                                path_type: Some("Invalid".to_string()),
                                backend: IngressBackend {
                                    service: Some(IngressServiceBackend {
                                        name: "my-service".to_string(),
                                        port: Some(ServiceBackendPort {
                                            number: 80,
                                            ..Default::default()
                                        }),
                                    }),
                                    ..Default::default()
                                },
                            }],
                        }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("pathType")));
    }

    #[test]
    fn test_validate_ingress_missing_backend() {
        let ingress = Ingress {
            metadata: ObjectMeta::named("test-ingress"),
            spec: Some(IngressSpec {
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    ingress_rule_value: IngressRuleValue {
                        http: Some(HTTPIngressRuleValue {
                            paths: vec![HTTPIngressPath {
                                path: "/".to_string(),
                                path_type: Some("Prefix".to_string()),
                                backend: IngressBackend::default(), // No backend
                            }],
                        }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("backend")));
    }

    #[test]
    fn test_validate_ingress_wildcard_host() {
        let ingress = Ingress {
            metadata: ObjectMeta::named("test-ingress"),
            spec: Some(IngressSpec {
                rules: vec![IngressRule {
                    host: "*.example.com".to_string(),
                    ingress_rule_value: IngressRuleValue {
                        http: Some(HTTPIngressRuleValue {
                            paths: vec![HTTPIngressPath {
                                path: "/".to_string(),
                                path_type: Some("Prefix".to_string()),
                                backend: IngressBackend {
                                    service: Some(IngressServiceBackend {
                                        name: "my-service".to_string(),
                                        port: Some(ServiceBackendPort {
                                            number: 80,
                                            ..Default::default()
                                        }),
                                    }),
                                    ..Default::default()
                                },
                            }],
                        }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_ingress(&ingress);
        assert!(errors.is_empty(), "Wildcard host should be valid: {:?}", errors);
    }

    #[test]
    fn test_validate_ingress_class_valid() {
        let ingress_class = IngressClass {
            metadata: ObjectMeta::named("nginx"),
            spec: Some(IngressClassSpec {
                controller: "k8s.io/ingress-nginx".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_ingress_class_missing_controller() {
        let ingress_class = IngressClass {
            metadata: ObjectMeta::named("test"),
            spec: Some(IngressClassSpec::default()),
            ..Default::default()
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("controller")));
    }

    #[test]
    fn test_validate_network_policy_valid() {
        let mut match_labels = BTreeMap::new();
        match_labels.insert("app".to_string(), "web".to_string());

        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector {
                    match_labels,
                    ..Default::default()
                },
                policy_types: vec!["Ingress".to_string()],
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(serde_json::json!(80)),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_network_policy_invalid_policy_type() {
        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                policy_types: vec!["Invalid".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("policyTypes")));
    }

    #[test]
    fn test_validate_network_policy_invalid_protocol() {
        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("INVALID".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("protocol")));
    }

    #[test]
    fn test_validate_cidr_valid_ipv4() {
        assert!(validate_cidr("10.0.0.0/8", "test").is_empty());
        assert!(validate_cidr("192.168.1.0/24", "test").is_empty());
        assert!(validate_cidr("0.0.0.0/0", "test").is_empty());
    }

    #[test]
    fn test_validate_cidr_valid_ipv6() {
        assert!(validate_cidr("2001:db8::/32", "test").is_empty());
        assert!(validate_cidr("::/0", "test").is_empty());
    }

    #[test]
    fn test_validate_cidr_invalid() {
        // Missing prefix
        assert!(!validate_cidr("192.168.1.0", "test").is_empty());

        // Invalid prefix length for IPv4
        assert!(!validate_cidr("192.168.1.0/33", "test").is_empty());

        // Invalid octet
        assert!(!validate_cidr("192.168.256.0/24", "test").is_empty());
    }

    #[test]
    fn test_validate_network_policy_port_range() {
        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        port: Some(serde_json::json!(80)),
                        end_port: Some(90),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(errors.is_empty(), "Port range should be valid: {:?}", errors);
    }

    #[test]
    fn test_validate_network_policy_invalid_port_range() {
        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        port: Some(serde_json::json!(90)),
                        end_port: Some(80), // end_port < port
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("endPort")));
    }

    #[test]
    fn test_validate_network_policy_with_ipblock() {
        let policy = NetworkPolicy {
            metadata: ObjectMeta::named("test-policy"),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    from: vec![NetworkPolicyPeer {
                        ip_block: Some(IPBlock {
                            cidr: "10.0.0.0/8".to_string(),
                            except: vec!["10.1.0.0/16".to_string()],
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_network_policy(&policy);
        assert!(errors.is_empty(), "IPBlock should be valid: {:?}", errors);
    }
}
