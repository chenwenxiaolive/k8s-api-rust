//! Discovery v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// EndpointSlice
// =============================================================================

/// EndpointSlice represents a subset of the endpoints that implement a service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// AddressType specifies the type of address carried by this EndpointSlice.
    pub address_type: String,
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
}

/// ForZone provides information about which zones should consume this endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForZone {
    /// Name represents the name of the zone.
    pub name: String,
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

/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}
