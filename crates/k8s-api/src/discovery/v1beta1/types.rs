//! Discovery v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, TypeMeta};
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
    /// addressType specifies the type of address carried by this EndpointSlice.
    pub address_type: String,
    /// endpoints is a list of unique endpoints in this slice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
    /// ports specifies the list of network ports exposed by each endpoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<EndpointPort>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    /// addresses of this endpoint.
    pub addresses: Vec<String>,
    /// conditions contains information about the current status of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<EndpointConditions>,
    /// hostname of this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// targetRef is a reference to a Kubernetes object that represents this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<crate::core::v1::ObjectReference>,
    /// topology contains arbitrary topology information associated with the endpoint.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub topology: std::collections::BTreeMap<String, String>,
    /// nodeName represents the name of the Node hosting this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConditions {
    /// ready indicates that this endpoint is prepared to receive traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// serving is identical to ready except that it is set regardless of the terminating state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving: Option<bool>,
    /// terminating indicates that this endpoint is terminating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    /// name represents the name of this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// protocol represents the IP protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// port represents the port number of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// appProtocol represents the application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSliceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<EndpointSlice>,
}

// Address type constants
pub const ADDRESS_TYPE_IPV4: &str = "IPv4";
pub const ADDRESS_TYPE_IPV6: &str = "IPv6";
pub const ADDRESS_TYPE_FQDN: &str = "FQDN";
