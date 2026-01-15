//! Admission v1beta1 type definitions (deprecated)

use k8s_apimachinery::apis::meta::v1::TypeMeta;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Operation = String;
pub type PatchType = String;

pub const OPERATION_CREATE: &str = "CREATE";
pub const OPERATION_UPDATE: &str = "UPDATE";
pub const OPERATION_DELETE: &str = "DELETE";
pub const OPERATION_CONNECT: &str = "CONNECT";

pub const PATCH_TYPE_JSON_PATCH: &str = "JSONPatch";

// =============================================================================
// AdmissionReview
// =============================================================================

/// AdmissionReview describes an admission review request/response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Request describes the attributes for the admission request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<AdmissionRequest>,
    /// Response describes the attributes for the admission response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<AdmissionResponse>,
}

/// AdmissionRequest describes the admission.Attributes for the admission request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionRequest {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Kind is the fully-qualified type of object being submitted.
    pub kind: GroupVersionKind,
    /// Resource is the fully-qualified resource being requested.
    pub resource: GroupVersionResource,
    /// SubResource is the subresource being requested, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_resource: String,
    /// RequestKind is the fully-qualified type of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_kind: Option<GroupVersionKind>,
    /// RequestResource is the fully-qualified resource of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_resource: Option<GroupVersionResource>,
    /// RequestSubResource is the name of the subresource of the original API request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_sub_resource: String,
    /// Name is the name of the object as presented in the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace is the namespace associated with the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Operation is the operation being performed.
    pub operation: Operation,
    /// UserInfo is information about the requesting user.
    pub user_info: UserInfo,
    /// Object is the object from the incoming request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<serde_json::Value>,
    /// OldObject is the existing object. Only populated for DELETE and UPDATE requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<serde_json::Value>,
    /// DryRun indicates that modifications will definitely not be persisted for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Options is the operation option structure of the operation being performed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// AdmissionResponse describes an admission response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionResponse {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Allowed indicates whether or not the admission request was permitted.
    pub allowed: bool,
    /// Result contains extra details into why an admission request was denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<k8s_apimachinery::apis::meta::v1::Status>,
    /// The patch body. Currently we only support "JSONPatch" which implements RFC 6902.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    /// The type of Patch. Currently we only allow "JSONPatch".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_type: Option<PatchType>,
    /// AuditAnnotations is an unstructured key value map set by remote admission controller.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub audit_annotations: HashMap<String, String>,
    /// warnings is a list of warning messages to return to the requesting API client.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// GroupVersionKind unambiguously identifies a kind.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionKind {
    pub group: String,
    pub version: String,
    pub kind: String,
}

/// GroupVersionResource unambiguously identifies a resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    pub group: String,
    pub version: String,
    pub resource: String,
}

/// UserInfo holds the information about the user needed to implement the user.Info interface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Vec<String>>,
}
