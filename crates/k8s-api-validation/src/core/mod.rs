//! Core v1 validation
//!
//! This module provides validation for core/v1 API types.

use crate::common::{
    validate_dns_label, validate_dns_subdomain_name, validate_env_var_name, validate_labels,
    validate_object_meta, validate_port_name, validate_port_number, validate_protocol,
};
use crate::{ValidationError, ValidationResult};
use k8s_api::core::v1::{
    ConfigMap, Container, ContainerPort, EnvVar, Namespace, Pod, PodSpec, Secret, Service,
    ServicePort, ServiceSpec,
};
use std::collections::HashSet;

// =============================================================================
// Pod Validation
// =============================================================================

/// Validates a Pod.
pub fn validate_pod(pod: &Pod) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&pod.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &pod.spec {
        errors.extend(validate_pod_spec(spec, "spec"));
    }

    errors
}

/// Validates a PodSpec.
pub fn validate_pod_spec(spec: &PodSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Containers must not be empty
    if spec.containers.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.containers", field_path),
            "at least one container is required",
        ));
    }

    // Check for duplicate container names
    let mut container_names = HashSet::new();
    for (i, container) in spec.containers.iter().enumerate() {
        if !container.name.is_empty() && !container_names.insert(&container.name) {
            errors.push(ValidationError::duplicate(
                format!("{}.containers[{}].name", field_path, i),
                &container.name,
            ));
        }
        errors.extend(validate_container(
            container,
            &format!("{}.containers[{}]", field_path, i),
        ));
    }

    // Validate init containers
    let mut init_names = HashSet::new();
    for (i, container) in spec.init_containers.iter().enumerate() {
        if !container.name.is_empty() && !init_names.insert(&container.name) {
            errors.push(ValidationError::duplicate(
                format!("{}.initContainers[{}].name", field_path, i),
                &container.name,
            ));
        }
        // Init container names must also be unique across all containers
        if !container.name.is_empty() && container_names.contains(&container.name) {
            errors.push(ValidationError::duplicate(
                format!("{}.initContainers[{}].name", field_path, i),
                format!(
                    "{} - init container name conflicts with container name",
                    container.name
                ),
            ));
        }
        errors.extend(validate_container(
            container,
            &format!("{}.initContainers[{}]", field_path, i),
        ));
    }

    // Validate restart policy
    if !spec.restart_policy.is_empty() {
        let valid_policies = ["Always", "OnFailure", "Never"];
        if !valid_policies.contains(&spec.restart_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.restartPolicy", field_path),
                &spec.restart_policy,
                &valid_policies,
            ));
        }
    }

    // Validate DNS policy
    if !spec.dns_policy.is_empty() {
        let valid_policies = [
            "ClusterFirst",
            "ClusterFirstWithHostNet",
            "Default",
            "None",
        ];
        if !valid_policies.contains(&spec.dns_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.dnsPolicy", field_path),
                &spec.dns_policy,
                &valid_policies,
            ));
        }
    }

    // Validate service account name
    if !spec.service_account_name.is_empty() {
        errors.extend(validate_dns_subdomain_name(
            &spec.service_account_name,
            &format!("{}.serviceAccountName", field_path),
        ));
    }

    // Validate node name
    if !spec.node_name.is_empty() {
        errors.extend(validate_dns_subdomain_name(
            &spec.node_name,
            &format!("{}.nodeName", field_path),
        ));
    }

    // Validate volumes
    let mut volume_names = HashSet::new();
    for (i, volume) in spec.volumes.iter().enumerate() {
        if volume.name.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.volumes[{}].name", field_path, i),
                "volume name is required",
            ));
        } else {
            if !volume_names.insert(&volume.name) {
                errors.push(ValidationError::duplicate(
                    format!("{}.volumes[{}].name", field_path, i),
                    &volume.name,
                ));
            }
            errors.extend(validate_dns_label(
                &volume.name,
                &format!("{}.volumes[{}].name", field_path, i),
            ));
        }
    }

    // Validate termination grace period
    if let Some(grace_period) = spec.termination_grace_period_seconds {
        if grace_period < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.terminationGracePeriodSeconds", field_path),
                "must be non-negative",
            ));
        }
    }

    // Validate active deadline seconds
    if let Some(deadline) = spec.active_deadline_seconds {
        if deadline < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.activeDeadlineSeconds", field_path),
                "must be non-negative",
            ));
        }
    }

    // Validate priority
    if let Some(priority) = spec.priority {
        // Priority can be any i32 value, but we validate it's set intentionally
        if priority < i32::MIN || priority > i32::MAX {
            errors.push(ValidationError::out_of_range(
                format!("{}.priority", field_path),
                i32::MIN as i64,
                i32::MAX as i64,
                priority as i64,
            ));
        }
    }

    errors
}

/// Validates a Container.
pub fn validate_container(container: &Container, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Name is required
    if container.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field_path),
            "container name is required",
        ));
    } else {
        errors.extend(validate_dns_label(
            &container.name,
            &format!("{}.name", field_path),
        ));
    }

    // Image is required
    if container.image.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.image", field_path),
            "container image is required",
        ));
    }

    // Validate ports
    let mut port_names = HashSet::new();
    let mut host_ports = HashSet::new();
    for (i, port) in container.ports.iter().enumerate() {
        errors.extend(validate_container_port(
            port,
            &format!("{}.ports[{}]", field_path, i),
        ));

        // Check for duplicate port names
        if !port.name.is_empty() && !port_names.insert(&port.name) {
            errors.push(ValidationError::duplicate(
                format!("{}.ports[{}].name", field_path, i),
                &port.name,
            ));
        }

        // Check for duplicate host ports (if specified)
        if let Some(host_port) = port.host_port {
            if host_port > 0 {
                let key = (host_port, port.protocol.clone());
                if !host_ports.insert(key) {
                    errors.push(ValidationError::duplicate(
                        format!("{}.ports[{}].hostPort", field_path, i),
                        format!("{}:{}", host_port, port.protocol),
                    ));
                }
            }
        }
    }

    // Validate environment variables
    let mut env_names = HashSet::new();
    for (i, env) in container.env.iter().enumerate() {
        errors.extend(validate_env_var(env, &format!("{}.env[{}]", field_path, i)));

        // Check for duplicate env var names
        if !env.name.is_empty() && !env_names.insert(&env.name) {
            // Duplicate env vars are allowed (later ones override earlier ones)
            // but we don't report an error
        }
    }

    // Validate image pull policy
    if !container.image_pull_policy.is_empty() {
        let valid_policies = ["Always", "IfNotPresent", "Never"];
        if !valid_policies.contains(&container.image_pull_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.imagePullPolicy", field_path),
                &container.image_pull_policy,
                &valid_policies,
            ));
        }
    }

    // Validate termination message policy
    if !container.termination_message_policy.is_empty() {
        let valid_policies = ["File", "FallbackToLogsOnError"];
        if !valid_policies.contains(&container.termination_message_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.terminationMessagePolicy", field_path),
                &container.termination_message_policy,
                &valid_policies,
            ));
        }
    }

    errors
}

fn validate_container_port(port: &ContainerPort, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate container port
    errors.extend(validate_port_number(
        port.container_port,
        &format!("{}.containerPort", field_path),
    ));

    // Validate host port if specified
    if let Some(host_port) = port.host_port {
        if host_port != 0 {
            errors.extend(validate_port_number(
                host_port,
                &format!("{}.hostPort", field_path),
            ));
        }
    }

    // Validate port name if specified
    if !port.name.is_empty() {
        errors.extend(validate_port_name(
            &port.name,
            &format!("{}.name", field_path),
        ));
    }

    // Validate protocol
    errors.extend(validate_protocol(
        &port.protocol,
        &format!("{}.protocol", field_path),
    ));

    errors
}

fn validate_env_var(env: &EnvVar, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Name is required
    if env.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field_path),
            "environment variable name is required",
        ));
    } else {
        errors.extend(validate_env_var_name(
            &env.name,
            &format!("{}.name", field_path),
        ));
    }

    errors
}

// =============================================================================
// Namespace Validation
// =============================================================================

/// Validates a Namespace.
pub fn validate_namespace(namespace: &Namespace) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&namespace.metadata, "metadata", true));

    // Namespace names must be DNS labels
    if !namespace.metadata.name.is_empty() {
        errors.extend(validate_dns_label(
            &namespace.metadata.name,
            "metadata.name",
        ));
    }

    errors
}

// =============================================================================
// Service Validation
// =============================================================================

/// Validates a Service.
pub fn validate_service(service: &Service) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&service.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &service.spec {
        errors.extend(validate_service_spec(spec, "spec"));
    }

    errors
}

/// Validates a ServiceSpec.
pub fn validate_service_spec(spec: &ServiceSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate type
    if !spec.service_type.is_empty() {
        let valid_types = ["ClusterIP", "NodePort", "LoadBalancer", "ExternalName"];
        if !valid_types.contains(&spec.service_type.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.type", field_path),
                &spec.service_type,
                &valid_types,
            ));
        }
    }

    // Validate selector labels
    errors.extend(validate_labels(
        &spec.selector,
        &format!("{}.selector", field_path),
    ));

    // Validate ports
    let mut port_names = HashSet::new();
    for (i, port) in spec.ports.iter().enumerate() {
        errors.extend(validate_service_port(
            port,
            &format!("{}.ports[{}]", field_path, i),
        ));

        // Check for duplicate port names
        if !port.name.is_empty() && !port_names.insert(&port.name) {
            errors.push(ValidationError::duplicate(
                format!("{}.ports[{}].name", field_path, i),
                &port.name,
            ));
        }
    }

    // Validate session affinity
    if !spec.session_affinity.is_empty() {
        let valid_affinities = ["None", "ClientIP"];
        if !valid_affinities.contains(&spec.session_affinity.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.sessionAffinity", field_path),
                &spec.session_affinity,
                &valid_affinities,
            ));
        }
    }

    // Validate external traffic policy
    if !spec.external_traffic_policy.is_empty() {
        let valid_policies = ["Cluster", "Local"];
        if !valid_policies.contains(&spec.external_traffic_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.externalTrafficPolicy", field_path),
                &spec.external_traffic_policy,
                &valid_policies,
            ));
        }
    }

    // Validate internal traffic policy
    if let Some(ref policy) = spec.internal_traffic_policy {
        let valid_policies = ["Cluster", "Local"];
        if !valid_policies.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.internalTrafficPolicy", field_path),
                policy,
                &valid_policies,
            ));
        }
    }

    // Validate health check node port
    if let Some(port) = spec.health_check_node_port {
        if port != 0 {
            errors.extend(validate_port_number(
                port,
                &format!("{}.healthCheckNodePort", field_path),
            ));
        }
    }

    errors
}

fn validate_service_port(port: &ServicePort, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate port number
    errors.extend(validate_port_number(
        port.port,
        &format!("{}.port", field_path),
    ));

    // Validate node port if specified
    if let Some(node_port) = port.node_port {
        if node_port != 0 {
            // Node ports must be in range 30000-32767 by default
            if node_port < 1 || node_port > 65535 {
                errors.push(ValidationError::out_of_range(
                    format!("{}.nodePort", field_path),
                    1,
                    65535,
                    node_port as i64,
                ));
            }
        }
    }

    // Validate port name if specified
    if !port.name.is_empty() {
        errors.extend(validate_port_name(
            &port.name,
            &format!("{}.name", field_path),
        ));
    }

    // Validate protocol
    errors.extend(validate_protocol(
        &port.protocol,
        &format!("{}.protocol", field_path),
    ));

    errors
}

// =============================================================================
// ConfigMap Validation
// =============================================================================

/// Validates a ConfigMap.
pub fn validate_configmap(configmap: &ConfigMap) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&configmap.metadata, "metadata", true));

    // Validate total size (ConfigMaps are limited to 1MB)
    let total_size: usize = configmap.data.values().map(|v| v.len()).sum::<usize>()
        + configmap
            .binary_data
            .values()
            .map(|v| v.len())
            .sum::<usize>();

    if total_size > 1024 * 1024 {
        errors.push(ValidationError::too_long(
            "data + binaryData",
            1024 * 1024,
            total_size,
        ));
    }

    errors
}

// =============================================================================
// Secret Validation
// =============================================================================

/// Validates a Secret.
pub fn validate_secret(secret: &Secret) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&secret.metadata, "metadata", true));

    // Validate secret type
    if !secret.secret_type.is_empty() {
        let valid_types = [
            "Opaque",
            "kubernetes.io/service-account-token",
            "kubernetes.io/dockercfg",
            "kubernetes.io/dockerconfigjson",
            "kubernetes.io/basic-auth",
            "kubernetes.io/ssh-auth",
            "kubernetes.io/tls",
            "bootstrap.kubernetes.io/token",
        ];
        // Custom types are allowed, so we only warn about well-known types
        // being misspelled if they start with "kubernetes.io/"
        if secret.secret_type.starts_with("kubernetes.io/")
            && !valid_types.contains(&secret.secret_type.as_str())
        {
            // This is just informational, not an error
        }
    }

    // Validate total size (Secrets are limited to 1MB)
    let total_size: usize = secret.data.values().map(|v| v.len()).sum::<usize>()
        + secret
            .string_data
            .values()
            .map(|v| v.len())
            .sum::<usize>();

    if total_size > 1024 * 1024 {
        errors.push(ValidationError::too_long(
            "data + stringData",
            1024 * 1024,
            total_size,
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::core::v1::{Container, Pod, PodSpec};
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_pod_missing_name() {
        let pod = Pod {
            metadata: ObjectMeta::default(),
            spec: Some(PodSpec {
                containers: vec![Container::new("nginx", "nginx:latest")],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pod(&pod);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("metadata.name")));
    }

    #[test]
    fn test_validate_pod_no_containers() {
        let pod = Pod {
            metadata: ObjectMeta::named("test"),
            spec: Some(PodSpec::default()),
            ..Default::default()
        };

        let errors = validate_pod(&pod);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("containers")));
    }

    #[test]
    fn test_validate_valid_pod() {
        let pod = Pod {
            metadata: ObjectMeta::named("test"),
            spec: Some(PodSpec {
                containers: vec![Container::new("nginx", "nginx:latest")],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pod(&pod);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pod_duplicate_container_names() {
        let pod = Pod {
            metadata: ObjectMeta::named("test"),
            spec: Some(PodSpec {
                containers: vec![
                    Container::new("nginx", "nginx:latest"),
                    Container::new("nginx", "nginx:1.19"),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pod(&pod);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn test_validate_pod_invalid_restart_policy() {
        let pod = Pod {
            metadata: ObjectMeta::named("test"),
            spec: Some(PodSpec {
                containers: vec![Container::new("nginx", "nginx:latest")],
                restart_policy: "Invalid".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pod(&pod);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("restartPolicy")));
    }

    #[test]
    fn test_validate_namespace() {
        let namespace = Namespace {
            metadata: ObjectMeta::named("my-namespace"),
            ..Default::default()
        };

        let errors = validate_namespace(&namespace);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_namespace_invalid_name() {
        let namespace = Namespace {
            metadata: ObjectMeta::named("My.Invalid.Namespace"),
            ..Default::default()
        };

        let errors = validate_namespace(&namespace);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validate_service() {
        let service = Service {
            metadata: ObjectMeta::named("my-service"),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_service(&service);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_service_invalid_port() {
        let service = Service {
            metadata: ObjectMeta::named("my-service"),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 0,
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_service(&service);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("port")));
    }
}
