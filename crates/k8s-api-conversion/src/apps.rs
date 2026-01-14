//! Apps API conversions
//!
//! This module provides conversions between apps API versions.

use crate::{ConversionError, Convertible};

// =============================================================================
// Deployment: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::Deployment> for k8s_api::apps::v1beta1::Deployment {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::Deployment, ConversionError> {
        Ok(k8s_api::apps::v1::Deployment {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "Deployment"),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_deployment_spec_to_v1(s)).transpose()?,
            status: self.status.as_ref().map(|s| convert_deployment_status_to_v1(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::Deployment) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta1", "Deployment"),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_deployment_spec_from_v1(s)).transpose()?,
            status: other.status.as_ref().map(|s| convert_deployment_status_from_v1(s)),
        })
    }
}

fn convert_deployment_spec_to_v1(
    spec: &k8s_api::apps::v1beta1::DeploymentSpec,
) -> Result<k8s_api::apps::v1::DeploymentSpec, ConversionError> {
    Ok(k8s_api::apps::v1::DeploymentSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        strategy: spec.strategy.as_ref().map(|s| k8s_api::apps::v1::DeploymentStrategy {
            strategy_type: s.strategy_type.clone(),
            rolling_update: s.rolling_update.as_ref().map(|ru| {
                k8s_api::apps::v1::RollingUpdateDeployment {
                    max_unavailable: ru.max_unavailable.clone(),
                    max_surge: ru.max_surge.clone(),
                }
            }),
        }),
        min_ready_seconds: spec.min_ready_seconds,
        revision_history_limit: spec.revision_history_limit,
        paused: spec.paused,
        progress_deadline_seconds: spec.progress_deadline_seconds,
    })
}

fn convert_deployment_spec_from_v1(
    spec: &k8s_api::apps::v1::DeploymentSpec,
) -> Result<k8s_api::apps::v1beta1::DeploymentSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta1::DeploymentSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        strategy: spec.strategy.as_ref().map(|s| k8s_api::apps::v1beta1::DeploymentStrategy {
            strategy_type: s.strategy_type.clone(),
            rolling_update: s.rolling_update.as_ref().map(|ru| {
                k8s_api::apps::v1beta1::RollingUpdateDeployment {
                    max_unavailable: ru.max_unavailable.clone(),
                    max_surge: ru.max_surge.clone(),
                }
            }),
        }),
        min_ready_seconds: spec.min_ready_seconds,
        revision_history_limit: spec.revision_history_limit,
        paused: spec.paused,
        progress_deadline_seconds: spec.progress_deadline_seconds,
        rollback_to: None, // v1 doesn't have rollback_to
    })
}

fn convert_deployment_status_to_v1(
    status: &k8s_api::apps::v1beta1::DeploymentStatus,
) -> k8s_api::apps::v1::DeploymentStatus {
    k8s_api::apps::v1::DeploymentStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        updated_replicas: status.updated_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        unavailable_replicas: status.unavailable_replicas,
        conditions: status.conditions.iter().map(|c| {
            k8s_api::apps::v1::DeploymentCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_update_time: c.last_update_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
        collision_count: status.collision_count,
    }
}

fn convert_deployment_status_from_v1(
    status: &k8s_api::apps::v1::DeploymentStatus,
) -> k8s_api::apps::v1beta1::DeploymentStatus {
    k8s_api::apps::v1beta1::DeploymentStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        updated_replicas: status.updated_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        unavailable_replicas: status.unavailable_replicas,
        conditions: status.conditions.iter().map(|c| {
            k8s_api::apps::v1beta1::DeploymentCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_update_time: c.last_update_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
        collision_count: status.collision_count,
    }
}

// =============================================================================
// StatefulSet: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::StatefulSet> for k8s_api::apps::v1beta1::StatefulSet {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::StatefulSet, ConversionError> {
        Ok(k8s_api::apps::v1::StatefulSet {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "StatefulSet"),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_statefulset_spec_to_v1(s)).transpose()?,
            status: self.status.as_ref().map(|s| convert_statefulset_status_to_v1(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::StatefulSet) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta1", "StatefulSet"),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_statefulset_spec_from_v1(s)).transpose()?,
            status: other.status.as_ref().map(|s| convert_statefulset_status_from_v1(s)),
        })
    }
}

fn convert_statefulset_spec_to_v1(
    spec: &k8s_api::apps::v1beta1::StatefulSetSpec,
) -> Result<k8s_api::apps::v1::StatefulSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1::StatefulSetSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        volume_claim_templates: spec.volume_claim_templates.clone(),
        service_name: spec.service_name.clone(),
        pod_management_policy: spec.pod_management_policy.clone(),
        update_strategy: spec.update_strategy.as_ref().map(|s| {
            k8s_api::apps::v1::StatefulSetUpdateStrategy {
                strategy_type: s.strategy_type.clone(),
                rolling_update: s.rolling_update.as_ref().map(|ru| {
                    k8s_api::apps::v1::RollingUpdateStatefulSetStrategy {
                        partition: ru.partition,
                        max_unavailable: None,
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: None,
        persistent_volume_claim_retention_policy: None,
        ordinals: None,
    })
}

fn convert_statefulset_spec_from_v1(
    spec: &k8s_api::apps::v1::StatefulSetSpec,
) -> Result<k8s_api::apps::v1beta1::StatefulSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta1::StatefulSetSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        volume_claim_templates: spec.volume_claim_templates.clone(),
        service_name: spec.service_name.clone(),
        pod_management_policy: spec.pod_management_policy.clone(),
        update_strategy: spec.update_strategy.as_ref().map(|s| {
            k8s_api::apps::v1beta1::StatefulSetUpdateStrategy {
                strategy_type: s.strategy_type.clone(),
                rolling_update: s.rolling_update.as_ref().map(|ru| {
                    k8s_api::apps::v1beta1::RollingUpdateStatefulSetStrategy {
                        partition: ru.partition,
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
    })
}

fn convert_statefulset_status_to_v1(
    status: &k8s_api::apps::v1beta1::StatefulSetStatus,
) -> k8s_api::apps::v1::StatefulSetStatus {
    k8s_api::apps::v1::StatefulSetStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        ready_replicas: status.ready_replicas,
        current_replicas: status.current_replicas,
        updated_replicas: status.updated_replicas,
        current_revision: status.current_revision.clone(),
        update_revision: status.update_revision.clone(),
        collision_count: status.collision_count,
        conditions: status.conditions.iter().map(|c| {
            k8s_api::apps::v1::StatefulSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
        available_replicas: None,
    }
}

fn convert_statefulset_status_from_v1(
    status: &k8s_api::apps::v1::StatefulSetStatus,
) -> k8s_api::apps::v1beta1::StatefulSetStatus {
    k8s_api::apps::v1beta1::StatefulSetStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        ready_replicas: status.ready_replicas,
        current_replicas: status.current_replicas,
        updated_replicas: status.updated_replicas,
        current_revision: status.current_revision.clone(),
        update_revision: status.update_revision.clone(),
        collision_count: status.collision_count,
        conditions: status.conditions.iter().map(|c| {
            k8s_api::apps::v1beta1::StatefulSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_conversion_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;
        use std::collections::BTreeMap;

        let mut labels = BTreeMap::new();
        labels.insert("app".to_string(), "test".to_string());

        let v1beta1_deployment = k8s_api::apps::v1beta1::Deployment {
            metadata: ObjectMeta::named("test"),
            spec: Some(k8s_api::apps::v1beta1::DeploymentSpec {
                replicas: Some(3),
                selector: Some(k8s_apimachinery::apis::meta::v1::LabelSelector {
                    match_labels: labels.clone(),
                    ..Default::default()
                }),
                template: k8s_api::core::v1::PodTemplateSpec {
                    metadata: ObjectMeta {
                        labels: labels.clone(),
                        ..Default::default()
                    },
                    spec: Some(k8s_api::core::v1::PodSpec {
                        containers: vec![k8s_api::core::v1::Container::new("nginx", "nginx:latest")],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_deployment: k8s_api::apps::v1::Deployment = v1beta1_deployment.convert_to().unwrap();
        assert_eq!(v1_deployment.metadata.name, "test");
        assert_eq!(v1_deployment.spec.as_ref().unwrap().replicas, Some(3));

        // Convert back to v1beta1
        let converted_back: k8s_api::apps::v1beta1::Deployment =
            k8s_api::apps::v1beta1::Deployment::convert_from(&v1_deployment).unwrap();
        assert_eq!(converted_back.metadata.name, "test");
        assert_eq!(converted_back.spec.as_ref().unwrap().replicas, Some(3));
    }
}
