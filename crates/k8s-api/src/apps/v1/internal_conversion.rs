use super::*;

impl InternalConversion for Deployment {
    type Internal = crate::apps::internal::Deployment;
}

impl InternalConversion for DeploymentSpec {
    type Internal = crate::apps::internal::DeploymentSpec;
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

impl InternalConversion for DaemonSet {
    type Internal = crate::apps::internal::DaemonSet;
}

impl InternalConversion for DaemonSetSpec {
    type Internal = crate::apps::internal::DaemonSetSpec;
}

impl InternalConversion for DaemonSetUpdateStrategy {
    type Internal = crate::apps::internal::DaemonSetUpdateStrategy;
}

impl InternalConversion for RollingUpdateDaemonSet {
    type Internal = crate::apps::internal::RollingUpdateDaemonSet;
}

impl InternalConversion for DaemonSetStatus {
    type Internal = crate::apps::internal::DaemonSetStatus;
}

impl InternalConversion for DaemonSetCondition {
    type Internal = crate::apps::internal::DaemonSetCondition;
}

impl InternalConversion for DaemonSetList {
    type Internal = crate::apps::internal::DaemonSetList;
}

impl InternalConversion for ReplicaSet {
    type Internal = crate::apps::internal::ReplicaSet;
}

impl InternalConversion for ReplicaSetSpec {
    type Internal = crate::apps::internal::ReplicaSetSpec;
}

impl InternalConversion for ReplicaSetStatus {
    type Internal = crate::apps::internal::ReplicaSetStatus;
}

impl InternalConversion for ReplicaSetCondition {
    type Internal = crate::apps::internal::ReplicaSetCondition;
}

impl InternalConversion for ReplicaSetList {
    type Internal = crate::apps::internal::ReplicaSetList;
}

impl InternalConversion for ControllerRevision {
    type Internal = crate::apps::internal::ControllerRevision;
}

impl InternalConversion for ControllerRevisionList {
    type Internal = crate::apps::internal::ControllerRevisionList;
}
