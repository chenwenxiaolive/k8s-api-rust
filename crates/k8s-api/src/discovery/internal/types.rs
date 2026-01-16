//! Internal type definitions for discovery.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub use crate::core::internal::ObjectReference;

pub type AddressType = String;

pub const ADDRESS_TYPE_FQDN: &str = "FQDN";
pub const ADDRESS_TYPE_IPV4: &str = "IPv4";
pub const ADDRESS_TYPE_IPV6: &str = "IPv6";


/// Endpoint represents a single logical "backend" implementing a service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    /// Addresses of this endpoint.
    pub addresses: Vec<String>,
    /// Conditions contains information about the current status of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<EndpointConditions>,
    /// Hostname of this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// TargetRef is a reference to a Kubernetes object that represents this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<ObjectReference>,
    /// DeprecatedTopology is deprecated and only retained for compatibility.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub deprecated_topology: std::collections::BTreeMap<String, String>,
    /// NodeName represents the name of the Node hosting this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// Zone is the name of the Zone this endpoint exists in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// Hints contains information associated with how an endpoint should be consumed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hints: Option<EndpointHints>,
}


/// EndpointConditions represents the current condition of an endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConditions {
    /// Ready indicates that this endpoint is prepared to receive traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// Serving is identical to ready except that it is set regardless of the terminating state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving: Option<bool>,
    /// Terminating indicates that this endpoint is terminating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating: Option<bool>,
}


/// EndpointHints provides hints describing how an endpoint should be consumed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHints {
    /// ForZones indicates the zone(s) this endpoint should be consumed by.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub for_zones: Vec<ForZone>,
    /// ForNodes indicates the node(s) this endpoint should be consumed by.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub for_nodes: Vec<ForNode>,
}


/// EndpointPort represents a Port used by an EndpointSlice.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    /// Name represents the name of this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol represents the IP protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Port represents the port number of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// AppProtocol represents the application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}


/// EndpointSlice represents a subset of the endpoints that implement a service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// AddressType specifies the type of address carried by this EndpointSlice.
    pub address_type: AddressType,
    /// Endpoints is a list of unique endpoints in this slice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
    /// Ports specifies the list of network ports exposed by each endpoint in this slice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<EndpointPort>,
}


/// EndpointSliceList represents a list of endpoint slices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSliceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<EndpointSlice>,
}


/// ForNode provides information about which nodes should consume this endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForNode {
    /// Name represents the name of the node.
    pub name: String,
}


/// ForZone provides information about which zones should consume this endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForZone {
    /// Name represents the name of the zone.
    pub name: String,
}
