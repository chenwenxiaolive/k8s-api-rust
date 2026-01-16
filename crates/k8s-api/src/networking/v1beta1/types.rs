//! Networking v1beta1 type definitions
//!
//! This module provides beta-level networking types including:
//! - IPAddress (K8s 1.31+): Represents a single IP address for allocation
//! - ServiceCIDR (K8s 1.31+): Defines IP address ranges for ClusterIP allocation
//! - Ingress (deprecated in 1.19, use networking.k8s.io/v1): Legacy Ingress types

use k8s_api_core::IntOrString;
use k8s_apimachinery::apis::meta::v1::{Condition, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub use crate::core::v1::TypedLocalObjectReference;

pub type PathType = String;

// =============================================================================
// IPAddress (K8s 1.31+)
// =============================================================================

/// IPAddress represents a single IP of a single IP Family.
/// The object is designed to be used by APIs that operate on IP addresses.
/// The object is used by the Service core API for allocation of IP addresses.
/// An IP address can be represented in different formats, to guarantee the uniqueness of the IP,
/// the name of the object is the IP address in canonical format.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec is the desired state of the IPAddress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IPAddressSpec>,
}

/// IPAddressList contains a list of IPAddress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of IPAddresses.
    pub items: Vec<IPAddress>,
}

/// IPAddressSpec describe the attributes in an IP Address.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressSpec {
    /// ParentRef references the resource that an IPAddress is attached to.
    /// An IPAddress must reference a parent object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<ParentReference>,
}

/// ParentReference describes a reference to a parent object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    /// Group is the group of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Resource is the resource of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Namespace is the namespace of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Name is the name of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

// =============================================================================
// ServiceCIDR (K8s 1.31+)
// =============================================================================

/// ServiceCIDR defines a range of IP addresses using CIDR format.
/// This range is used to allocate ClusterIPs to Service objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDR {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec is the desired state of the ServiceCIDR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceCIDRSpec>,
    /// status represents the current state of the ServiceCIDR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceCIDRStatus>,
}

/// ServiceCIDRList contains a list of ServiceCIDR objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of ServiceCIDRs.
    pub items: Vec<ServiceCIDR>,
}

/// ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpec {
    /// CIDRs defines the IP blocks in CIDR notation from which to assign service cluster IPs.
    /// Max of two CIDRs is allowed, one of each IP family. This field is immutable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cidrs: Vec<String>,
}

/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRStatus {
    /// conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

// ServiceCIDR condition types
pub const SERVICE_CIDR_CONDITION_READY: &str = "Ready";
pub const SERVICE_CIDR_REASON_TERMINATING: &str = "Terminating";

// =============================================================================
// Ingress (deprecated in 1.19, use networking.k8s.io/v1)
// =============================================================================

/// Ingress is a collection of rules that allow inbound connections to reach the
/// endpoints defined by a backend.
/// Deprecated: use networking.k8s.io/v1 Ingress instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec is the desired state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,
    /// status is the current state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}

/// IngressList is a collection of Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of Ingress.
    pub items: Vec<Ingress>,
}


/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    /// ingressClassName is the name of the IngressClass cluster resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    /// backend is the default backend capable of servicing requests that don't match any rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<IngressBackend>,
    /// tls represents the TLS configuration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,
    /// rules is a list of host rules used to configure the Ingress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
}

/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    /// hosts is a list of hosts included in the TLS certificate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
    /// secretName is the name of the secret used to terminate TLS traffic on port 443.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
}

/// IngressStatus describes the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    /// loadBalancer contains the current status of the load-balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}

/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    /// ingress is a list containing ingress points for the load-balancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}

/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    /// ip is set for load-balancer ingress points that are IP based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// hostname is set for load-balancer ingress points that are DNS based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    /// ports provides information about the ports exposed by this LoadBalancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IngressPortStatus>,
}

/// IngressPortStatus represents the error condition of a service port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressPortStatus {
    /// port is the port number of the ingress port.
    pub port: i32,
    /// protocol is the protocol of the ingress port (TCP, UDP, SCTP).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    /// error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// IngressRule represents the rules mapping the paths under a specified host to
/// the related backend services.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    /// host is the fully qualified domain name of a network host.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(flatten)]
    pub ingress_rule_value: IngressRuleValue,
}

/// IngressRuleValue represents a rule to apply against incoming requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRuleValue {
    /// http is currently the only supported IngressRuleValue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}

/// HTTPIngressRuleValue is a list of http selectors pointing to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    /// paths is a collection of paths that map requests to backends.
    pub paths: Vec<HTTPIngressPath>,
}

/// HTTPIngressPath associates a path with a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    /// path is matched against the path of an incoming request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// pathType determines the interpretation of the path matching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_type: Option<PathType>,
    /// backend defines the referenced service endpoint to which the traffic will be forwarded.
    #[serde(default)]
    pub backend: IngressBackend,
}

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    /// serviceName specifies the name of the referenced service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_name: String,
    /// servicePort specifies the port of the referenced service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_port: Option<IntOrString>,
    /// resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<TypedLocalObjectReference>,
}

// Path type constants
pub const PATH_TYPE_EXACT: &str = "Exact";
pub const PATH_TYPE_PREFIX: &str = "Prefix";
pub const PATH_TYPE_IMPLEMENTATION_SPECIFIC: &str = "ImplementationSpecific";

// =============================================================================
// IngressClass (deprecated in 1.19, use networking.k8s.io/v1)
// =============================================================================

/// IngressClass represents the class of the Ingress, referenced by the Ingress Spec.
/// Deprecated: use networking.k8s.io/v1 IngressClass instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec is the desired state of the IngressClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressClassSpec>,
}

/// IngressClassList is a collection of IngressClasses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of IngressClasses.
    pub items: Vec<IngressClass>,
}

/// IngressClassSpec provides information about the class of an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassSpec {
    /// controller refers to the name of the controller that should handle this class.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub controller: String,
    /// parameters is a link to a custom resource containing additional configuration for the controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IngressClassParametersReference>,
}

/// IngressClassParametersReference identifies an API object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassParametersReference {
    /// apiGroup is the group for the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// kind is the type of resource being referenced.
    pub kind: String,
    /// name is the name of resource being referenced.
    pub name: String,
    /// scope represents if this refers to a cluster or namespace scoped resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// namespace is the namespace of the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

// IngressClass parameter reference scope constants
pub const INGRESS_CLASS_PARAMETERS_REFERENCE_SCOPE_NAMESPACE: &str = "Namespace";
pub const INGRESS_CLASS_PARAMETERS_REFERENCE_SCOPE_CLUSTER: &str = "Cluster";
