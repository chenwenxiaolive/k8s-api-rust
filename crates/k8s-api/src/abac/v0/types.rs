//! ABAC v0 API type definitions

use k8s_apimachinery::apis::meta::v1::TypeMeta;
use serde::{Deserialize, Serialize};

// =============================================================================
// Policy
// =============================================================================

/// Policy contains a single ABAC policy rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// User is the username this rule applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// Group is the group this rule applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Readonly matches readonly requests when true, and all requests when false.
    #[serde(default, skip_serializing_if = "is_false", rename = "readonly")]
    pub read_only: bool,
    /// Resource is the name of a resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Namespace is the name of a namespace.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

fn is_false(value: &bool) -> bool {
    !*value
}
