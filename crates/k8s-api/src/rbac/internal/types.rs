//! Internal type definitions for rbac.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub const API_GROUP_ALL: &str = "*";
pub const AUTO_UPDATE_ANNOTATION_KEY: &str = "rbac.authorization.k8s.io/autoupdate";
pub const GROUP_KIND: &str = "Group";
pub const NON_RESOURCE_ALL: &str = "*";
pub const RESOURCE_ALL: &str = "*";
pub const SERVICE_ACCOUNT_KIND: &str = "ServiceAccount";
pub const USER_KIND: &str = "User";
pub const VERB_ALL: &str = "*";


/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregationRule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cluster_role_selectors: Vec<k8s_apimachinery::apis::meta::v1::LabelSelector>,
}


/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRole {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: Option<AggregationRule>,
}


/// ClusterRoleBinding references a ClusterRole, but not contain it. It can reference a ClusterRole in the global namespace, and adds who information via Subject.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}


/// ClusterRoleBindingList is a collection of ClusterRoleBindings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ClusterRoleBinding>,
}


/// ClusterRoleList is a collection of ClusterRoles.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ClusterRole>,
}


/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.
    pub verbs: Vec<String>,
    /// APIGroups is the name of the APIGroup that contains the resources.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    /// Resources is a list of resources this rule applies to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub non_resource_urls: Vec<String>,
}


/// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,
}


/// RoleBinding references a role, but does not contain it. It can reference a Role in the same namespace or a ClusterRole in the global namespace.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}


/// RoleBindingList is a collection of RoleBindings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<RoleBinding>,
}


/// RoleList is a collection of Roles.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Role>,
}


/// RoleRef contains information that points to the role being used.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced.
    pub api_group: String,
    /// Kind is the type of resource being referenced.
    pub kind: String,
    /// Name is the name of resource being referenced.
    pub name: String,
}


/// Subject contains a reference to the object or user identities a role binding applies to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount".
    pub kind: String,
    /// APIGroup holds the API group of the referenced subject.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    /// Name of the object being referenced.
    pub name: String,
    /// Namespace of the referenced object. If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}
