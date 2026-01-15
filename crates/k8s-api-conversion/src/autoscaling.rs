//! Autoscaling API conversions
//!
//! This module provides conversions between autoscaling API versions.
//!
//! Note: v1 only supports CPU-based autoscaling, while v2 supports multiple metric types.
//! Converting from v2 to v1 may result in loss of metric information.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// HorizontalPodAutoscaler: v1 <-> v2
// =============================================================================

impl Convertible<k8s_api::autoscaling::v2::HorizontalPodAutoscaler>
    for k8s_api::autoscaling::v1::HorizontalPodAutoscaler
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::autoscaling::v2::HorizontalPodAutoscaler, ConversionError> {
        Ok(k8s_api::autoscaling::v2::HorizontalPodAutoscaler {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "autoscaling/v2",
                "HorizontalPodAutoscaler",
            ),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_hpa_spec_to_v2(s)),
            status: self.status.as_ref().map(|s| convert_hpa_status_to_v2(s)),
        })
    }

    fn convert_from(
        other: &k8s_api::autoscaling::v2::HorizontalPodAutoscaler,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "autoscaling/v1",
                "HorizontalPodAutoscaler",
            ),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_hpa_spec_from_v2(s)),
            status: other.status.as_ref().map(|s| convert_hpa_status_from_v2(s)),
        })
    }
}

fn convert_hpa_spec_to_v2(
    spec: &k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec,
) -> k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    let mut metrics = Vec::new();

    // Convert targetCPUUtilizationPercentage to a Resource metric
    if let Some(cpu_target) = spec.target_cpu_utilization_percentage {
        metrics.push(k8s_api::autoscaling::v2::MetricSpec {
            type_: "Resource".to_string(),
            resource: Some(k8s_api::autoscaling::v2::ResourceMetricSource {
                name: "cpu".to_string(),
                target: k8s_api::autoscaling::v2::MetricTarget {
                    type_: "Utilization".to_string(),
                    average_utilization: Some(cpu_target),
                    ..Default::default()
                },
            }),
            ..Default::default()
        });
    }

    k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
        scale_target_ref: k8s_api::autoscaling::v2::CrossVersionObjectReference {
            kind: spec.scale_target_ref.kind.clone(),
            name: spec.scale_target_ref.name.clone(),
            api_version: spec.scale_target_ref.api_version.clone(),
        },
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics,
        behavior: None, // v1 doesn't have behavior
    }
}

fn convert_hpa_spec_from_v2(
    spec: &k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec,
) -> k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec {
    // Try to extract CPU utilization from v2 metrics
    let target_cpu_utilization_percentage = spec.metrics.iter().find_map(|m| {
        if m.type_ == "Resource" {
            if let Some(ref resource) = m.resource {
                if resource.name == "cpu" && resource.target.type_ == "Utilization" {
                    return resource.target.average_utilization;
                }
            }
        }
        None
    });

    k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec {
        scale_target_ref: k8s_api::autoscaling::v1::CrossVersionObjectReference {
            kind: spec.scale_target_ref.kind.clone(),
            name: spec.scale_target_ref.name.clone(),
            api_version: spec.scale_target_ref.api_version.clone(),
        },
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        target_cpu_utilization_percentage,
    }
}

fn convert_hpa_status_to_v2(
    status: &k8s_api::autoscaling::v1::HorizontalPodAutoscalerStatus,
) -> k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    let mut current_metrics = Vec::new();

    // Convert currentCPUUtilizationPercentage to a Resource metric status
    if let Some(cpu_current) = status.current_cpu_utilization_percentage {
        current_metrics.push(k8s_api::autoscaling::v2::MetricStatus {
            type_: "Resource".to_string(),
            resource: Some(k8s_api::autoscaling::v2::ResourceMetricStatus {
                name: "cpu".to_string(),
                current: k8s_api::autoscaling::v2::MetricValueStatus {
                    average_utilization: Some(cpu_current),
                    ..Default::default()
                },
            }),
            ..Default::default()
        });
    }

    k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time.clone(),
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics,
        conditions: Vec::new(), // v1 doesn't have conditions
    }
}

fn convert_hpa_status_from_v2(
    status: &k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus,
) -> k8s_api::autoscaling::v1::HorizontalPodAutoscalerStatus {
    // Try to extract CPU utilization from v2 current metrics
    let current_cpu_utilization_percentage = status.current_metrics.iter().find_map(|m| {
        if m.type_ == "Resource" {
            if let Some(ref resource) = m.resource {
                if resource.name == "cpu" {
                    return resource.current.average_utilization;
                }
            }
        }
        None
    });

    k8s_api::autoscaling::v1::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time.clone(),
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_cpu_utilization_percentage,
    }
}

// =============================================================================
// HorizontalPodAutoscaler: v2beta1 <-> v2
// =============================================================================

impl Convertible<k8s_api::autoscaling::v2::HorizontalPodAutoscaler>
    for k8s_api::autoscaling::v2beta1::HorizontalPodAutoscaler
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::autoscaling::v2::HorizontalPodAutoscaler, ConversionError> {
        Ok(k8s_api::autoscaling::v2::HorizontalPodAutoscaler {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "autoscaling/v2",
                "HorizontalPodAutoscaler",
            ),
            metadata: self.metadata.clone(),
            spec: self
                .spec
                .as_ref()
                .map(|s| convert_hpa_spec_v2beta1_to_v2(s)),
            status: self
                .status
                .as_ref()
                .map(|s| convert_hpa_status_v2beta1_to_v2(s)),
        })
    }

    fn convert_from(
        other: &k8s_api::autoscaling::v2::HorizontalPodAutoscaler,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "autoscaling/v2beta1",
                "HorizontalPodAutoscaler",
            ),
            metadata: other.metadata.clone(),
            spec: other
                .spec
                .as_ref()
                .map(|s| convert_hpa_spec_v2beta1_from_v2(s)),
            status: other
                .status
                .as_ref()
                .map(|s| convert_hpa_status_v2beta1_from_v2(s)),
        })
    }
}

fn convert_hpa_spec_v2beta1_to_v2(
    spec: &k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec,
) -> k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_v2beta1_to_v2(
            &spec.scale_target_ref,
        ),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics: spec
            .metrics
            .iter()
            .map(convert_metric_spec_v2beta1_to_v2)
            .collect(),
        behavior: None,
    }
}

fn convert_hpa_spec_v2beta1_from_v2(
    spec: &k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec,
) -> k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec {
    k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_v2_to_v2beta1(
            &spec.scale_target_ref,
        ),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics: spec
            .metrics
            .iter()
            .map(convert_metric_spec_v2beta1_from_v2)
            .collect(),
    }
}

fn convert_hpa_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus,
) -> k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time.clone(),
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics: status
            .current_metrics
            .iter()
            .map(convert_metric_status_v2beta1_to_v2)
            .collect(),
        conditions: status
            .conditions
            .iter()
            .map(convert_hpa_condition_v2beta1_to_v2)
            .collect(),
    }
}

fn convert_hpa_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::HorizontalPodAutoscalerStatus,
) -> k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus {
    k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time.clone(),
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics: status
            .current_metrics
            .iter()
            .map(convert_metric_status_v2beta1_from_v2)
            .collect(),
        conditions: status
            .conditions
            .iter()
            .map(convert_hpa_condition_v2beta1_from_v2)
            .collect(),
    }
}

fn convert_metric_spec_v2beta1_to_v2(
    spec: &k8s_api::autoscaling::v2beta1::MetricSpec,
) -> k8s_api::autoscaling::v2::MetricSpec {
    k8s_api::autoscaling::v2::MetricSpec {
        type_: spec.type_.clone(),
        object: spec
            .object
            .as_ref()
            .map(convert_object_metric_source_v2beta1_to_v2),
        pods: spec
            .pods
            .as_ref()
            .map(convert_pods_metric_source_v2beta1_to_v2),
        resource: spec
            .resource
            .as_ref()
            .map(convert_resource_metric_source_v2beta1_to_v2),
        container_resource: spec
            .container_resource
            .as_ref()
            .map(convert_container_resource_metric_source_v2beta1_to_v2),
        external: spec
            .external
            .as_ref()
            .map(convert_external_metric_source_v2beta1_to_v2),
    }
}

fn convert_metric_spec_v2beta1_from_v2(
    spec: &k8s_api::autoscaling::v2::MetricSpec,
) -> k8s_api::autoscaling::v2beta1::MetricSpec {
    k8s_api::autoscaling::v2beta1::MetricSpec {
        type_: spec.type_.clone(),
        object: spec
            .object
            .as_ref()
            .map(convert_object_metric_source_v2beta1_from_v2),
        pods: spec
            .pods
            .as_ref()
            .map(convert_pods_metric_source_v2beta1_from_v2),
        resource: spec
            .resource
            .as_ref()
            .map(convert_resource_metric_source_v2beta1_from_v2),
        container_resource: spec
            .container_resource
            .as_ref()
            .map(convert_container_resource_metric_source_v2beta1_from_v2),
        external: spec
            .external
            .as_ref()
            .map(convert_external_metric_source_v2beta1_from_v2),
    }
}

fn convert_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::MetricStatus,
) -> k8s_api::autoscaling::v2::MetricStatus {
    k8s_api::autoscaling::v2::MetricStatus {
        type_: status.type_.clone(),
        object: status
            .object
            .as_ref()
            .map(convert_object_metric_status_v2beta1_to_v2),
        pods: status
            .pods
            .as_ref()
            .map(convert_pods_metric_status_v2beta1_to_v2),
        resource: status
            .resource
            .as_ref()
            .map(convert_resource_metric_status_v2beta1_to_v2),
        container_resource: status
            .container_resource
            .as_ref()
            .map(convert_container_resource_metric_status_v2beta1_to_v2),
        external: status
            .external
            .as_ref()
            .map(convert_external_metric_status_v2beta1_to_v2),
    }
}

fn convert_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::MetricStatus,
) -> k8s_api::autoscaling::v2beta1::MetricStatus {
    k8s_api::autoscaling::v2beta1::MetricStatus {
        type_: status.type_.clone(),
        object: status
            .object
            .as_ref()
            .map(convert_object_metric_status_v2beta1_from_v2),
        pods: status
            .pods
            .as_ref()
            .map(convert_pods_metric_status_v2beta1_from_v2),
        resource: status
            .resource
            .as_ref()
            .map(convert_resource_metric_status_v2beta1_from_v2),
        container_resource: status
            .container_resource
            .as_ref()
            .map(convert_container_resource_metric_status_v2beta1_from_v2),
        external: status
            .external
            .as_ref()
            .map(convert_external_metric_status_v2beta1_from_v2),
    }
}

fn convert_cross_version_object_reference_v2beta1_to_v2(
    reference: &k8s_api::autoscaling::v2beta1::CrossVersionObjectReference,
) -> k8s_api::autoscaling::v2::CrossVersionObjectReference {
    k8s_api::autoscaling::v2::CrossVersionObjectReference {
        kind: reference.kind.clone(),
        name: reference.name.clone(),
        api_version: reference.api_version.clone(),
    }
}

fn convert_cross_version_object_reference_v2_to_v2beta1(
    reference: &k8s_api::autoscaling::v2::CrossVersionObjectReference,
) -> k8s_api::autoscaling::v2beta1::CrossVersionObjectReference {
    k8s_api::autoscaling::v2beta1::CrossVersionObjectReference {
        kind: reference.kind.clone(),
        name: reference.name.clone(),
        api_version: reference.api_version.clone(),
    }
}

fn convert_hpa_condition_v2beta1_to_v2(
    condition: &k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition,
) -> k8s_api::autoscaling::v2::HorizontalPodAutoscalerCondition {
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerCondition {
        condition_type: condition.condition_type.clone(),
        status: condition.status.clone(),
        last_transition_time: condition.last_transition_time.clone(),
        reason: condition.reason.clone(),
        message: condition.message.clone(),
    }
}

fn convert_hpa_condition_v2beta1_from_v2(
    condition: &k8s_api::autoscaling::v2::HorizontalPodAutoscalerCondition,
) -> k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition {
    k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition {
        condition_type: condition.condition_type.clone(),
        status: condition.status.clone(),
        last_transition_time: condition.last_transition_time.clone(),
        reason: condition.reason.clone(),
        message: condition.message.clone(),
    }
}

fn convert_object_metric_source_v2beta1_to_v2(
    source: &k8s_api::autoscaling::v2beta1::ObjectMetricSource,
) -> k8s_api::autoscaling::v2::ObjectMetricSource {
    k8s_api::autoscaling::v2::ObjectMetricSource {
        described_object: convert_cross_version_object_reference_v2beta1_to_v2(&source.target),
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: source.metric_name.clone(),
            selector: source.selector.clone(),
        },
        target: k8s_api::autoscaling::v2::MetricTarget {
            type_: if source.average_value.is_some() {
                "AverageValue".to_string()
            } else {
                "Value".to_string()
            },
            value: Some(source.target_value.clone()),
            average_value: source.average_value.clone(),
            ..Default::default()
        },
    }
}

fn convert_object_metric_source_v2beta1_from_v2(
    source: &k8s_api::autoscaling::v2::ObjectMetricSource,
) -> k8s_api::autoscaling::v2beta1::ObjectMetricSource {
    k8s_api::autoscaling::v2beta1::ObjectMetricSource {
        target: convert_cross_version_object_reference_v2_to_v2beta1(&source.described_object),
        metric_name: source.metric.name.clone(),
        target_value: select_metric_target_value(&source.target),
        selector: source.metric.selector.clone(),
        average_value: source.target.average_value.clone(),
    }
}

fn convert_object_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::ObjectMetricStatus,
) -> k8s_api::autoscaling::v2::ObjectMetricStatus {
    k8s_api::autoscaling::v2::ObjectMetricStatus {
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: status.metric_name.clone(),
            selector: status.selector.clone(),
        },
        current: k8s_api::autoscaling::v2::MetricValueStatus {
            value: Some(status.current_value.clone()),
            average_value: status.average_value.clone(),
            ..Default::default()
        },
        described_object: convert_cross_version_object_reference_v2beta1_to_v2(&status.target),
    }
}

fn convert_object_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::ObjectMetricStatus,
) -> k8s_api::autoscaling::v2beta1::ObjectMetricStatus {
    k8s_api::autoscaling::v2beta1::ObjectMetricStatus {
        target: convert_cross_version_object_reference_v2_to_v2beta1(&status.described_object),
        metric_name: status.metric.name.clone(),
        current_value: select_metric_status_value(&status.current),
        selector: status.metric.selector.clone(),
        average_value: status.current.average_value.clone(),
    }
}

fn convert_pods_metric_source_v2beta1_to_v2(
    source: &k8s_api::autoscaling::v2beta1::PodsMetricSource,
) -> k8s_api::autoscaling::v2::PodsMetricSource {
    k8s_api::autoscaling::v2::PodsMetricSource {
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: source.metric_name.clone(),
            selector: source.selector.clone(),
        },
        target: k8s_api::autoscaling::v2::MetricTarget {
            type_: "AverageValue".to_string(),
            average_value: Some(source.target_average_value.clone()),
            ..Default::default()
        },
    }
}

fn convert_pods_metric_source_v2beta1_from_v2(
    source: &k8s_api::autoscaling::v2::PodsMetricSource,
) -> k8s_api::autoscaling::v2beta1::PodsMetricSource {
    k8s_api::autoscaling::v2beta1::PodsMetricSource {
        metric_name: source.metric.name.clone(),
        target_average_value: select_metric_target_value(&source.target),
        selector: source.metric.selector.clone(),
    }
}

fn convert_pods_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::PodsMetricStatus,
) -> k8s_api::autoscaling::v2::PodsMetricStatus {
    k8s_api::autoscaling::v2::PodsMetricStatus {
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: status.metric_name.clone(),
            selector: status.selector.clone(),
        },
        current: k8s_api::autoscaling::v2::MetricValueStatus {
            average_value: Some(status.current_average_value.clone()),
            ..Default::default()
        },
    }
}

fn convert_pods_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::PodsMetricStatus,
) -> k8s_api::autoscaling::v2beta1::PodsMetricStatus {
    k8s_api::autoscaling::v2beta1::PodsMetricStatus {
        metric_name: status.metric.name.clone(),
        current_average_value: select_metric_status_value(&status.current),
        selector: status.metric.selector.clone(),
    }
}

fn convert_resource_metric_source_v2beta1_to_v2(
    source: &k8s_api::autoscaling::v2beta1::ResourceMetricSource,
) -> k8s_api::autoscaling::v2::ResourceMetricSource {
    let mut target = k8s_api::autoscaling::v2::MetricTarget {
        type_: String::new(),
        ..Default::default()
    };

    if let Some(utilization) = source.target_average_utilization {
        target.type_ = "Utilization".to_string();
        target.average_utilization = Some(utilization);
    }

    if let Some(average_value) = source.target_average_value.clone() {
        if target.type_.is_empty() {
            target.type_ = "AverageValue".to_string();
        }
        target.average_value = Some(average_value);
    }

    if target.type_.is_empty() {
        target.type_ = "AverageValue".to_string();
    }

    k8s_api::autoscaling::v2::ResourceMetricSource {
        name: source.name.clone(),
        target,
    }
}

fn convert_resource_metric_source_v2beta1_from_v2(
    source: &k8s_api::autoscaling::v2::ResourceMetricSource,
) -> k8s_api::autoscaling::v2beta1::ResourceMetricSource {
    k8s_api::autoscaling::v2beta1::ResourceMetricSource {
        name: source.name.clone(),
        target_average_utilization: source.target.average_utilization,
        target_average_value: select_metric_target_average_value(&source.target),
    }
}

fn convert_resource_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::ResourceMetricStatus,
) -> k8s_api::autoscaling::v2::ResourceMetricStatus {
    k8s_api::autoscaling::v2::ResourceMetricStatus {
        name: status.name.clone(),
        current: k8s_api::autoscaling::v2::MetricValueStatus {
            average_utilization: status.current_average_utilization,
            average_value: status.current_average_value.clone(),
            ..Default::default()
        },
    }
}

fn convert_resource_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::ResourceMetricStatus,
) -> k8s_api::autoscaling::v2beta1::ResourceMetricStatus {
    k8s_api::autoscaling::v2beta1::ResourceMetricStatus {
        name: status.name.clone(),
        current_average_utilization: status.current.average_utilization,
        current_average_value: select_metric_status_average_value(&status.current),
    }
}

fn convert_container_resource_metric_source_v2beta1_to_v2(
    source: &k8s_api::autoscaling::v2beta1::ContainerResourceMetricSource,
) -> k8s_api::autoscaling::v2::ContainerResourceMetricSource {
    let mut target = k8s_api::autoscaling::v2::MetricTarget {
        type_: String::new(),
        ..Default::default()
    };

    if let Some(utilization) = source.target_average_utilization {
        target.type_ = "Utilization".to_string();
        target.average_utilization = Some(utilization);
    }

    if let Some(average_value) = source.target_average_value.clone() {
        if target.type_.is_empty() {
            target.type_ = "AverageValue".to_string();
        }
        target.average_value = Some(average_value);
    }

    if target.type_.is_empty() {
        target.type_ = "AverageValue".to_string();
    }

    k8s_api::autoscaling::v2::ContainerResourceMetricSource {
        name: source.name.clone(),
        container: source.container.clone(),
        target,
    }
}

fn convert_container_resource_metric_source_v2beta1_from_v2(
    source: &k8s_api::autoscaling::v2::ContainerResourceMetricSource,
) -> k8s_api::autoscaling::v2beta1::ContainerResourceMetricSource {
    k8s_api::autoscaling::v2beta1::ContainerResourceMetricSource {
        name: source.name.clone(),
        container: source.container.clone(),
        target_average_utilization: source.target.average_utilization,
        target_average_value: select_metric_target_average_value(&source.target),
    }
}

fn convert_container_resource_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::ContainerResourceMetricStatus,
) -> k8s_api::autoscaling::v2::ContainerResourceMetricStatus {
    k8s_api::autoscaling::v2::ContainerResourceMetricStatus {
        name: status.name.clone(),
        container: status.container.clone(),
        current: k8s_api::autoscaling::v2::MetricValueStatus {
            average_utilization: status.current_average_utilization,
            average_value: status.current_average_value.clone(),
            ..Default::default()
        },
    }
}

fn convert_container_resource_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::ContainerResourceMetricStatus,
) -> k8s_api::autoscaling::v2beta1::ContainerResourceMetricStatus {
    k8s_api::autoscaling::v2beta1::ContainerResourceMetricStatus {
        name: status.name.clone(),
        container: status.container.clone(),
        current_average_utilization: status.current.average_utilization,
        current_average_value: select_metric_status_average_value(&status.current),
    }
}

fn convert_external_metric_source_v2beta1_to_v2(
    source: &k8s_api::autoscaling::v2beta1::ExternalMetricSource,
) -> k8s_api::autoscaling::v2::ExternalMetricSource {
    let mut target = k8s_api::autoscaling::v2::MetricTarget {
        type_: String::new(),
        ..Default::default()
    };

    if let Some(value) = source.target_value.clone() {
        target.type_ = "Value".to_string();
        target.value = Some(value);
    }

    if let Some(average_value) = source.target_average_value.clone() {
        if target.type_.is_empty() {
            target.type_ = "AverageValue".to_string();
        }
        target.average_value = Some(average_value);
    }

    if target.type_.is_empty() {
        target.type_ = "Value".to_string();
    }

    k8s_api::autoscaling::v2::ExternalMetricSource {
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: source.metric_name.clone(),
            selector: source.metric_selector.clone(),
        },
        target,
    }
}

fn convert_external_metric_source_v2beta1_from_v2(
    source: &k8s_api::autoscaling::v2::ExternalMetricSource,
) -> k8s_api::autoscaling::v2beta1::ExternalMetricSource {
    k8s_api::autoscaling::v2beta1::ExternalMetricSource {
        metric_name: source.metric.name.clone(),
        metric_selector: source.metric.selector.clone(),
        target_value: source.target.value.clone(),
        target_average_value: source.target.average_value.clone(),
    }
}

fn convert_external_metric_status_v2beta1_to_v2(
    status: &k8s_api::autoscaling::v2beta1::ExternalMetricStatus,
) -> k8s_api::autoscaling::v2::ExternalMetricStatus {
    k8s_api::autoscaling::v2::ExternalMetricStatus {
        metric: k8s_api::autoscaling::v2::MetricIdentifier {
            name: status.metric_name.clone(),
            selector: status.metric_selector.clone(),
        },
        current: k8s_api::autoscaling::v2::MetricValueStatus {
            value: Some(status.current_value.clone()),
            average_value: status.current_average_value.clone(),
            ..Default::default()
        },
    }
}

fn convert_external_metric_status_v2beta1_from_v2(
    status: &k8s_api::autoscaling::v2::ExternalMetricStatus,
) -> k8s_api::autoscaling::v2beta1::ExternalMetricStatus {
    k8s_api::autoscaling::v2beta1::ExternalMetricStatus {
        metric_name: status.metric.name.clone(),
        metric_selector: status.metric.selector.clone(),
        current_value: select_metric_status_value(&status.current),
        current_average_value: status.current.average_value.clone(),
    }
}

fn select_metric_target_value(
    target: &k8s_api::autoscaling::v2::MetricTarget,
) -> k8s_api_core::Quantity {
    target
        .value
        .clone()
        .or_else(|| target.average_value.clone())
        .unwrap_or_else(|| k8s_api_core::Quantity::new("0"))
}

fn select_metric_target_average_value(
    target: &k8s_api::autoscaling::v2::MetricTarget,
) -> Option<k8s_api_core::Quantity> {
    target.average_value.clone().or_else(|| target.value.clone())
}

fn select_metric_status_value(
    status: &k8s_api::autoscaling::v2::MetricValueStatus,
) -> k8s_api_core::Quantity {
    status
        .value
        .clone()
        .or_else(|| status.average_value.clone())
        .unwrap_or_else(|| k8s_api_core::Quantity::new("0"))
}

fn select_metric_status_average_value(
    status: &k8s_api::autoscaling::v2::MetricValueStatus,
) -> Option<k8s_api_core::Quantity> {
    status.average_value.clone().or_else(|| status.value.clone())
}

// =============================================================================
// HorizontalPodAutoscaler: v2beta2 <-> v2
// =============================================================================

impl Convertible<k8s_api::autoscaling::v2::HorizontalPodAutoscaler>
    for k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::autoscaling::v2::HorizontalPodAutoscaler, ConversionError> {
        let mut converted: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "autoscaling/v2",
            "HorizontalPodAutoscaler",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::autoscaling::v2::HorizontalPodAutoscaler,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "autoscaling/v2beta2",
            "HorizontalPodAutoscaler",
        );
        Ok(converted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_hpa_v1_to_v2_conversion() {
        let v1_hpa = k8s_api::autoscaling::v1::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: Some(k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v1::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "my-deployment".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(2),
                max_replicas: 10,
                target_cpu_utilization_percentage: Some(80),
            }),
            status: Some(k8s_api::autoscaling::v1::HorizontalPodAutoscalerStatus {
                current_replicas: 3,
                desired_replicas: 5,
                current_cpu_utilization_percentage: Some(75),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v2
        let v2_hpa: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            v1_hpa.convert_to().unwrap();

        assert_eq!(v2_hpa.metadata.name, "test-hpa");
        let spec = v2_hpa.spec.as_ref().unwrap();
        assert_eq!(spec.max_replicas, 10);
        assert_eq!(spec.min_replicas, Some(2));
        assert_eq!(spec.scale_target_ref.kind, "Deployment");

        // Check metric conversion
        assert_eq!(spec.metrics.len(), 1);
        let metric = &spec.metrics[0];
        assert_eq!(metric.type_, "Resource");
        let resource = metric.resource.as_ref().unwrap();
        assert_eq!(resource.name, "cpu");
        assert_eq!(resource.target.type_, "Utilization");
        assert_eq!(resource.target.average_utilization, Some(80));

        // Check status conversion
        let status = v2_hpa.status.as_ref().unwrap();
        assert_eq!(status.current_replicas, 3);
        assert_eq!(status.desired_replicas, 5);
        assert_eq!(status.current_metrics.len(), 1);
    }

    #[test]
    fn test_hpa_v2_to_v1_conversion() {
        let v2_hpa = k8s_api::autoscaling::v2::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("test-hpa"),
            spec: Some(k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v2::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "my-deployment".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(1),
                max_replicas: 5,
                metrics: vec![
                    k8s_api::autoscaling::v2::MetricSpec {
                        type_: "Resource".to_string(),
                        resource: Some(k8s_api::autoscaling::v2::ResourceMetricSource {
                            name: "cpu".to_string(),
                            target: k8s_api::autoscaling::v2::MetricTarget {
                                type_: "Utilization".to_string(),
                                average_utilization: Some(70),
                                ..Default::default()
                            },
                        }),
                        ..Default::default()
                    },
                    // This memory metric will be lost in v1 conversion
                    k8s_api::autoscaling::v2::MetricSpec {
                        type_: "Resource".to_string(),
                        resource: Some(k8s_api::autoscaling::v2::ResourceMetricSource {
                            name: "memory".to_string(),
                            target: k8s_api::autoscaling::v2::MetricTarget {
                                type_: "Utilization".to_string(),
                                average_utilization: Some(80),
                                ..Default::default()
                            },
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_hpa: k8s_api::autoscaling::v1::HorizontalPodAutoscaler =
            k8s_api::autoscaling::v1::HorizontalPodAutoscaler::convert_from(&v2_hpa).unwrap();

        assert_eq!(v1_hpa.metadata.name, "test-hpa");
        let spec = v1_hpa.spec.as_ref().unwrap();
        assert_eq!(spec.max_replicas, 5);
        assert_eq!(spec.min_replicas, Some(1));
        assert_eq!(spec.target_cpu_utilization_percentage, Some(70));
    }

    #[test]
    fn test_hpa_conversion_roundtrip() {
        let v1_hpa = k8s_api::autoscaling::v1::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("roundtrip-hpa"),
            spec: Some(k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v1::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "test-deployment".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(3),
                max_replicas: 15,
                target_cpu_utilization_percentage: Some(60),
            }),
            ..Default::default()
        };

        // v1 -> v2 -> v1
        let v2_hpa: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            v1_hpa.convert_to().unwrap();
        let converted_back: k8s_api::autoscaling::v1::HorizontalPodAutoscaler =
            k8s_api::autoscaling::v1::HorizontalPodAutoscaler::convert_from(&v2_hpa).unwrap();

        assert_eq!(converted_back.metadata.name, "roundtrip-hpa");
        let spec = converted_back.spec.as_ref().unwrap();
        assert_eq!(spec.max_replicas, 15);
        assert_eq!(spec.min_replicas, Some(3));
        assert_eq!(spec.target_cpu_utilization_percentage, Some(60));
    }

    #[test]
    fn test_hpa_v1_without_cpu_target() {
        // v1 HPA without CPU target should convert to v2 with empty metrics
        let v1_hpa = k8s_api::autoscaling::v1::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("no-cpu-hpa"),
            spec: Some(k8s_api::autoscaling::v1::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v1::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "test".to_string(),
                    ..Default::default()
                },
                min_replicas: Some(1),
                max_replicas: 10,
                target_cpu_utilization_percentage: None,
            }),
            ..Default::default()
        };

        let v2_hpa: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            v1_hpa.convert_to().unwrap();

        let spec = v2_hpa.spec.as_ref().unwrap();
        assert!(spec.metrics.is_empty());
    }

    #[test]
    fn test_hpa_v2_with_external_metric() {
        // v2 HPA with only external metric should convert to v1 without CPU target
        let v2_hpa = k8s_api::autoscaling::v2::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("external-hpa"),
            spec: Some(k8s_api::autoscaling::v2::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v2::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "test".to_string(),
                    ..Default::default()
                },
                min_replicas: Some(1),
                max_replicas: 10,
                metrics: vec![k8s_api::autoscaling::v2::MetricSpec {
                    type_: "External".to_string(),
                    external: Some(k8s_api::autoscaling::v2::ExternalMetricSource {
                        metric: k8s_api::autoscaling::v2::MetricIdentifier {
                            name: "queue_length".to_string(),
                            ..Default::default()
                        },
                        target: k8s_api::autoscaling::v2::MetricTarget {
                            type_: "AverageValue".to_string(),
                            average_value: Some(k8s_api_core::Quantity::new("100")),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let v1_hpa: k8s_api::autoscaling::v1::HorizontalPodAutoscaler =
            k8s_api::autoscaling::v1::HorizontalPodAutoscaler::convert_from(&v2_hpa).unwrap();

        let spec = v1_hpa.spec.as_ref().unwrap();
        // External metric is lost in v1 conversion
        assert_eq!(spec.target_cpu_utilization_percentage, None);
    }

    #[test]
    fn test_hpa_v2beta1_to_v2_conversion() {
        let v2beta1_hpa = k8s_api::autoscaling::v2beta1::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("beta1-hpa"),
            spec: Some(k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v2beta1::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "backend".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(1),
                max_replicas: 4,
                metrics: vec![
                    k8s_api::autoscaling::v2beta1::MetricSpec {
                        type_: "Object".to_string(),
                        object: Some(k8s_api::autoscaling::v2beta1::ObjectMetricSource {
                            target: k8s_api::autoscaling::v2beta1::CrossVersionObjectReference {
                                kind: "Service".to_string(),
                                name: "api".to_string(),
                                api_version: "v1".to_string(),
                            },
                            metric_name: "requests".to_string(),
                            target_value: k8s_api_core::Quantity::new("10"),
                            selector: None,
                            average_value: Some(k8s_api_core::Quantity::new("5")),
                        }),
                        ..Default::default()
                    },
                    k8s_api::autoscaling::v2beta1::MetricSpec {
                        type_: "Resource".to_string(),
                        resource: Some(k8s_api::autoscaling::v2beta1::ResourceMetricSource {
                            name: "cpu".to_string(),
                            target_average_utilization: Some(75),
                            target_average_value: None,
                        }),
                        ..Default::default()
                    },
                ],
            }),
            ..Default::default()
        };

        let v2_hpa: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            v2beta1_hpa.convert_to().unwrap();
        let spec = v2_hpa.spec.as_ref().unwrap();
        assert_eq!(spec.scale_target_ref.kind, "Deployment");
        assert_eq!(spec.metrics.len(), 2);

        let object_metric = spec.metrics.iter().find(|m| m.type_ == "Object").unwrap();
        let object = object_metric.object.as_ref().unwrap();
        assert_eq!(object.metric.name, "requests");
        assert_eq!(object.target.type_, "AverageValue");
        assert_eq!(
            object.target.value,
            Some(k8s_api_core::Quantity::new("10"))
        );
        assert_eq!(
            object.target.average_value,
            Some(k8s_api_core::Quantity::new("5"))
        );

        let resource_metric = spec
            .metrics
            .iter()
            .find(|m| m.type_ == "Resource")
            .unwrap();
        let resource = resource_metric.resource.as_ref().unwrap();
        assert_eq!(resource.name, "cpu");
        assert_eq!(resource.target.type_, "Utilization");
        assert_eq!(resource.target.average_utilization, Some(75));
    }

    #[test]
    fn test_hpa_v2beta2_roundtrip() {
        let v2beta2_hpa = k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler {
            metadata: ObjectMeta::named("beta2-hpa"),
            spec: Some(k8s_api::autoscaling::v2beta2::HorizontalPodAutoscalerSpec {
                scale_target_ref: k8s_api::autoscaling::v2beta2::CrossVersionObjectReference {
                    kind: "Deployment".to_string(),
                    name: "backend".to_string(),
                    api_version: "apps/v1".to_string(),
                },
                min_replicas: Some(1),
                max_replicas: 3,
                behavior: Some(k8s_api::autoscaling::v2beta2::HorizontalPodAutoscalerBehavior {
                    scale_up: Some(k8s_api::autoscaling::v2beta2::HPAScalingRules {
                        stabilization_window_seconds: Some(30),
                        select_policy: Some("Max".to_string()),
                        policies: vec![k8s_api::autoscaling::v2beta2::HPAScalingPolicy {
                            type_: "Pods".to_string(),
                            value: 4,
                            period_seconds: 60,
                        }],
                    }),
                    scale_down: None,
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let v2_hpa: k8s_api::autoscaling::v2::HorizontalPodAutoscaler =
            v2beta2_hpa.convert_to().unwrap();
        assert_eq!(v2_hpa.type_meta.api_version, "autoscaling/v2");

        let converted_back: k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler =
            k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler::convert_from(&v2_hpa)
                .unwrap();
        assert_eq!(converted_back.metadata.name, "beta2-hpa");
        assert_eq!(
            converted_back.type_meta.api_version,
            "autoscaling/v2beta2"
        );
    }
}
