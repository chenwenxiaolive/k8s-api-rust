//! Apps API validation
//!
//! This module provides validation for apps/v1 API types.

use crate::common::{validate_dns_subdomain_name, validate_labels, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::apps::v1::{
    DaemonSet, DaemonSetSpec, Deployment, DeploymentSpec, ReplicaSet, ReplicaSetSpec, StatefulSet,
    StatefulSetSpec,
};

// =============================================================================
// Deployment Validation
// =============================================================================

/// Validates a Deployment.
pub fn validate_deployment(deployment: &Deployment) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&deployment.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &deployment.spec {
        errors.extend(validate_deployment_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a DeploymentSpec.
pub fn validate_deployment_spec(spec: &DeploymentSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Replicas must be non-negative
    if let Some(replicas) = spec.replicas {
        if replicas < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.replicas", field_path),
                "replicas must be non-negative",
            ));
        }
    }

    // Selector is required
    if let Some(ref selector) = spec.selector {
        if selector.match_labels.is_empty() && selector.match_expressions.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.selector", field_path),
                "selector must have matchLabels or matchExpressions",
            ));
        }

        // Validate selector labels
        errors.extend(validate_labels(
            &selector.match_labels,
            &format!("{}.selector.matchLabels", field_path),
        ));
    } else {
        errors.push(ValidationError::required(
            format!("{}.selector", field_path),
            "selector is required",
        ));
    }

    // Validate template
    errors.extend(validate_pod_template_spec(
        &spec.template,
        &format!("{}.template", field_path),
    ));

    // Validate strategy
    if let Some(strategy) = &spec.strategy {
        errors.extend(validate_deployment_strategy(
            strategy,
            &format!("{}.strategy", field_path),
        ));
    }

    // Validate revision history limit
    if let Some(limit) = spec.revision_history_limit {
        if limit < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.revisionHistoryLimit", field_path),
                "revisionHistoryLimit must be non-negative",
            ));
        }
    }

    // Validate progress deadline seconds
    if let Some(deadline) = spec.progress_deadline_seconds {
        if deadline < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.progressDeadlineSeconds", field_path),
                "progressDeadlineSeconds must be non-negative",
            ));
        }
    }

    errors
}

fn validate_deployment_strategy(
    strategy: &k8s_api::apps::v1::DeploymentStrategy,
    field_path: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate strategy type
    if !strategy.strategy_type.is_empty() {
        let valid_types = ["Recreate", "RollingUpdate"];
        if !valid_types.contains(&strategy.strategy_type.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.type", field_path),
                &strategy.strategy_type,
                &valid_types,
            ));
        }
    }

    errors
}

// =============================================================================
// StatefulSet Validation
// =============================================================================

/// Validates a StatefulSet.
pub fn validate_statefulset(statefulset: &StatefulSet) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(
        &statefulset.metadata,
        "metadata",
        true,
    ));

    // Validate spec
    if let Some(spec) = &statefulset.spec {
        errors.extend(validate_statefulset_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a StatefulSetSpec.
pub fn validate_statefulset_spec(spec: &StatefulSetSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Replicas must be non-negative
    if let Some(replicas) = spec.replicas {
        if replicas < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.replicas", field_path),
                "replicas must be non-negative",
            ));
        }
    }

    // Selector is required
    if let Some(ref selector) = spec.selector {
        if selector.match_labels.is_empty() && selector.match_expressions.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.selector", field_path),
                "selector must have matchLabels or matchExpressions",
            ));
        }

        // Validate selector labels
        errors.extend(validate_labels(
            &selector.match_labels,
            &format!("{}.selector.matchLabels", field_path),
        ));
    } else {
        errors.push(ValidationError::required(
            format!("{}.selector", field_path),
            "selector is required",
        ));
    }

    // ServiceName is required
    if spec.service_name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.serviceName", field_path),
            "serviceName is required",
        ));
    } else {
        errors.extend(validate_dns_subdomain_name(
            &spec.service_name,
            &format!("{}.serviceName", field_path),
        ));
    }

    // Validate template
    errors.extend(validate_pod_template_spec(
        &spec.template,
        &format!("{}.template", field_path),
    ));

    // Validate pod management policy
    if !spec.pod_management_policy.is_empty() {
        let valid_policies = ["OrderedReady", "Parallel"];
        if !valid_policies.contains(&spec.pod_management_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.podManagementPolicy", field_path),
                &spec.pod_management_policy,
                &valid_policies,
            ));
        }
    }

    errors
}

// =============================================================================
// DaemonSet Validation
// =============================================================================

/// Validates a DaemonSet.
pub fn validate_daemonset(daemonset: &DaemonSet) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&daemonset.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &daemonset.spec {
        errors.extend(validate_daemonset_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a DaemonSetSpec.
pub fn validate_daemonset_spec(spec: &DaemonSetSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Selector is required
    if let Some(ref selector) = spec.selector {
        if selector.match_labels.is_empty() && selector.match_expressions.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.selector", field_path),
                "selector must have matchLabels or matchExpressions",
            ));
        }

        // Validate selector labels
        errors.extend(validate_labels(
            &selector.match_labels,
            &format!("{}.selector.matchLabels", field_path),
        ));
    } else {
        errors.push(ValidationError::required(
            format!("{}.selector", field_path),
            "selector is required",
        ));
    }

    // Validate template
    errors.extend(validate_pod_template_spec(
        &spec.template,
        &format!("{}.template", field_path),
    ));

    // Validate min ready seconds
    if let Some(min_ready) = spec.min_ready_seconds {
        if min_ready < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.minReadySeconds", field_path),
                "minReadySeconds must be non-negative",
            ));
        }
    }

    // Validate revision history limit
    if let Some(limit) = spec.revision_history_limit {
        if limit < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.revisionHistoryLimit", field_path),
                "revisionHistoryLimit must be non-negative",
            ));
        }
    }

    errors
}

// =============================================================================
// ReplicaSet Validation
// =============================================================================

/// Validates a ReplicaSet.
pub fn validate_replicaset(replicaset: &ReplicaSet) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&replicaset.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &replicaset.spec {
        errors.extend(validate_replicaset_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a ReplicaSetSpec.
pub fn validate_replicaset_spec(spec: &ReplicaSetSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Replicas must be non-negative
    if let Some(replicas) = spec.replicas {
        if replicas < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.replicas", field_path),
                "replicas must be non-negative",
            ));
        }
    }

    // Selector is required
    if let Some(ref selector) = spec.selector {
        if selector.match_labels.is_empty() && selector.match_expressions.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.selector", field_path),
                "selector must have matchLabels or matchExpressions",
            ));
        }

        // Validate selector labels
        errors.extend(validate_labels(
            &selector.match_labels,
            &format!("{}.selector.matchLabels", field_path),
        ));
    } else {
        errors.push(ValidationError::required(
            format!("{}.selector", field_path),
            "selector is required",
        ));
    }

    // Validate template
    if let Some(template) = &spec.template {
        errors.extend(validate_pod_template_spec(
            template,
            &format!("{}.template", field_path),
        ));
    }

    // Validate min ready seconds
    if let Some(min_ready) = spec.min_ready_seconds {
        if min_ready < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.minReadySeconds", field_path),
                "minReadySeconds must be non-negative",
            ));
        }
    }

    errors
}

pub mod internal {
    use super::*;
    use k8s_api::apps::internal as api;

    pub fn validate_deployment(deployment: &api::Deployment) -> ValidationResult {
        crate::internal::validate_with(deployment, "deployment", super::validate_deployment)
    }

    pub fn validate_deployment_spec(spec: &api::DeploymentSpec, field_path: &str) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_deployment_spec(external_spec, field_path)
        })
    }

    pub fn validate_statefulset(statefulset: &api::StatefulSet) -> ValidationResult {
        crate::internal::validate_with(statefulset, "statefulSet", super::validate_statefulset)
    }

    pub fn validate_statefulset_spec(
        spec: &api::StatefulSetSpec,
        field_path: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_statefulset_spec(external_spec, field_path)
        })
    }

    pub fn validate_daemonset(daemonset: &api::DaemonSet) -> ValidationResult {
        crate::internal::validate_with(daemonset, "daemonSet", super::validate_daemonset)
    }

    pub fn validate_daemonset_spec(
        spec: &api::DaemonSetSpec,
        field_path: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_daemonset_spec(external_spec, field_path)
        })
    }

    pub fn validate_replicaset(replicaset: &api::ReplicaSet) -> ValidationResult {
        crate::internal::validate_with(replicaset, "replicaSet", super::validate_replicaset)
    }

    pub fn validate_replicaset_spec(
        spec: &api::ReplicaSetSpec,
        field_path: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_replicaset_spec(external_spec, field_path)
        })
    }
}

// =============================================================================
// PodTemplateSpec Validation (shared)
// =============================================================================

fn validate_pod_template_spec(
    template: &k8s_api::core::v1::PodTemplateSpec,
    field_path: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate template metadata
    errors.extend(validate_object_meta(
        &template.metadata,
        &format!("{}.metadata", field_path),
        false,
    ));

    // Validate template labels
    errors.extend(validate_labels(
        &template.metadata.labels,
        &format!("{}.metadata.labels", field_path),
    ));

    // Validate pod spec
    if let Some(spec) = &template.spec {
        errors.extend(crate::core::validate_pod_spec(
            spec,
            &format!("{}.spec", field_path),
        ));
    } else {
        errors.push(ValidationError::required(
            format!("{}.spec", field_path),
            "pod template spec is required",
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::apps::v1::{Deployment, DeploymentSpec};
    use k8s_api::core::v1::{Container, PodSpec, PodTemplateSpec};
    use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_deployment_missing_spec() {
        let deployment = Deployment {
            metadata: ObjectMeta::named("test"),
            spec: None,
            ..Default::default()
        };

        let errors = validate_deployment(&deployment);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_deployment_missing_selector() {
        let deployment = Deployment {
            metadata: ObjectMeta::named("test"),
            spec: Some(DeploymentSpec {
                selector: Some(LabelSelector::default()),
                template: PodTemplateSpec {
                    metadata: ObjectMeta::default(),
                    spec: Some(PodSpec {
                        containers: vec![Container::new("nginx", "nginx:latest")],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_deployment(&deployment);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("selector")));
    }

    #[test]
    fn test_validate_valid_deployment() {
        let mut labels = BTreeMap::new();
        labels.insert("app".to_string(), "nginx".to_string());

        let deployment = Deployment {
            metadata: ObjectMeta::named("test"),
            spec: Some(DeploymentSpec {
                replicas: Some(3),
                selector: Some(LabelSelector {
                    match_labels: labels.clone(),
                    ..Default::default()
                }),
                template: PodTemplateSpec {
                    metadata: ObjectMeta {
                        labels: labels,
                        ..Default::default()
                    },
                    spec: Some(PodSpec {
                        containers: vec![Container::new("nginx", "nginx:latest")],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_deployment(&deployment);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_deployment_negative_replicas() {
        let mut labels = BTreeMap::new();
        labels.insert("app".to_string(), "nginx".to_string());

        let deployment = Deployment {
            metadata: ObjectMeta::named("test"),
            spec: Some(DeploymentSpec {
                replicas: Some(-1),
                selector: Some(LabelSelector {
                    match_labels: labels.clone(),
                    ..Default::default()
                }),
                template: PodTemplateSpec {
                    metadata: ObjectMeta {
                        labels: labels,
                        ..Default::default()
                    },
                    spec: Some(PodSpec {
                        containers: vec![Container::new("nginx", "nginx:latest")],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_deployment(&deployment);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("replicas")));
    }
}
