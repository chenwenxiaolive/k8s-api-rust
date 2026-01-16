//! Internal type definitions for apiregistration.

use k8s_apimachinery::apis::meta::v1::{Condition, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};


/// APIService represents a server for a particular GroupVersion.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIService {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<APIServiceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<APIServiceStatus>,
}


/// APIServiceList is a list of APIService objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<APIService>,
}


/// APIServiceSpec contains information for locating and communicating with a server.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceSpec {
    /// Service is a reference to the service for this API server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,
    /// Group is the API group name this server hosts.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version this server hosts.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ca_bundle: String,
    /// GroupPriorityMinimum is the priority this group should have at least.
    pub group_priority_minimum: i32,
    /// VersionPriority controls the ordering of this API version inside of its group.
    pub version_priority: i32,
}


/// APIServiceStatus contains derived information about an API server.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceStatus {
    /// Conditions are the current service state of the APIService.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}


/// ServiceReference holds a reference to Service.legacy.k8s.io.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Name is the name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Port is the port on the service that hosts the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
