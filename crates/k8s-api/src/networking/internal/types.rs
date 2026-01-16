//! Internal type definitions for networking.

use k8s_apimachinery::apis::meta::v1::{Condition, ListMeta, ObjectMeta, TypeMeta, LabelSelector};
use serde::{Deserialize, Serialize};

pub use crate::core::internal::TypedLocalObjectReference;

pub type PathType = String;
pub type PolicyType = String;

pub const INGRESS_CLASS_PARAMETERS_REFERENCE_SCOPE_CLUSTER: &str = "Cluster";
pub const INGRESS_CLASS_PARAMETERS_REFERENCE_SCOPE_NAMESPACE: &str = "Namespace";
pub const PATH_TYPE_EXACT: &str = "Exact";
pub const PATH_TYPE_IMPLEMENTATION_SPECIFIC: &str = "ImplementationSpecific";
pub const PATH_TYPE_PREFIX: &str = "Prefix";
pub const POLICY_TYPE_EGRESS: &str = "Egress";
pub const POLICY_TYPE_INGRESS: &str = "Ingress";
pub const SERVICE_CIDR_CONDITION_READY: &str = "Ready";
pub const SERVICE_CIDR_REASON_TERMINATING: &str = "Terminating";


/// HTTPIngressPath associates a path with a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_type: Option<PathType>,
    pub backend: IngressBackend,
}


/// HTTPIngressRuleValue is a list of http selectors pointing to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    pub paths: Vec<HTTPIngressPath>,
}


/// IPAddress represents a single IP of a single IP Family.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<IPAddressSpec>,
}


/// IPAddressList contains a list of IPAddress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<IPAddress>,
}


/// IPAddressSpec describes the attributes in an IP Address.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<ParentReference>,
}


/// IPBlock describes a particular CIDR range.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    pub cidr: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<String>,
}


/// Ingress is a collection of rules that allow inbound connections to reach the endpoints.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}


/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<IngressServiceBackend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<TypedLocalObjectReference>,
}


/// IngressClass represents the class of the Ingress, referenced by the Ingress Spec.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressClassSpec>,
}


/// IngressClassList is a collection of IngressClasses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<IngressClass>,
}


/// IngressClassParametersReference identifies an API object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassParametersReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}


/// IngressClassSpec provides information about the class of an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassSpec {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IngressClassParametersReference>,
}


/// IngressList is a collection of Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<Ingress>,
}


/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IngressPortStatus>,
}


/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}


/// IngressPortStatus represents the error condition of a service port.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressPortStatus {
    pub port: i32,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}


/// IngressRule represents the rules mapping the paths under a specified host.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(flatten)]
    pub ingress_rule_value: IngressRuleValue,
}


/// IngressRuleValue represents a rule to apply against incoming requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRuleValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}


/// IngressServiceBackend references a Kubernetes Service as a Backend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressServiceBackend {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<ServiceBackendPort>,
}


/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<IngressBackend>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
}


/// IngressStatus describes the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}


/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
}


/// NetworkPolicy describes what network traffic is allowed for a set of Pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}


/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<NetworkPolicyPeer>,
}


/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<NetworkPolicyPeer>,
}


/// NetworkPolicyList is a list of NetworkPolicy objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<NetworkPolicy>,
}


/// NetworkPolicyPeer describes a peer to allow traffic to/from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<LabelSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<IPBlock>,
}


/// NetworkPolicyPort describes a port to allow traffic on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::core::v1::Protocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
}


/// NetworkPolicySpec provides the specification of a NetworkPolicy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpec {
    pub pod_selector: LabelSelector,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<NetworkPolicyIngressRule>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<NetworkPolicyEgressRule>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policy_types: Vec<PolicyType>,
}


/// ParentReference describes a reference to a parent object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
}


/// ServiceBackendPort is the service port being referenced.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceBackendPort {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub number: i32,
}


/// ServiceCIDR defines a range of IP addresses using CIDR format.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDR {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceCIDRSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceCIDRStatus>,
}


/// ServiceCIDRList contains a list of ServiceCIDR objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<ServiceCIDR>,
}


/// ServiceCIDRSpec defines the CIDRs for allocating ClusterIPs for Services.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpec {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cidrs: Vec<String>,
}


/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRStatus {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}
