//! Autoscaling API conversions
//!
//! This module provides conversions between autoscaling API versions.
//!
//! Note: v1 only supports CPU-based autoscaling, while v2 supports multiple metric types.
//! Converting from v2 to v1 may result in loss of metric information.

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
                            average_value: Some("100".to_string()),
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
}
