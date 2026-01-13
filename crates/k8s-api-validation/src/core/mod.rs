//! Core v1 validation

use crate::{ValidationError, ValidationResult};
use k8s_api::core::v1::{Container, Pod, PodSpec};

/// Validates a Pod.
pub fn validate_pod(pod: &Pod) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    if pod.metadata.name.is_empty() && pod.metadata.generate_name.is_empty() {
        errors.push(ValidationError::required(
            "metadata.name",
            "name or generateName is required",
        ));
    }

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

    // Validate each container
    for (i, container) in spec.containers.iter().enumerate() {
        errors.extend(validate_container(
            container,
            &format!("{}.containers[{}]", field_path, i),
        ));
    }

    // Validate init containers
    for (i, container) in spec.init_containers.iter().enumerate() {
        errors.extend(validate_container(
            container,
            &format!("{}.initContainers[{}]", field_path, i),
        ));
    }

    // Validate restart policy
    if !spec.restart_policy.is_empty() {
        let valid_policies = ["Always", "OnFailure", "Never"];
        if !valid_policies.contains(&spec.restart_policy.as_str()) {
            errors.push(ValidationError::invalid(
                format!("{}.restartPolicy", field_path),
                format!(
                    "must be one of {:?}, got: {}",
                    valid_policies, spec.restart_policy
                ),
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
    }

    // Image is required (though it can be omitted if pulling from a default)
    if container.image.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.image", field_path),
            "container image is required",
        ));
    }

    // Validate ports
    for (i, port) in container.ports.iter().enumerate() {
        if port.container_port < 1 || port.container_port > 65535 {
            errors.push(ValidationError::invalid(
                format!("{}.ports[{}].containerPort", field_path, i),
                "port must be between 1 and 65535",
            ));
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::core::v1::{Container, Pod, PodSpec};
    use k8s_apimachinery::ObjectMeta;

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
        assert!(errors.is_empty());
    }
}
