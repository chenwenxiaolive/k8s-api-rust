//! Autoscaling API validation
//!
//! This module provides validation for autoscaling API types including:
//! - HorizontalPodAutoscaler (v1 and v2)

use crate::common::{validate_dns_subdomain_name, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::autoscaling::v2::{
    ContainerResourceMetricSource, ExternalMetricSource, HPAScalingPolicy, HPAScalingRules,
    HorizontalPodAutoscaler, HorizontalPodAutoscalerBehavior, HorizontalPodAutoscalerSpec,
    MetricSpec, MetricTarget, ObjectMetricSource, PodsMetricSource, ResourceMetricSource,
};

/// Valid metric types
const VALID_METRIC_TYPES: &[&str] = &["Resource", "Pods", "Object", "External", "ContainerResource"];

/// Valid metric target types
const VALID_TARGET_TYPES: &[&str] = &["Utilization", "Value", "AverageValue"];

/// Valid scaling policy select types
const VALID_SELECT_POLICIES: &[&str] = &["Max", "Min", "Disabled"];

/// Valid scaling policy types
const VALID_SCALING_POLICY_TYPES: &[&str] = &["Pods", "Percent"];

/// Maximum stabilization window seconds
const MAX_STABILIZATION_WINDOW_SECONDS: i32 = 3600;

/// Maximum period seconds for scaling policy
const MAX_PERIOD_SECONDS: i32 = 1800;

// =============================================================================
// HorizontalPodAutoscaler Validation
// =============================================================================

/// Validates a HorizontalPodAutoscaler resource.
pub fn validate_hpa(hpa: &HorizontalPodAutoscaler) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&hpa.metadata, "metadata", true));

    if let Some(spec) = &hpa.spec {
        errors.extend(validate_hpa_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a HorizontalPodAutoscalerSpec.
pub fn validate_hpa_spec(spec: &HorizontalPodAutoscalerSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate scaleTargetRef
    errors.extend(validate_scale_target_ref(
        &spec.scale_target_ref,
        &format!("{}.scaleTargetRef", field),
    ));

    // Validate maxReplicas
    if spec.max_replicas < 1 {
        errors.push(ValidationError::out_of_range(
            format!("{}.maxReplicas", field),
            1,
            i32::MAX as i64,
            spec.max_replicas as i64,
        ));
    }

    // Validate minReplicas
    if let Some(min_replicas) = spec.min_replicas {
        if min_replicas < 0 {
            errors.push(ValidationError::out_of_range(
                format!("{}.minReplicas", field),
                0,
                i32::MAX as i64,
                min_replicas as i64,
            ));
        }

        // minReplicas should be <= maxReplicas
        if min_replicas > spec.max_replicas {
            errors.push(ValidationError::invalid(
                format!("{}.minReplicas", field),
                format!(
                    "minReplicas ({}) must be less than or equal to maxReplicas ({})",
                    min_replicas, spec.max_replicas
                ),
            ));
        }
    }

    // Validate metrics
    for (i, metric) in spec.metrics.iter().enumerate() {
        errors.extend(validate_metric_spec(
            metric,
            &format!("{}.metrics[{}]", field, i),
        ));
    }

    // Validate behavior
    if let Some(behavior) = &spec.behavior {
        errors.extend(validate_hpa_behavior(
            behavior,
            &format!("{}.behavior", field),
        ));
    }

    errors
}

/// Validates a CrossVersionObjectReference (scaleTargetRef).
fn validate_scale_target_ref(
    target: &k8s_api::autoscaling::v2::CrossVersionObjectReference,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Kind is required
    if target.kind.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.kind", field),
            "kind is required",
        ));
    }

    // Name is required
    if target.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "name is required",
        ));
    } else {
        errors.extend(validate_dns_subdomain_name(
            &target.name,
            &format!("{}.name", field),
        ));
    }

    errors
}

// =============================================================================
// MetricSpec Validation
// =============================================================================

/// Validates a MetricSpec.
pub fn validate_metric_spec(metric: &MetricSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate type
    if metric.type_.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.type", field),
            "type is required",
        ));
    } else if !VALID_METRIC_TYPES.contains(&metric.type_.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.type", field),
            &metric.type_,
            VALID_METRIC_TYPES,
        ));
    }

    // Validate that the appropriate source is present based on type
    match metric.type_.as_str() {
        "Resource" => {
            if let Some(resource) = &metric.resource {
                errors.extend(validate_resource_metric_source(
                    resource,
                    &format!("{}.resource", field),
                ));
            } else {
                errors.push(ValidationError::required(
                    format!("{}.resource", field),
                    "resource is required when type is Resource",
                ));
            }
        }
        "Pods" => {
            if let Some(pods) = &metric.pods {
                errors.extend(validate_pods_metric_source(
                    pods,
                    &format!("{}.pods", field),
                ));
            } else {
                errors.push(ValidationError::required(
                    format!("{}.pods", field),
                    "pods is required when type is Pods",
                ));
            }
        }
        "Object" => {
            if let Some(object) = &metric.object {
                errors.extend(validate_object_metric_source(
                    object,
                    &format!("{}.object", field),
                ));
            } else {
                errors.push(ValidationError::required(
                    format!("{}.object", field),
                    "object is required when type is Object",
                ));
            }
        }
        "External" => {
            if let Some(external) = &metric.external {
                errors.extend(validate_external_metric_source(
                    external,
                    &format!("{}.external", field),
                ));
            } else {
                errors.push(ValidationError::required(
                    format!("{}.external", field),
                    "external is required when type is External",
                ));
            }
        }
        "ContainerResource" => {
            if let Some(container_resource) = &metric.container_resource {
                errors.extend(validate_container_resource_metric_source(
                    container_resource,
                    &format!("{}.containerResource", field),
                ));
            } else {
                errors.push(ValidationError::required(
                    format!("{}.containerResource", field),
                    "containerResource is required when type is ContainerResource",
                ));
            }
        }
        _ => {}
    }

    errors
}

pub mod internal {
    use super::*;
    use k8s_api::autoscaling::internal as api;

    pub fn validate_hpa(hpa: &api::HorizontalPodAutoscaler) -> ValidationResult {
        crate::internal::validate_with(hpa, "horizontalPodAutoscaler", super::validate_hpa)
    }

    pub fn validate_hpa_spec(
        spec: &api::HorizontalPodAutoscalerSpec,
        field: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field, |external_spec| {
            super::validate_hpa_spec(external_spec, field)
        })
    }

    pub fn validate_metric_spec(metric: &api::MetricSpec, field: &str) -> ValidationResult {
        crate::internal::validate_with(metric, field, |external_metric| {
            super::validate_metric_spec(external_metric, field)
        })
    }
}

/// Validates a ResourceMetricSource.
fn validate_resource_metric_source(source: &ResourceMetricSource, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Name is required
    if source.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "resource name is required",
        ));
    }

    // Validate target
    errors.extend(validate_metric_target(
        &source.target,
        &format!("{}.target", field),
        true, // Resource metrics support Utilization
    ));

    errors
}

/// Validates a PodsMetricSource.
fn validate_pods_metric_source(source: &PodsMetricSource, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metric identifier
    errors.extend(validate_metric_identifier(
        &source.metric,
        &format!("{}.metric", field),
    ));

    // Validate target - Pods metrics don't support Utilization
    errors.extend(validate_metric_target(
        &source.target,
        &format!("{}.target", field),
        false,
    ));

    errors
}

/// Validates an ObjectMetricSource.
fn validate_object_metric_source(source: &ObjectMetricSource, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate describedObject
    errors.extend(validate_scale_target_ref(
        &source.described_object,
        &format!("{}.describedObject", field),
    ));

    // Validate metric identifier
    errors.extend(validate_metric_identifier(
        &source.metric,
        &format!("{}.metric", field),
    ));

    // Validate target - Object metrics don't support Utilization
    errors.extend(validate_metric_target(
        &source.target,
        &format!("{}.target", field),
        false,
    ));

    errors
}

/// Validates an ExternalMetricSource.
fn validate_external_metric_source(source: &ExternalMetricSource, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metric identifier
    errors.extend(validate_metric_identifier(
        &source.metric,
        &format!("{}.metric", field),
    ));

    // Validate target - External metrics don't support Utilization
    errors.extend(validate_metric_target(
        &source.target,
        &format!("{}.target", field),
        false,
    ));

    errors
}

/// Validates a ContainerResourceMetricSource.
fn validate_container_resource_metric_source(
    source: &ContainerResourceMetricSource,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Name is required
    if source.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "resource name is required",
        ));
    }

    // Container is required
    if source.container.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.container", field),
            "container name is required",
        ));
    }

    // Validate target - ContainerResource metrics support Utilization
    errors.extend(validate_metric_target(
        &source.target,
        &format!("{}.target", field),
        true,
    ));

    errors
}

/// Validates a MetricIdentifier.
fn validate_metric_identifier(
    metric: &k8s_api::autoscaling::v2::MetricIdentifier,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if metric.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "metric name is required",
        ));
    }

    errors
}

/// Validates a MetricTarget.
fn validate_metric_target(
    target: &MetricTarget,
    field: &str,
    allow_utilization: bool,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Type is required
    if target.type_.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.type", field),
            "target type is required",
        ));
    } else if !VALID_TARGET_TYPES.contains(&target.type_.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.type", field),
            &target.type_,
            VALID_TARGET_TYPES,
        ));
    }

    // Validate based on type
    match target.type_.as_str() {
        "Utilization" => {
            if !allow_utilization {
                errors.push(ValidationError::invalid(
                    format!("{}.type", field),
                    "Utilization type is only valid for Resource and ContainerResource metrics",
                ));
            }

            if target.average_utilization.is_none() {
                errors.push(ValidationError::required(
                    format!("{}.averageUtilization", field),
                    "averageUtilization is required when type is Utilization",
                ));
            } else if let Some(util) = target.average_utilization {
                if util < 1 || util > 100 {
                    errors.push(ValidationError::out_of_range(
                        format!("{}.averageUtilization", field),
                        1,
                        100,
                        util as i64,
                    ));
                }
            }
        }
        "Value" => {
            if target.value.is_none() {
                errors.push(ValidationError::required(
                    format!("{}.value", field),
                    "value is required when type is Value",
                ));
            }
        }
        "AverageValue" => {
            if target.average_value.is_none() {
                errors.push(ValidationError::required(
                    format!("{}.averageValue", field),
                    "averageValue is required when type is AverageValue",
                ));
            }
        }
        _ => {}
    }

    errors
}

// =============================================================================
// Behavior Validation
// =============================================================================

/// Validates HorizontalPodAutoscalerBehavior.
fn validate_hpa_behavior(
    behavior: &HorizontalPodAutoscalerBehavior,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if let Some(scale_up) = &behavior.scale_up {
        errors.extend(validate_hpa_scaling_rules(
            scale_up,
            &format!("{}.scaleUp", field),
        ));
    }

    if let Some(scale_down) = &behavior.scale_down {
        errors.extend(validate_hpa_scaling_rules(
            scale_down,
            &format!("{}.scaleDown", field),
        ));
    }

    errors
}

/// Validates HPAScalingRules.
fn validate_hpa_scaling_rules(rules: &HPAScalingRules, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate stabilizationWindowSeconds
    if let Some(window) = rules.stabilization_window_seconds {
        if window < 0 || window > MAX_STABILIZATION_WINDOW_SECONDS {
            errors.push(ValidationError::out_of_range(
                format!("{}.stabilizationWindowSeconds", field),
                0,
                MAX_STABILIZATION_WINDOW_SECONDS as i64,
                window as i64,
            ));
        }
    }

    // Validate selectPolicy
    if let Some(policy) = &rules.select_policy {
        if !VALID_SELECT_POLICIES.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.selectPolicy", field),
                policy,
                VALID_SELECT_POLICIES,
            ));
        }
    }

    // Validate policies
    for (i, policy) in rules.policies.iter().enumerate() {
        errors.extend(validate_hpa_scaling_policy(
            policy,
            &format!("{}.policies[{}]", field, i),
        ));
    }

    errors
}

/// Validates HPAScalingPolicy.
fn validate_hpa_scaling_policy(policy: &HPAScalingPolicy, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate type
    if policy.type_.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.type", field),
            "policy type is required",
        ));
    } else if !VALID_SCALING_POLICY_TYPES.contains(&policy.type_.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.type", field),
            &policy.type_,
            VALID_SCALING_POLICY_TYPES,
        ));
    }

    // Validate value
    if policy.value < 1 {
        errors.push(ValidationError::out_of_range(
            format!("{}.value", field),
            1,
            i32::MAX as i64,
            policy.value as i64,
        ));
    }

    // Validate periodSeconds
    if policy.period_seconds < 1 || policy.period_seconds > MAX_PERIOD_SECONDS {
        errors.push(ValidationError::out_of_range(
            format!("{}.periodSeconds", field),
            1,
            MAX_PERIOD_SECONDS as i64,
            policy.period_seconds as i64,
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::autoscaling::v2::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_hpa_valid() {
        let hpa = HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: Some(HorizontalPodAutoscalerSpec {
                scale_target_ref: CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "my-deployment".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(1),
                max_replicas: 10,
                metrics: vec![MetricSpec {
                    type_: "Resource".to_string(),
                    resource: Some(ResourceMetricSource {
                        name: "cpu".to_string(),
                        target: MetricTarget {
                            type_: "Utilization".to_string(),
                            average_utilization: Some(80),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_hpa(&hpa);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_hpa_missing_spec() {
        let hpa = HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: None,
            ..Default::default()
        };

        let errors = validate_hpa(&hpa);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_hpa_invalid_replicas() {
        let hpa = HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: Some(HorizontalPodAutoscalerSpec {
                scale_target_ref: CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "my-deployment".to_string(),
                    ..Default::default()
                },
                min_replicas: Some(10),
                max_replicas: 5, // Invalid: min > max
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_hpa(&hpa);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("minReplicas")));
    }

    #[test]
    fn test_validate_hpa_missing_scale_target() {
        let hpa = HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: Some(HorizontalPodAutoscalerSpec {
                scale_target_ref: CrossVersionObjectReference::default(),
                max_replicas: 10,
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_hpa(&hpa);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("scaleTargetRef.kind")));
        assert!(errors.iter().any(|e| e.field.contains("scaleTargetRef.name")));
    }

    #[test]
    fn test_validate_metric_spec_resource() {
        let metric = MetricSpec {
            type_: "Resource".to_string(),
            resource: Some(ResourceMetricSource {
                name: "cpu".to_string(),
                target: MetricTarget {
                    type_: "Utilization".to_string(),
                    average_utilization: Some(80),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_metric_spec_invalid_type() {
        let metric = MetricSpec {
            type_: "Invalid".to_string(),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("type")));
    }

    #[test]
    fn test_validate_metric_spec_missing_resource() {
        let metric = MetricSpec {
            type_: "Resource".to_string(),
            resource: None, // Missing
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("resource")));
    }

    #[test]
    fn test_validate_metric_spec_external() {
        let metric = MetricSpec {
            type_: "External".to_string(),
            external: Some(ExternalMetricSource {
                metric: MetricIdentifier {
                    name: "queue_length".to_string(),
                    ..Default::default()
                },
                target: MetricTarget {
                    type_: "AverageValue".to_string(),
                    average_value: Some("100".parse().unwrap()),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_metric_target_utilization_invalid_range() {
        let target = MetricTarget {
            type_: "Utilization".to_string(),
            average_utilization: Some(150), // Invalid: > 100
            ..Default::default()
        };

        let errors = validate_metric_target(&target, "target", true);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("averageUtilization")));
    }

    #[test]
    fn test_validate_metric_target_utilization_not_allowed() {
        let target = MetricTarget {
            type_: "Utilization".to_string(),
            average_utilization: Some(80),
            ..Default::default()
        };

        // Utilization not allowed for Pods metrics
        let errors = validate_metric_target(&target, "target", false);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("only valid for Resource")));
    }

    #[test]
    fn test_validate_hpa_behavior_valid() {
        let behavior = HorizontalPodAutoscalerBehavior {
            scale_up: Some(HPAScalingRules {
                stabilization_window_seconds: Some(60),
                select_policy: Some("Max".to_string()),
                policies: vec![HPAScalingPolicy {
                    type_: "Pods".to_string(),
                    value: 4,
                    period_seconds: 60,
                }],
                tolerance: None,
            }),
            scale_down: Some(HPAScalingRules {
                stabilization_window_seconds: Some(300),
                select_policy: Some("Min".to_string()),
                policies: vec![HPAScalingPolicy {
                    type_: "Percent".to_string(),
                    value: 10,
                    period_seconds: 60,
                }],
                tolerance: None,
            }),
        };

        let errors = validate_hpa_behavior(&behavior, "behavior");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_hpa_behavior_invalid_stabilization_window() {
        let behavior = HorizontalPodAutoscalerBehavior {
            scale_up: Some(HPAScalingRules {
                stabilization_window_seconds: Some(5000), // Invalid: > 3600
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_hpa_behavior(&behavior, "behavior");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("stabilizationWindowSeconds")));
    }

    #[test]
    fn test_validate_hpa_scaling_policy_invalid_type() {
        let policy = HPAScalingPolicy {
            type_: "Invalid".to_string(),
            value: 4,
            period_seconds: 60,
        };

        let errors = validate_hpa_scaling_policy(&policy, "policy");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("type")));
    }

    #[test]
    fn test_validate_hpa_scaling_policy_invalid_period() {
        let policy = HPAScalingPolicy {
            type_: "Pods".to_string(),
            value: 4,
            period_seconds: 2000, // Invalid: > 1800
        };

        let errors = validate_hpa_scaling_policy(&policy, "policy");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("periodSeconds")));
    }

    #[test]
    fn test_validate_container_resource_metric() {
        let metric = MetricSpec {
            type_: "ContainerResource".to_string(),
            container_resource: Some(ContainerResourceMetricSource {
                name: "cpu".to_string(),
                container: "app".to_string(),
                target: MetricTarget {
                    type_: "Utilization".to_string(),
                    average_utilization: Some(70),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_container_resource_missing_container() {
        let source = ContainerResourceMetricSource {
            name: "cpu".to_string(),
            container: "".to_string(), // Missing
            target: MetricTarget {
                type_: "Utilization".to_string(),
                average_utilization: Some(70),
                ..Default::default()
            },
        };

        let errors = validate_container_resource_metric_source(&source, "containerResource");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("container")));
    }

    #[test]
    fn test_validate_pods_metric() {
        let metric = MetricSpec {
            type_: "Pods".to_string(),
            pods: Some(PodsMetricSource {
                metric: MetricIdentifier {
                    name: "packets-per-second".to_string(),
                    ..Default::default()
                },
                target: MetricTarget {
                    type_: "AverageValue".to_string(),
                    average_value: Some("1k".parse().unwrap()),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_object_metric() {
        let metric = MetricSpec {
            type_: "Object".to_string(),
            object: Some(ObjectMetricSource {
                described_object: CrossVersionObjectReference {
                    kind: "Ingress".to_string(),
                    name: "main-route".to_string(),
                    api_version: "networking.k8s.io/v1".to_string(),
                },
                metric: MetricIdentifier {
                    name: "requests-per-second".to_string(),
                    ..Default::default()
                },
                target: MetricTarget {
                    type_: "Value".to_string(),
                    value: Some("10k".parse().unwrap()),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        let errors = validate_metric_spec(&metric, "metric");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }
}
