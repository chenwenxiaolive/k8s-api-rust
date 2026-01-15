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

// =============================================================================
// ControllerRevision: v1beta1/v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::ControllerRevision>
    for k8s_api::apps::v1beta1::ControllerRevision
{
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ControllerRevision, ConversionError> {
        Ok(k8s_api::apps::v1::ControllerRevision {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1",
                "ControllerRevision",
            ),
            metadata: self.metadata.clone(),
            data: self.data.clone(),
            revision: self.revision,
        })
    }

    fn convert_from(
        other: &k8s_api::apps::v1::ControllerRevision,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta1",
                "ControllerRevision",
            ),
            metadata: other.metadata.clone(),
            data: other.data.clone(),
            revision: other.revision,
        })
    }
}

impl Convertible<k8s_api::apps::v1::ControllerRevision>
    for k8s_api::apps::v1beta2::ControllerRevision
{
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ControllerRevision, ConversionError> {
        Ok(k8s_api::apps::v1::ControllerRevision {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1",
                "ControllerRevision",
            ),
            metadata: self.metadata.clone(),
            data: self.data.clone(),
            revision: self.revision,
        })
    }

    fn convert_from(
        other: &k8s_api::apps::v1::ControllerRevision,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "ControllerRevision",
            ),
            metadata: other.metadata.clone(),
            data: other.data.clone(),
            revision: other.revision,
        })
    }
}

// =============================================================================
// ControllerRevisionList: v1beta1/v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::ControllerRevisionList>
    for k8s_api::apps::v1beta1::ControllerRevisionList
{
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ControllerRevisionList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::ControllerRevisionList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1",
                "ControllerRevisionList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::ControllerRevisionList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta1::ControllerRevision::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta1",
                "ControllerRevisionList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::apps::v1::ControllerRevisionList>
    for k8s_api::apps::v1beta2::ControllerRevisionList
{
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ControllerRevisionList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::ControllerRevisionList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1",
                "ControllerRevisionList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::ControllerRevisionList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta2::ControllerRevision::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "ControllerRevisionList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// DeploymentList: v1beta1/v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::DeploymentList> for k8s_api::apps::v1beta1::DeploymentList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::DeploymentList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::DeploymentList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "DeploymentList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::DeploymentList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta1::Deployment::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta1",
                "DeploymentList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::apps::v1::DeploymentList> for k8s_api::apps::v1beta2::DeploymentList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::DeploymentList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::DeploymentList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "DeploymentList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::DeploymentList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta2::Deployment::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "DeploymentList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// StatefulSetList: v1beta1/v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::StatefulSetList> for k8s_api::apps::v1beta1::StatefulSetList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::StatefulSetList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::StatefulSetList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "StatefulSetList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::StatefulSetList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta1::StatefulSet::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta1",
                "StatefulSetList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::apps::v1::StatefulSetList> for k8s_api::apps::v1beta2::StatefulSetList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::StatefulSetList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::StatefulSetList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "StatefulSetList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::StatefulSetList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta2::StatefulSet::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "StatefulSetList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// DaemonSetList: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::DaemonSetList> for k8s_api::apps::v1beta2::DaemonSetList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::DaemonSetList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::DaemonSetList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "DaemonSetList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::DaemonSetList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta2::DaemonSet::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "DaemonSetList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// ReplicaSetList: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::apps::v1::ReplicaSetList> for k8s_api::apps::v1beta2::ReplicaSetList {
    fn convert_to(&self) -> Result<k8s_api::apps::v1::ReplicaSetList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::apps::v1::ReplicaSetList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1", "ReplicaSetList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::apps::v1::ReplicaSetList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::apps::v1beta2::ReplicaSet::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "apps/v1beta2",
                "ReplicaSetList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// Scale: v1beta1 <-> v1beta2
// =============================================================================

impl Convertible<k8s_api::apps::v1beta2::Scale> for k8s_api::apps::v1beta1::Scale {
    fn convert_to(&self) -> Result<k8s_api::apps::v1beta2::Scale, ConversionError> {
        Ok(k8s_api::apps::v1beta2::Scale {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta2", "Scale"),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(convert_scale_spec_to_v1beta2),
            status: self.status.as_ref().map(convert_scale_status_to_v1beta2),
        })
    }

    fn convert_from(other: &k8s_api::apps::v1beta2::Scale) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("apps/v1beta1", "Scale"),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(convert_scale_spec_from_v1beta2),
            status: other.status.as_ref().map(convert_scale_status_from_v1beta2),
        })
    }
}

fn convert_scale_spec_to_v1beta2(
    spec: &k8s_api::apps::v1beta1::ScaleSpec,
) -> k8s_api::apps::v1beta2::ScaleSpec {
    k8s_api::apps::v1beta2::ScaleSpec {
        replicas: spec.replicas.unwrap_or(0),
    }
}

fn convert_scale_spec_from_v1beta2(
    spec: &k8s_api::apps::v1beta2::ScaleSpec,
) -> k8s_api::apps::v1beta1::ScaleSpec {
    k8s_api::apps::v1beta1::ScaleSpec {
        replicas: Some(spec.replicas),
    }
}

fn convert_scale_status_to_v1beta2(
    status: &k8s_api::apps::v1beta1::ScaleStatus,
) -> k8s_api::apps::v1beta2::ScaleStatus {
    let selector = status
        .selector
        .as_deref()
        .map(parse_selector_string)
        .unwrap_or_default();

    k8s_api::apps::v1beta2::ScaleStatus {
        replicas: status.replicas,
        selector,
        target_selector: status.target_selector.clone().unwrap_or_default(),
    }
}

fn convert_scale_status_from_v1beta2(
    status: &k8s_api::apps::v1beta2::ScaleStatus,
) -> k8s_api::apps::v1beta1::ScaleStatus {
    let selector = if status.selector.is_empty() {
        None
    } else {
        Some(
            status
                .selector
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(","),
        )
    };
    let target_selector = if status.target_selector.is_empty() {
        None
    } else {
        Some(status.target_selector.clone())
    };

    k8s_api::apps::v1beta1::ScaleStatus {
        replicas: status.replicas,
        selector,
        target_selector,
    }
}

fn parse_selector_string(value: &str) -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    for part in value.split(',') {
        if let Some((key, val)) = part.split_once('=') {
            let key = key.trim();
            let val = val.trim();
            if !key.is_empty() && !val.is_empty() {
                map.insert(key.to_string(), val.to_string());
            }
        }
    }
    map
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

    #[test]
    fn test_controller_revision_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta1_revision = k8s_api::apps::v1beta1::ControllerRevision {
            metadata: ObjectMeta::named("rev-1"),
            data: Some(serde_json::json!({"key": "value"})),
            revision: 1,
            ..Default::default()
        };

        let v1_revision: k8s_api::apps::v1::ControllerRevision =
            v1beta1_revision.convert_to().unwrap();
        assert_eq!(v1_revision.metadata.name, "rev-1");
        assert_eq!(v1_revision.revision, 1);

        let v1beta2_roundtrip: k8s_api::apps::v1beta2::ControllerRevision =
            k8s_api::apps::v1beta2::ControllerRevision::convert_from(&v1_revision).unwrap();
        assert_eq!(v1beta2_roundtrip.metadata.name, "rev-1");
        assert_eq!(v1beta2_roundtrip.revision, 1);
    }

    #[test]
    fn test_deployment_list_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

        let list = k8s_api::apps::v1beta1::DeploymentList {
            metadata: ListMeta::default(),
            items: vec![k8s_api::apps::v1beta1::Deployment {
                metadata: ObjectMeta::named("deploy"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::apps::v1::DeploymentList = list.convert_to().unwrap();
        assert_eq!(v1_list.items.len(), 1);

        let roundtrip: k8s_api::apps::v1beta1::DeploymentList =
            k8s_api::apps::v1beta1::DeploymentList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items[0].metadata.name, "deploy");
    }

    #[test]
    fn test_scale_beta_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta1_scale = k8s_api::apps::v1beta1::Scale {
            metadata: ObjectMeta::named("scale"),
            spec: Some(k8s_api::apps::v1beta1::ScaleSpec { replicas: Some(3) }),
            status: Some(k8s_api::apps::v1beta1::ScaleStatus {
                replicas: 2,
                selector: Some("app=demo".to_string()),
                target_selector: Some("app=demo".to_string()),
            }),
            ..Default::default()
        };

        let v1beta2_scale: k8s_api::apps::v1beta2::Scale = v1beta1_scale.convert_to().unwrap();
        assert_eq!(v1beta2_scale.status.as_ref().unwrap().replicas, 2);

        let roundtrip: k8s_api::apps::v1beta1::Scale =
            k8s_api::apps::v1beta1::Scale::convert_from(&v1beta2_scale).unwrap();
        assert_eq!(roundtrip.spec.as_ref().unwrap().replicas, Some(3));
    }
}
