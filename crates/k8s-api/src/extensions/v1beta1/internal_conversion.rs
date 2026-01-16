use super::*;

impl InternalConversion for ScaleSpec {
    type Internal = crate::extensions::internal::ScaleSpec;
}

impl InternalConversion for ScaleStatus {
    type Internal = crate::extensions::internal::ScaleStatus;
}

impl InternalConversion for Scale {
    type Internal = crate::extensions::internal::Scale;
}

impl InternalConversion for Deployment {
    type Internal = crate::extensions::internal::Deployment;
}

impl InternalConversion for DeploymentSpec {
    type Internal = crate::extensions::internal::DeploymentSpec;
}

impl InternalConversion for DeploymentRollback {
    type Internal = crate::extensions::internal::DeploymentRollback;
}

impl InternalConversion for RollbackConfig {
    type Internal = crate::extensions::internal::RollbackConfig;
}

impl InternalConversion for DeploymentStrategy {
    type Internal = crate::extensions::internal::DeploymentStrategy;
}

impl InternalConversion for RollingUpdateDeployment {
    type Internal = crate::extensions::internal::RollingUpdateDeployment;
}

impl InternalConversion for DeploymentStatus {
    type Internal = crate::extensions::internal::DeploymentStatus;
}

impl InternalConversion for DeploymentCondition {
    type Internal = crate::extensions::internal::DeploymentCondition;
}

impl InternalConversion for DeploymentList {
    type Internal = crate::extensions::internal::DeploymentList;
}

impl InternalConversion for DaemonSetUpdateStrategy {
    type Internal = crate::extensions::internal::DaemonSetUpdateStrategy;
}

impl InternalConversion for RollingUpdateDaemonSet {
    type Internal = crate::extensions::internal::RollingUpdateDaemonSet;
}

impl InternalConversion for DaemonSetSpec {
    type Internal = crate::extensions::internal::DaemonSetSpec;
}

impl InternalConversion for DaemonSetStatus {
    type Internal = crate::extensions::internal::DaemonSetStatus;
}

impl InternalConversion for DaemonSetCondition {
    type Internal = crate::extensions::internal::DaemonSetCondition;
}

impl InternalConversion for DaemonSet {
    type Internal = crate::extensions::internal::DaemonSet;
}

impl InternalConversion for DaemonSetList {
    type Internal = crate::extensions::internal::DaemonSetList;
}

impl InternalConversion for Ingress {
    type Internal = crate::extensions::internal::Ingress;
}

impl InternalConversion for IngressList {
    type Internal = crate::extensions::internal::IngressList;
}

impl InternalConversion for IngressSpec {
    type Internal = crate::extensions::internal::IngressSpec;
}

impl InternalConversion for IngressTLS {
    type Internal = crate::extensions::internal::IngressTLS;
}

impl InternalConversion for IngressStatus {
    type Internal = crate::extensions::internal::IngressStatus;
}

impl InternalConversion for IngressLoadBalancerStatus {
    type Internal = crate::extensions::internal::IngressLoadBalancerStatus;
}

impl InternalConversion for IngressLoadBalancerIngress {
    type Internal = crate::extensions::internal::IngressLoadBalancerIngress;
}

impl InternalConversion for IngressPortStatus {
    type Internal = crate::extensions::internal::IngressPortStatus;
}

impl InternalConversion for IngressRule {
    type Internal = crate::extensions::internal::IngressRule;
}

impl InternalConversion for IngressRuleValue {
    type Internal = crate::extensions::internal::IngressRuleValue;
}

impl InternalConversion for HTTPIngressRuleValue {
    type Internal = crate::extensions::internal::HTTPIngressRuleValue;
}

impl InternalConversion for HTTPIngressPath {
    type Internal = crate::extensions::internal::HTTPIngressPath;
}

impl InternalConversion for IngressBackend {
    type Internal = crate::extensions::internal::IngressBackend;
}

impl InternalConversion for ReplicaSet {
    type Internal = crate::extensions::internal::ReplicaSet;
}

impl InternalConversion for ReplicaSetList {
    type Internal = crate::extensions::internal::ReplicaSetList;
}

impl InternalConversion for ReplicaSetSpec {
    type Internal = crate::extensions::internal::ReplicaSetSpec;
}

impl InternalConversion for ReplicaSetStatus {
    type Internal = crate::extensions::internal::ReplicaSetStatus;
}

impl InternalConversion for ReplicaSetCondition {
    type Internal = crate::extensions::internal::ReplicaSetCondition;
}

impl InternalConversion for NetworkPolicy {
    type Internal = crate::extensions::internal::NetworkPolicy;
}

impl InternalConversion for NetworkPolicySpec {
    type Internal = crate::extensions::internal::NetworkPolicySpec;
}

impl InternalConversion for NetworkPolicyIngressRule {
    type Internal = crate::extensions::internal::NetworkPolicyIngressRule;
}

impl InternalConversion for NetworkPolicyEgressRule {
    type Internal = crate::extensions::internal::NetworkPolicyEgressRule;
}

impl InternalConversion for NetworkPolicyPort {
    type Internal = crate::extensions::internal::NetworkPolicyPort;
}

impl InternalConversion for IPBlock {
    type Internal = crate::extensions::internal::IPBlock;
}

impl InternalConversion for NetworkPolicyPeer {
    type Internal = crate::extensions::internal::NetworkPolicyPeer;
}

impl InternalConversion for NetworkPolicyList {
    type Internal = crate::extensions::internal::NetworkPolicyList;
}
