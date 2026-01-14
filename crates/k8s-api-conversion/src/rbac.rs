//! RBAC API conversions
//!
//! This module provides conversions between RBAC API versions.
//!
//! Note: v1beta1 is deprecated. v1 is the stable version.

use crate::{ConversionError, Convertible};

// =============================================================================
// Role: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::rbac::v1::Role> for k8s_api::rbac::v1beta1::Role {
    fn convert_to(&self) -> Result<k8s_api::rbac::v1::Role, ConversionError> {
        Ok(k8s_api::rbac::v1::Role {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1",
                "Role",
            ),
            metadata: self.metadata.clone(),
            rules: self.rules.iter().map(convert_policy_rule_to_v1).collect(),
        })
    }

    fn convert_from(other: &k8s_api::rbac::v1::Role) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1beta1",
                "Role",
            ),
            metadata: other.metadata.clone(),
            rules: other.rules.iter().map(convert_policy_rule_from_v1).collect(),
        })
    }
}

// =============================================================================
// ClusterRole: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::rbac::v1::ClusterRole> for k8s_api::rbac::v1beta1::ClusterRole {
    fn convert_to(&self) -> Result<k8s_api::rbac::v1::ClusterRole, ConversionError> {
        Ok(k8s_api::rbac::v1::ClusterRole {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1",
                "ClusterRole",
            ),
            metadata: self.metadata.clone(),
            rules: self.rules.iter().map(convert_policy_rule_to_v1).collect(),
            aggregation_rule: self.aggregation_rule.as_ref().map(convert_aggregation_rule_to_v1),
        })
    }

    fn convert_from(other: &k8s_api::rbac::v1::ClusterRole) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1beta1",
                "ClusterRole",
            ),
            metadata: other.metadata.clone(),
            rules: other.rules.iter().map(convert_policy_rule_from_v1).collect(),
            aggregation_rule: other
                .aggregation_rule
                .as_ref()
                .map(convert_aggregation_rule_from_v1),
        })
    }
}

// =============================================================================
// RoleBinding: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::rbac::v1::RoleBinding> for k8s_api::rbac::v1beta1::RoleBinding {
    fn convert_to(&self) -> Result<k8s_api::rbac::v1::RoleBinding, ConversionError> {
        Ok(k8s_api::rbac::v1::RoleBinding {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1",
                "RoleBinding",
            ),
            metadata: self.metadata.clone(),
            subjects: self.subjects.iter().map(convert_subject_to_v1).collect(),
            role_ref: convert_role_ref_to_v1(&self.role_ref),
        })
    }

    fn convert_from(other: &k8s_api::rbac::v1::RoleBinding) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1beta1",
                "RoleBinding",
            ),
            metadata: other.metadata.clone(),
            subjects: other.subjects.iter().map(convert_subject_from_v1).collect(),
            role_ref: convert_role_ref_from_v1(&other.role_ref),
        })
    }
}

// =============================================================================
// ClusterRoleBinding: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::rbac::v1::ClusterRoleBinding>
    for k8s_api::rbac::v1beta1::ClusterRoleBinding
{
    fn convert_to(&self) -> Result<k8s_api::rbac::v1::ClusterRoleBinding, ConversionError> {
        Ok(k8s_api::rbac::v1::ClusterRoleBinding {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1",
                "ClusterRoleBinding",
            ),
            metadata: self.metadata.clone(),
            subjects: self.subjects.iter().map(convert_subject_to_v1).collect(),
            role_ref: convert_role_ref_to_v1(&self.role_ref),
        })
    }

    fn convert_from(
        other: &k8s_api::rbac::v1::ClusterRoleBinding,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "rbac.authorization.k8s.io/v1beta1",
                "ClusterRoleBinding",
            ),
            metadata: other.metadata.clone(),
            subjects: other.subjects.iter().map(convert_subject_from_v1).collect(),
            role_ref: convert_role_ref_from_v1(&other.role_ref),
        })
    }
}

// =============================================================================
// Helper conversion functions
// =============================================================================

fn convert_policy_rule_to_v1(
    rule: &k8s_api::rbac::v1beta1::PolicyRule,
) -> k8s_api::rbac::v1::PolicyRule {
    k8s_api::rbac::v1::PolicyRule {
        verbs: rule.verbs.clone(),
        api_groups: rule.api_groups.clone(),
        resources: rule.resources.clone(),
        resource_names: rule.resource_names.clone(),
        non_resource_urls: rule.non_resource_urls.clone(),
    }
}

fn convert_policy_rule_from_v1(
    rule: &k8s_api::rbac::v1::PolicyRule,
) -> k8s_api::rbac::v1beta1::PolicyRule {
    k8s_api::rbac::v1beta1::PolicyRule {
        verbs: rule.verbs.clone(),
        api_groups: rule.api_groups.clone(),
        resources: rule.resources.clone(),
        resource_names: rule.resource_names.clone(),
        non_resource_urls: rule.non_resource_urls.clone(),
    }
}

fn convert_subject_to_v1(subject: &k8s_api::rbac::v1beta1::Subject) -> k8s_api::rbac::v1::Subject {
    k8s_api::rbac::v1::Subject {
        kind: subject.kind.clone(),
        api_group: subject.api_group.clone(),
        name: subject.name.clone(),
        namespace: subject.namespace.clone(),
    }
}

fn convert_subject_from_v1(subject: &k8s_api::rbac::v1::Subject) -> k8s_api::rbac::v1beta1::Subject {
    k8s_api::rbac::v1beta1::Subject {
        kind: subject.kind.clone(),
        api_group: subject.api_group.clone(),
        name: subject.name.clone(),
        namespace: subject.namespace.clone(),
    }
}

fn convert_role_ref_to_v1(role_ref: &k8s_api::rbac::v1beta1::RoleRef) -> k8s_api::rbac::v1::RoleRef {
    k8s_api::rbac::v1::RoleRef {
        api_group: role_ref.api_group.clone(),
        kind: role_ref.kind.clone(),
        name: role_ref.name.clone(),
    }
}

fn convert_role_ref_from_v1(role_ref: &k8s_api::rbac::v1::RoleRef) -> k8s_api::rbac::v1beta1::RoleRef {
    k8s_api::rbac::v1beta1::RoleRef {
        api_group: role_ref.api_group.clone(),
        kind: role_ref.kind.clone(),
        name: role_ref.name.clone(),
    }
}

fn convert_aggregation_rule_to_v1(
    rule: &k8s_api::rbac::v1beta1::AggregationRule,
) -> k8s_api::rbac::v1::AggregationRule {
    k8s_api::rbac::v1::AggregationRule {
        cluster_role_selectors: rule.cluster_role_selectors.clone(),
    }
}

fn convert_aggregation_rule_from_v1(
    rule: &k8s_api::rbac::v1::AggregationRule,
) -> k8s_api::rbac::v1beta1::AggregationRule {
    k8s_api::rbac::v1beta1::AggregationRule {
        cluster_role_selectors: rule.cluster_role_selectors.clone(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_role_v1beta1_to_v1() {
        let v1beta1_role = k8s_api::rbac::v1beta1::Role {
            metadata: ObjectMeta {
                name: "test-role".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            rules: vec![k8s_api::rbac::v1beta1::PolicyRule {
                verbs: vec!["get".to_string(), "list".to_string()],
                api_groups: vec!["".to_string()],
                resources: vec!["pods".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_role: k8s_api::rbac::v1::Role = v1beta1_role.convert_to().unwrap();

        assert_eq!(v1_role.metadata.name, "test-role");
        assert_eq!(v1_role.rules.len(), 1);
        assert_eq!(v1_role.rules[0].verbs, vec!["get", "list"]);
        assert_eq!(v1_role.rules[0].resources, vec!["pods"]);
    }

    #[test]
    fn test_role_v1_to_v1beta1() {
        let v1_role = k8s_api::rbac::v1::Role {
            metadata: ObjectMeta {
                name: "test-role".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            rules: vec![k8s_api::rbac::v1::PolicyRule {
                verbs: vec!["get".to_string(), "list".to_string()],
                api_groups: vec!["".to_string()],
                resources: vec!["pods".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1beta1_role = k8s_api::rbac::v1beta1::Role::convert_from(&v1_role).unwrap();

        assert_eq!(v1beta1_role.metadata.name, "test-role");
        assert_eq!(v1beta1_role.rules.len(), 1);
        assert_eq!(v1beta1_role.rules[0].verbs, vec!["get", "list"]);
    }

    #[test]
    fn test_cluster_role_with_aggregation() {
        let v1beta1_cr = k8s_api::rbac::v1beta1::ClusterRole {
            metadata: ObjectMeta {
                name: "aggregate-role".to_string(),
                ..Default::default()
            },
            rules: vec![],
            aggregation_rule: Some(k8s_api::rbac::v1beta1::AggregationRule {
                cluster_role_selectors: vec![k8s_apimachinery::apis::meta::v1::LabelSelector {
                    match_labels: [("aggregate".to_string(), "true".to_string())]
                        .into_iter()
                        .collect(),
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };

        let v1_cr: k8s_api::rbac::v1::ClusterRole = v1beta1_cr.convert_to().unwrap();

        assert!(v1_cr.aggregation_rule.is_some());
        assert_eq!(
            v1_cr.aggregation_rule.unwrap().cluster_role_selectors.len(),
            1
        );
    }

    #[test]
    fn test_role_binding_v1beta1_to_v1() {
        let v1beta1_rb = k8s_api::rbac::v1beta1::RoleBinding {
            metadata: ObjectMeta {
                name: "test-binding".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            subjects: vec![k8s_api::rbac::v1beta1::Subject {
                kind: "ServiceAccount".to_string(),
                name: "my-sa".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            }],
            role_ref: k8s_api::rbac::v1beta1::RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "Role".to_string(),
                name: "test-role".to_string(),
            },
            ..Default::default()
        };

        let v1_rb: k8s_api::rbac::v1::RoleBinding = v1beta1_rb.convert_to().unwrap();

        assert_eq!(v1_rb.metadata.name, "test-binding");
        assert_eq!(v1_rb.subjects.len(), 1);
        assert_eq!(v1_rb.subjects[0].kind, "ServiceAccount");
        assert_eq!(v1_rb.role_ref.kind, "Role");
    }

    #[test]
    fn test_cluster_role_binding_v1beta1_to_v1() {
        let v1beta1_crb = k8s_api::rbac::v1beta1::ClusterRoleBinding {
            metadata: ObjectMeta {
                name: "cluster-admin-binding".to_string(),
                ..Default::default()
            },
            subjects: vec![
                k8s_api::rbac::v1beta1::Subject {
                    kind: "User".to_string(),
                    name: "admin".to_string(),
                    api_group: "rbac.authorization.k8s.io".to_string(),
                    ..Default::default()
                },
                k8s_api::rbac::v1beta1::Subject {
                    kind: "Group".to_string(),
                    name: "admins".to_string(),
                    api_group: "rbac.authorization.k8s.io".to_string(),
                    ..Default::default()
                },
            ],
            role_ref: k8s_api::rbac::v1beta1::RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "ClusterRole".to_string(),
                name: "cluster-admin".to_string(),
            },
            ..Default::default()
        };

        let v1_crb: k8s_api::rbac::v1::ClusterRoleBinding = v1beta1_crb.convert_to().unwrap();

        assert_eq!(v1_crb.metadata.name, "cluster-admin-binding");
        assert_eq!(v1_crb.subjects.len(), 2);
        assert_eq!(v1_crb.subjects[0].kind, "User");
        assert_eq!(v1_crb.subjects[1].kind, "Group");
        assert_eq!(v1_crb.role_ref.kind, "ClusterRole");
    }

    #[test]
    fn test_roundtrip_role() {
        let original = k8s_api::rbac::v1beta1::Role {
            metadata: ObjectMeta {
                name: "roundtrip-role".to_string(),
                namespace: "test-ns".to_string(),
                ..Default::default()
            },
            rules: vec![
                k8s_api::rbac::v1beta1::PolicyRule {
                    verbs: vec!["get".to_string(), "list".to_string(), "watch".to_string()],
                    api_groups: vec!["".to_string()],
                    resources: vec!["pods".to_string(), "services".to_string()],
                    ..Default::default()
                },
                k8s_api::rbac::v1beta1::PolicyRule {
                    verbs: vec!["create".to_string(), "delete".to_string()],
                    api_groups: vec!["apps".to_string()],
                    resources: vec!["deployments".to_string()],
                    resource_names: vec!["my-deployment".to_string()],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let v1: k8s_api::rbac::v1::Role = original.convert_to().unwrap();
        let roundtrip = k8s_api::rbac::v1beta1::Role::convert_from(&v1).unwrap();

        assert_eq!(original.metadata.name, roundtrip.metadata.name);
        assert_eq!(original.metadata.namespace, roundtrip.metadata.namespace);
        assert_eq!(original.rules.len(), roundtrip.rules.len());
        assert_eq!(original.rules[0].verbs, roundtrip.rules[0].verbs);
        assert_eq!(original.rules[1].resource_names, roundtrip.rules[1].resource_names);
    }

    #[test]
    fn test_roundtrip_cluster_role_binding() {
        let original = k8s_api::rbac::v1beta1::ClusterRoleBinding {
            metadata: ObjectMeta {
                name: "roundtrip-crb".to_string(),
                ..Default::default()
            },
            subjects: vec![k8s_api::rbac::v1beta1::Subject {
                kind: "ServiceAccount".to_string(),
                name: "default".to_string(),
                namespace: "kube-system".to_string(),
                ..Default::default()
            }],
            role_ref: k8s_api::rbac::v1beta1::RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "ClusterRole".to_string(),
                name: "system:node".to_string(),
            },
            ..Default::default()
        };

        let v1: k8s_api::rbac::v1::ClusterRoleBinding = original.convert_to().unwrap();
        let roundtrip = k8s_api::rbac::v1beta1::ClusterRoleBinding::convert_from(&v1).unwrap();

        assert_eq!(original.metadata.name, roundtrip.metadata.name);
        assert_eq!(original.subjects.len(), roundtrip.subjects.len());
        assert_eq!(original.subjects[0].namespace, roundtrip.subjects[0].namespace);
        assert_eq!(original.role_ref.name, roundtrip.role_ref.name);
    }

    #[test]
    fn test_policy_rule_with_non_resource_urls() {
        let v1beta1_role = k8s_api::rbac::v1beta1::Role {
            metadata: ObjectMeta {
                name: "api-access".to_string(),
                ..Default::default()
            },
            rules: vec![k8s_api::rbac::v1beta1::PolicyRule {
                verbs: vec!["get".to_string()],
                non_resource_urls: vec!["/healthz".to_string(), "/readyz".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_role: k8s_api::rbac::v1::Role = v1beta1_role.convert_to().unwrap();

        assert_eq!(v1_role.rules[0].non_resource_urls.len(), 2);
        assert!(v1_role.rules[0].non_resource_urls.contains(&"/healthz".to_string()));
    }
}
