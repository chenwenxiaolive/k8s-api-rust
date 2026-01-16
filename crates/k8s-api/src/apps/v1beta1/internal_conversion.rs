use super::*;

impl InternalConversion for Deployment {
    type Internal = crate::apps::internal::Deployment;
}

impl InternalConversion for DeploymentSpec {
    type Internal = crate::apps::internal::DeploymentSpec;
}

impl InternalConversion for RollbackConfig {
    type Internal = crate::apps::internal::RollbackConfig;
}

impl InternalConversion for DeploymentStrategy {
    type Internal = crate::apps::internal::DeploymentStrategy;
}

impl InternalConversion for RollingUpdateDeployment {
    type Internal = crate::apps::internal::RollingUpdateDeployment;
}

impl InternalConversion for DeploymentStatus {
    type Internal = crate::apps::internal::DeploymentStatus;
}

impl InternalConversion for DeploymentCondition {
    type Internal = crate::apps::internal::DeploymentCondition;
}

impl InternalConversion for DeploymentList {
    type Internal = crate::apps::internal::DeploymentList;
}

impl InternalConversion for DeploymentRollback {
    type Internal = crate::apps::internal::DeploymentRollback;
}

impl InternalConversion for StatefulSet {
    type Internal = crate::apps::internal::StatefulSet;
}

impl InternalConversion for StatefulSetSpec {
    type Internal = crate::apps::internal::StatefulSetSpec;
}

impl InternalConversion for StatefulSetUpdateStrategy {
    type Internal = crate::apps::internal::StatefulSetUpdateStrategy;
}

impl InternalConversion for RollingUpdateStatefulSetStrategy {
    type Internal = crate::apps::internal::RollingUpdateStatefulSetStrategy;
}

impl InternalConversion for StatefulSetPersistentVolumeClaimRetentionPolicy {
    type Internal = crate::apps::internal::StatefulSetPersistentVolumeClaimRetentionPolicy;
}

impl InternalConversion for StatefulSetOrdinals {
    type Internal = crate::apps::internal::StatefulSetOrdinals;
}

impl InternalConversion for StatefulSetStatus {
    type Internal = crate::apps::internal::StatefulSetStatus;
}

impl InternalConversion for StatefulSetCondition {
    type Internal = crate::apps::internal::StatefulSetCondition;
}

impl InternalConversion for StatefulSetList {
    type Internal = crate::apps::internal::StatefulSetList;
}

impl InternalConversion for ControllerRevision {
    type Internal = crate::apps::internal::ControllerRevision;
}

impl InternalConversion for ControllerRevisionList {
    type Internal = crate::apps::internal::ControllerRevisionList;
}

impl InternalConversion for Scale {
    type Internal = crate::apps::internal::Scale;
}

impl InternalConversion for ScaleSpec {
    type Internal = crate::apps::internal::ScaleSpec;
}

impl InternalConversion for ScaleStatus {
    type Internal = crate::apps::internal::ScaleStatus;
}
