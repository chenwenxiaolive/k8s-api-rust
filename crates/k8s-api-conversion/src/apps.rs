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
                        max_unavailable: ru.max_unavailable.clone(),
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: None,
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .as_ref()
            .map(convert_pvc_retention_to_v1),
        ordinals: spec.ordinals.as_ref().map(convert_ordinals_to_v1),
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
                        max_unavailable: ru.max_unavailable.clone(),
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .as_ref()
            .map(convert_pvc_retention_from_v1),
        ordinals: spec.ordinals.as_ref().map(convert_ordinals_from_v1),
    })
}

fn convert_pvc_retention_to_v1(
    policy: &k8s_api::apps::v1beta1::StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: policy.when_deleted.clone(),
        when_scaled: policy.when_scaled.clone(),
    }
}

fn convert_pvc_retention_from_v1(
    policy: &k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> k8s_api::apps::v1beta1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    k8s_api::apps::v1beta1::StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: policy.when_deleted.clone(),
        when_scaled: policy.when_scaled.clone(),
    }
}

fn convert_ordinals_to_v1(
    ordinals: &k8s_api::apps::v1beta1::StatefulSetOrdinals,
) -> k8s_api::apps::v1::StatefulSetOrdinals {
    k8s_api::apps::v1::StatefulSetOrdinals {
        start: ordinals.start,
    }
}

fn convert_ordinals_from_v1(
    ordinals: &k8s_api::apps::v1::StatefulSetOrdinals,
) -> k8s_api::apps::v1beta1::StatefulSetOrdinals {
    k8s_api::apps::v1beta1::StatefulSetOrdinals {
        start: ordinals.start,
    }
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

// =============================================================================
// v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::Deployment> for k8s_api::apps::v1beta2::Deployment {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::Deployment, ConversionError> {
        Ok(k8s_api::apps::v1::Deployment {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "Deployment"),
            metadata: self.metadata.clone(),
            spec: self
                .spec
                .as_ref()
                .map(|s| convert_deployment_spec_to_v1_from_beta2(s))
                .transpose()?,
            status: self
                .status
                .as_ref()
                .map(|s| convert_deployment_status_to_v1_from_beta2(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::Deployment) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta2", "Deployment"),
            metadata: other.metadata.clone(),
            spec: other
                .spec
                .as_ref()
                .map(|s| convert_deployment_spec_from_v1_to_beta2(s))
                .transpose()?,
            status: other
                .status
                .as_ref()
                .map(|s| convert_deployment_status_from_v1_to_beta2(s)),
        })
    }
}

fn convert_deployment_spec_to_v1_from_beta2(
    spec: &k8s_api::apps::v1beta2::DeploymentSpec,
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

fn convert_deployment_spec_from_v1_to_beta2(
    spec: &k8s_api::apps::v1::DeploymentSpec,
) -> Result<k8s_api::apps::v1beta2::DeploymentSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta2::DeploymentSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        strategy: spec.strategy.as_ref().map(|s| k8s_api::apps::v1beta2::DeploymentStrategy {
            strategy_type: s.strategy_type.clone(),
            rolling_update: s.rolling_update.as_ref().map(|ru| {
                k8s_api::apps::v1beta2::RollingUpdateDeployment {
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

fn convert_deployment_status_to_v1_from_beta2(
    status: &k8s_api::apps::v1beta2::DeploymentStatus,
) -> k8s_api::apps::v1::DeploymentStatus {
    k8s_api::apps::v1::DeploymentStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        updated_replicas: status.updated_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        unavailable_replicas: status.unavailable_replicas,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1::DeploymentCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_update_time: c.last_update_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
        collision_count: status.collision_count,
    }
}

fn convert_deployment_status_from_v1_to_beta2(
    status: &k8s_api::apps::v1::DeploymentStatus,
) -> k8s_api::apps::v1beta2::DeploymentStatus {
    k8s_api::apps::v1beta2::DeploymentStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        updated_replicas: status.updated_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        unavailable_replicas: status.unavailable_replicas,
        terminating_replicas: None,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1beta2::DeploymentCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_update_time: c.last_update_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
        collision_count: status.collision_count,
    }
}

impl Convertible<k8s_api::apps::v1::StatefulSet> for k8s_api::apps::v1beta2::StatefulSet {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::StatefulSet, ConversionError> {
        Ok(k8s_api::apps::v1::StatefulSet {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "StatefulSet"),
            metadata: self.metadata.clone(),
            spec: self
                .spec
                .as_ref()
                .map(|s| convert_statefulset_spec_to_v1_from_beta2(s))
                .transpose()?,
            status: self
                .status
                .as_ref()
                .map(|s| convert_statefulset_status_to_v1_from_beta2(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::StatefulSet) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta2", "StatefulSet"),
            metadata: other.metadata.clone(),
            spec: other
                .spec
                .as_ref()
                .map(|s| convert_statefulset_spec_from_v1_to_beta2(s))
                .transpose()?,
            status: other
                .status
                .as_ref()
                .map(|s| convert_statefulset_status_from_v1_to_beta2(s)),
        })
    }
}

fn convert_statefulset_spec_to_v1_from_beta2(
    spec: &k8s_api::apps::v1beta2::StatefulSetSpec,
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
                        max_unavailable: ru.max_unavailable.clone(),
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: spec.min_ready_seconds,
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .as_ref()
            .map(convert_pvc_retention_to_v1_from_beta2),
        ordinals: spec.ordinals.as_ref().map(convert_ordinals_to_v1_from_beta2),
    })
}

fn convert_statefulset_spec_from_v1_to_beta2(
    spec: &k8s_api::apps::v1::StatefulSetSpec,
) -> Result<k8s_api::apps::v1beta2::StatefulSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta2::StatefulSetSpec {
        replicas: spec.replicas,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        volume_claim_templates: spec.volume_claim_templates.clone(),
        service_name: spec.service_name.clone(),
        pod_management_policy: spec.pod_management_policy.clone(),
        update_strategy: spec.update_strategy.as_ref().map(|s| {
            k8s_api::apps::v1beta2::StatefulSetUpdateStrategy {
                strategy_type: s.strategy_type.clone(),
                rolling_update: s.rolling_update.as_ref().map(|ru| {
                    k8s_api::apps::v1beta2::RollingUpdateStatefulSetStrategy {
                        partition: ru.partition,
                        max_unavailable: ru.max_unavailable.clone(),
                    }
                }),
            }
        }),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: spec.min_ready_seconds,
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .as_ref()
            .map(convert_pvc_retention_from_v1_to_beta2),
        ordinals: spec.ordinals.as_ref().map(convert_ordinals_from_v1_to_beta2),
    })
}

fn convert_pvc_retention_to_v1_from_beta2(
    policy: &k8s_api::apps::v1beta2::StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: policy.when_deleted.clone(),
        when_scaled: policy.when_scaled.clone(),
    }
}

fn convert_pvc_retention_from_v1_to_beta2(
    policy: &k8s_api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> k8s_api::apps::v1beta2::StatefulSetPersistentVolumeClaimRetentionPolicy {
    k8s_api::apps::v1beta2::StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: policy.when_deleted.clone(),
        when_scaled: policy.when_scaled.clone(),
    }
}

fn convert_ordinals_to_v1_from_beta2(
    ordinals: &k8s_api::apps::v1beta2::StatefulSetOrdinals,
) -> k8s_api::apps::v1::StatefulSetOrdinals {
    k8s_api::apps::v1::StatefulSetOrdinals {
        start: ordinals.start,
    }
}

fn convert_ordinals_from_v1_to_beta2(
    ordinals: &k8s_api::apps::v1::StatefulSetOrdinals,
) -> k8s_api::apps::v1beta2::StatefulSetOrdinals {
    k8s_api::apps::v1beta2::StatefulSetOrdinals {
        start: ordinals.start,
    }
}

fn convert_statefulset_status_to_v1_from_beta2(
    status: &k8s_api::apps::v1beta2::StatefulSetStatus,
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
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1::StatefulSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
        available_replicas: status.available_replicas,
    }
}

fn convert_statefulset_status_from_v1_to_beta2(
    status: &k8s_api::apps::v1::StatefulSetStatus,
) -> k8s_api::apps::v1beta2::StatefulSetStatus {
    k8s_api::apps::v1beta2::StatefulSetStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        ready_replicas: status.ready_replicas,
        current_replicas: status.current_replicas,
        updated_replicas: status.updated_replicas,
        current_revision: status.current_revision.clone(),
        update_revision: status.update_revision.clone(),
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1beta2::StatefulSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
        available_replicas: status.available_replicas,
    }
}

impl Convertible<k8s_api::apps::v1::DaemonSet> for k8s_api::apps::v1beta2::DaemonSet {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::DaemonSet, ConversionError> {
        Ok(k8s_api::apps::v1::DaemonSet {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "DaemonSet"),
            metadata: self.metadata.clone(),
            spec: self
                .spec
                .as_ref()
                .map(|s| convert_daemonset_spec_to_v1_from_beta2(s))
                .transpose()?,
            status: self
                .status
                .as_ref()
                .map(|s| convert_daemonset_status_to_v1_from_beta2(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::DaemonSet) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta2", "DaemonSet"),
            metadata: other.metadata.clone(),
            spec: other
                .spec
                .as_ref()
                .map(|s| convert_daemonset_spec_from_v1_to_beta2(s))
                .transpose()?,
            status: other
                .status
                .as_ref()
                .map(|s| convert_daemonset_status_from_v1_to_beta2(s)),
        })
    }
}

fn convert_daemonset_spec_to_v1_from_beta2(
    spec: &k8s_api::apps::v1beta2::DaemonSetSpec,
) -> Result<k8s_api::apps::v1::DaemonSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1::DaemonSetSpec {
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        update_strategy: spec.update_strategy.as_ref().map(|s| k8s_api::apps::v1::DaemonSetUpdateStrategy {
            strategy_type: s.strategy_type.clone(),
            rolling_update: s.rolling_update.as_ref().map(|ru| {
                k8s_api::apps::v1::RollingUpdateDaemonSet {
                    max_unavailable: ru.max_unavailable.clone(),
                    max_surge: ru.max_surge.clone(),
                }
            }),
        }),
        min_ready_seconds: spec.min_ready_seconds,
        revision_history_limit: spec.revision_history_limit,
    })
}

fn convert_daemonset_spec_from_v1_to_beta2(
    spec: &k8s_api::apps::v1::DaemonSetSpec,
) -> Result<k8s_api::apps::v1beta2::DaemonSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta2::DaemonSetSpec {
        selector: spec.selector.clone(),
        template: spec.template.clone(),
        update_strategy: spec.update_strategy.as_ref().map(|s| k8s_api::apps::v1beta2::DaemonSetUpdateStrategy {
            strategy_type: s.strategy_type.clone(),
            rolling_update: s.rolling_update.as_ref().map(|ru| {
                k8s_api::apps::v1beta2::RollingUpdateDaemonSet {
                    max_unavailable: ru.max_unavailable.clone(),
                    max_surge: ru.max_surge.clone(),
                }
            }),
        }),
        min_ready_seconds: spec.min_ready_seconds,
        revision_history_limit: spec.revision_history_limit,
    })
}

fn convert_daemonset_status_to_v1_from_beta2(
    status: &k8s_api::apps::v1beta2::DaemonSetStatus,
) -> k8s_api::apps::v1::DaemonSetStatus {
    k8s_api::apps::v1::DaemonSetStatus {
        current_number_scheduled: status.current_number_scheduled,
        number_misscheduled: status.number_misscheduled,
        desired_number_scheduled: status.desired_number_scheduled,
        number_ready: status.number_ready,
        observed_generation: status.observed_generation,
        updated_number_scheduled: status.updated_number_scheduled,
        number_available: status.number_available,
        number_unavailable: status.number_unavailable,
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1::DaemonSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
    }
}

fn convert_daemonset_status_from_v1_to_beta2(
    status: &k8s_api::apps::v1::DaemonSetStatus,
) -> k8s_api::apps::v1beta2::DaemonSetStatus {
    k8s_api::apps::v1beta2::DaemonSetStatus {
        current_number_scheduled: status.current_number_scheduled,
        number_misscheduled: status.number_misscheduled,
        desired_number_scheduled: status.desired_number_scheduled,
        number_ready: status.number_ready,
        observed_generation: status.observed_generation,
        updated_number_scheduled: status.updated_number_scheduled,
        number_available: status.number_available,
        number_unavailable: status.number_unavailable,
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1beta2::DaemonSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
    }
}

impl Convertible<k8s_api::apps::v1::ReplicaSet> for k8s_api::apps::v1beta2::ReplicaSet {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ReplicaSet, ConversionError> {
        Ok(k8s_api::apps::v1::ReplicaSet {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "ReplicaSet"),
            metadata: self.metadata.clone(),
            spec: self
                .spec
                .as_ref()
                .map(|s| convert_replicaset_spec_to_v1_from_beta2(s))
                .transpose()?,
            status: self
                .status
                .as_ref()
                .map(|s| convert_replicaset_status_to_v1_from_beta2(s)),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::ReplicaSet) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta2", "ReplicaSet"),
            metadata: other.metadata.clone(),
            spec: other
                .spec
                .as_ref()
                .map(|s| convert_replicaset_spec_from_v1_to_beta2(s))
                .transpose()?,
            status: other
                .status
                .as_ref()
                .map(|s| convert_replicaset_status_from_v1_to_beta2(s)),
        })
    }
}

fn convert_replicaset_spec_to_v1_from_beta2(
    spec: &k8s_api::apps::v1beta2::ReplicaSetSpec,
) -> Result<k8s_api::apps::v1::ReplicaSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1::ReplicaSetSpec {
        replicas: spec.replicas,
        min_ready_seconds: spec.min_ready_seconds,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
    })
}

fn convert_replicaset_spec_from_v1_to_beta2(
    spec: &k8s_api::apps::v1::ReplicaSetSpec,
) -> Result<k8s_api::apps::v1beta2::ReplicaSetSpec, ConversionError> {
    Ok(k8s_api::apps::v1beta2::ReplicaSetSpec {
        replicas: spec.replicas,
        min_ready_seconds: spec.min_ready_seconds,
        selector: spec.selector.clone(),
        template: spec.template.clone(),
    })
}

fn convert_replicaset_status_to_v1_from_beta2(
    status: &k8s_api::apps::v1beta2::ReplicaSetStatus,
) -> k8s_api::apps::v1::ReplicaSetStatus {
    k8s_api::apps::v1::ReplicaSetStatus {
        replicas: status.replicas,
        fully_labeled_replicas: status.fully_labeled_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        observed_generation: status.observed_generation,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1::ReplicaSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
    }
}

fn convert_replicaset_status_from_v1_to_beta2(
    status: &k8s_api::apps::v1::ReplicaSetStatus,
) -> k8s_api::apps::v1beta2::ReplicaSetStatus {
    k8s_api::apps::v1beta2::ReplicaSetStatus {
        replicas: status.replicas,
        fully_labeled_replicas: status.fully_labeled_replicas,
        ready_replicas: status.ready_replicas,
        available_replicas: status.available_replicas,
        terminating_replicas: None,
        observed_generation: status.observed_generation,
        conditions: status
            .conditions
            .iter()
            .map(|c| k8s_api::apps::v1beta2::ReplicaSetCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            })
            .collect(),
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

    #[test]
    fn test_deployment_v1beta2_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta2_deployment = k8s_api::apps::v1beta2::Deployment {
            metadata: ObjectMeta::named("beta2"),
            spec: Some(k8s_api::apps::v1beta2::DeploymentSpec {
                replicas: Some(2),
                template: k8s_api::core::v1::PodTemplateSpec {
                    metadata: ObjectMeta::named("tmpl"),
                    spec: Some(k8s_api::core::v1::PodSpec {
                        containers: vec![k8s_api::core::v1::Container::new(
                            "nginx",
                            "nginx:latest",
                        )],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let v1: k8s_api::apps::v1::Deployment = v1beta2_deployment.convert_to().unwrap();
        let roundtrip: k8s_api::apps::v1beta2::Deployment =
            k8s_api::apps::v1beta2::Deployment::convert_from(&v1).unwrap();

        assert_eq!(roundtrip.metadata.name, "beta2");
        assert_eq!(roundtrip.spec.as_ref().unwrap().replicas, Some(2));
    }
}
