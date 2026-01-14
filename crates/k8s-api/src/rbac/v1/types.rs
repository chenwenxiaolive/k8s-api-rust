//! RBAC v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// Role
// =============================================================================

/// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,
}

/// RoleList is a collection of Roles.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Role>,
}

// =============================================================================
// ClusterRole
// =============================================================================

/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRole {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: Option<AggregationRule>,
}

/// ClusterRoleList is a collection of ClusterRoles.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ClusterRole>,
}

/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregationRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_role_selectors: Vec<k8s_apimachinery::apis::meta::v1::LabelSelector>,
}

// =============================================================================
// RoleBinding
// =============================================================================

/// RoleBinding references a role, but does not contain it. It can reference a Role in the same namespace or a ClusterRole in the global namespace.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}

/// RoleBindingList is a collection of RoleBindings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<RoleBinding>,
}

// =============================================================================
// ClusterRoleBinding
// =============================================================================

/// ClusterRoleBinding references a ClusterRole, but not contain it. It can reference a ClusterRole in the global namespace, and adds who information via Subject.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}

/// ClusterRoleBindingList is a collection of ClusterRoleBindings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ClusterRoleBinding>,
}

// =============================================================================
// PolicyRule
// =============================================================================

/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.
    pub verbs: Vec<String>,
    /// APIGroups is the name of the APIGroup that contains the resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_urls: Vec<String>,
}

// =============================================================================
// Subject
// =============================================================================

/// Subject contains a reference to the object or user identities a role binding applies to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount".
    pub kind: String,
    /// APIGroup holds the API group of the referenced subject.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    /// Name of the object being referenced.
    pub name: String,
    /// Namespace of the referenced object. If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

// =============================================================================
// RoleRef
// =============================================================================

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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_serialization_roundtrip() {
        let role = Role {
            type_meta: TypeMeta::new("rbac.authorization.k8s.io/v1", "Role"),
            metadata: ObjectMeta::namespaced("default", "pod-reader"),
            rules: vec![PolicyRule {
                verbs: vec!["get".to_string(), "watch".to_string(), "list".to_string()],
                api_groups: vec!["".to_string()],
                resources: vec!["pods".to_string()],
                ..Default::default()
            }],
        };

        let json = serde_json::to_string(&role).unwrap();
        let parsed: Role = serde_json::from_str(&json).unwrap();

        assert_eq!(role.metadata.name, parsed.metadata.name);
        assert_eq!(role.rules.len(), parsed.rules.len());
        assert_eq!(role.rules[0].verbs, parsed.rules[0].verbs);
    }

    #[test]
    fn test_cluster_role_serialization_roundtrip() {
        let cluster_role = ClusterRole {
            type_meta: TypeMeta::new("rbac.authorization.k8s.io/v1", "ClusterRole"),
            metadata: ObjectMeta::named("secret-reader"),
            rules: vec![PolicyRule {
                verbs: vec!["get".to_string(), "watch".to_string(), "list".to_string()],
                api_groups: vec!["".to_string()],
                resources: vec!["secrets".to_string()],
                ..Default::default()
            }],
            aggregation_rule: None,
        };

        let json = serde_json::to_string(&cluster_role).unwrap();
        let parsed: ClusterRole = serde_json::from_str(&json).unwrap();

        assert_eq!(cluster_role.metadata.name, parsed.metadata.name);
        assert_eq!(cluster_role.rules.len(), parsed.rules.len());
    }

    #[test]
    fn test_role_binding_serialization_roundtrip() {
        let role_binding = RoleBinding {
            type_meta: TypeMeta::new("rbac.authorization.k8s.io/v1", "RoleBinding"),
            metadata: ObjectMeta::namespaced("default", "read-pods"),
            subjects: vec![Subject {
                kind: "User".to_string(),
                api_group: "rbac.authorization.k8s.io".to_string(),
                name: "jane".to_string(),
                namespace: String::new(),
            }],
            role_ref: RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "Role".to_string(),
                name: "pod-reader".to_string(),
            },
        };

        let json = serde_json::to_string(&role_binding).unwrap();
        let parsed: RoleBinding = serde_json::from_str(&json).unwrap();

        assert_eq!(role_binding.metadata.name, parsed.metadata.name);
        assert_eq!(role_binding.subjects.len(), parsed.subjects.len());
        assert_eq!(role_binding.role_ref.name, parsed.role_ref.name);
    }

    #[test]
    fn test_cluster_role_binding_serialization_roundtrip() {
        let crb = ClusterRoleBinding {
            type_meta: TypeMeta::new("rbac.authorization.k8s.io/v1", "ClusterRoleBinding"),
            metadata: ObjectMeta::named("read-secrets-global"),
            subjects: vec![
                Subject {
                    kind: "Group".to_string(),
                    api_group: "rbac.authorization.k8s.io".to_string(),
                    name: "manager".to_string(),
                    namespace: String::new(),
                },
                Subject {
                    kind: "ServiceAccount".to_string(),
                    name: "default".to_string(),
                    namespace: "kube-system".to_string(),
                    api_group: String::new(),
                },
            ],
            role_ref: RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "ClusterRole".to_string(),
                name: "secret-reader".to_string(),
            },
        };

        let json = serde_json::to_string(&crb).unwrap();
        let parsed: ClusterRoleBinding = serde_json::from_str(&json).unwrap();

        assert_eq!(crb.metadata.name, parsed.metadata.name);
        assert_eq!(crb.subjects.len(), parsed.subjects.len());
        assert_eq!(crb.subjects[0].kind, parsed.subjects[0].kind);
        assert_eq!(crb.subjects[1].namespace, parsed.subjects[1].namespace);
    }
}
