//! RBAC API validation
//!
//! This module provides validation for RBAC API types including:
//! - Role
//! - ClusterRole
//! - RoleBinding
//! - ClusterRoleBinding

use crate::common::{validate_dns_label, validate_dns_subdomain_name, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::rbac::v1::{
    ClusterRole, ClusterRoleBinding, PolicyRule, Role, RoleBinding, RoleRef, Subject,
};

/// Valid subject kinds
const VALID_SUBJECT_KINDS: &[&str] = &["User", "Group", "ServiceAccount"];

/// Valid role ref kinds for RoleBinding
const VALID_ROLE_REF_KINDS: &[&str] = &["Role", "ClusterRole"];

/// Valid role ref kinds for ClusterRoleBinding
const VALID_CLUSTER_ROLE_REF_KINDS: &[&str] = &["ClusterRole"];

/// RBAC API group
const RBAC_API_GROUP: &str = "rbac.authorization.k8s.io";

/// Valid verbs for RBAC rules
const VALID_VERBS: &[&str] = &[
    "get", "list", "watch", "create", "update", "patch", "delete", "deletecollection",
    "impersonate", "bind", "escalate", "*",
];

// =============================================================================
// Role Validation
// =============================================================================

/// Validates a Role resource.
pub fn validate_role(role: &Role) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&role.metadata, "metadata", true));

    for (i, rule) in role.rules.iter().enumerate() {
        errors.extend(validate_policy_rule(rule, &format!("rules[{}]", i)));
    }

    errors
}

// =============================================================================
// ClusterRole Validation
// =============================================================================

/// Validates a ClusterRole resource.
pub fn validate_cluster_role(cluster_role: &ClusterRole) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&cluster_role.metadata, "metadata", true));

    for (i, rule) in cluster_role.rules.iter().enumerate() {
        errors.extend(validate_policy_rule(rule, &format!("rules[{}]", i)));
    }

    errors
}

// =============================================================================
// RoleBinding Validation
// =============================================================================

/// Validates a RoleBinding resource.
pub fn validate_role_binding(binding: &RoleBinding) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&binding.metadata, "metadata", true));

    // Validate subjects
    for (i, subject) in binding.subjects.iter().enumerate() {
        errors.extend(validate_subject(subject, &format!("subjects[{}]", i)));
    }

    // Validate roleRef
    errors.extend(validate_role_ref(
        &binding.role_ref,
        "roleRef",
        VALID_ROLE_REF_KINDS,
    ));

    errors
}

// =============================================================================
// ClusterRoleBinding Validation
// =============================================================================

/// Validates a ClusterRoleBinding resource.
pub fn validate_cluster_role_binding(binding: &ClusterRoleBinding) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&binding.metadata, "metadata", true));

    // Validate subjects
    for (i, subject) in binding.subjects.iter().enumerate() {
        errors.extend(validate_subject(subject, &format!("subjects[{}]", i)));
    }

    // Validate roleRef - ClusterRoleBinding can only reference ClusterRole
    errors.extend(validate_role_ref(
        &binding.role_ref,
        "roleRef",
        VALID_CLUSTER_ROLE_REF_KINDS,
    ));

    errors
}

// =============================================================================
// PolicyRule Validation
// =============================================================================

/// Validates a PolicyRule.
pub fn validate_policy_rule(rule: &PolicyRule, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Verbs is required
    if rule.verbs.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.verbs", field),
            "verbs is required",
        ));
    } else {
        // Validate each verb
        for (i, verb) in rule.verbs.iter().enumerate() {
            if verb.is_empty() {
                errors.push(ValidationError::invalid(
                    format!("{}.verbs[{}]", field, i),
                    "verb cannot be empty",
                ));
            } else if !VALID_VERBS.contains(&verb.as_str()) {
                // Allow any verb but warn about common ones
                // In Kubernetes, custom verbs are allowed
            }
        }
    }

    // Must have either resources+apiGroups or nonResourceURLs
    let has_resources = !rule.resources.is_empty();
    let has_non_resource_urls = !rule.non_resource_urls.is_empty();

    if !has_resources && !has_non_resource_urls {
        errors.push(ValidationError::required(
            field,
            "either resources or nonResourceURLs must be specified",
        ));
    }

    if has_resources && has_non_resource_urls {
        errors.push(ValidationError::invalid(
            field,
            "resources and nonResourceURLs cannot both be specified in the same rule",
        ));
    }

    // If resources are specified, apiGroups should also be specified
    if has_resources && rule.api_groups.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.apiGroups", field),
            "apiGroups is required when resources is specified",
        ));
    }

    // resourceNames can only be used with resources
    if !rule.resource_names.is_empty() && !has_resources {
        errors.push(ValidationError::forbidden(
            format!("{}.resourceNames", field),
            "resourceNames can only be used with resources",
        ));
    }

    // Validate nonResourceURLs format
    for (i, url) in rule.non_resource_urls.iter().enumerate() {
        if url.is_empty() {
            errors.push(ValidationError::invalid(
                format!("{}.nonResourceURLs[{}]", field, i),
                "nonResourceURL cannot be empty",
            ));
        } else if !url.starts_with('/') {
            errors.push(ValidationError::invalid(
                format!("{}.nonResourceURLs[{}]", field, i),
                "nonResourceURL must start with '/'",
            ));
        }
    }

    errors
}

// =============================================================================
// Subject Validation
// =============================================================================

/// Validates a Subject.
pub fn validate_subject(subject: &Subject, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Kind is required
    if subject.kind.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.kind", field),
            "kind is required",
        ));
    } else if !VALID_SUBJECT_KINDS.contains(&subject.kind.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.kind", field),
            &subject.kind,
            VALID_SUBJECT_KINDS,
        ));
    }

    // Name is required
    if subject.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "name is required",
        ));
    }

    // Validate based on kind
    match subject.kind.as_str() {
        "ServiceAccount" => {
            // ServiceAccount requires namespace
            if subject.namespace.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.namespace", field),
                    "namespace is required for ServiceAccount",
                ));
            } else {
                errors.extend(validate_dns_label(
                    &subject.namespace,
                    &format!("{}.namespace", field),
                ));
            }

            // ServiceAccount name must be valid DNS subdomain
            if !subject.name.is_empty() {
                errors.extend(validate_dns_subdomain_name(
                    &subject.name,
                    &format!("{}.name", field),
                ));
            }

            // apiGroup should be empty for ServiceAccount (core API group)
            if !subject.api_group.is_empty() && subject.api_group != "" {
                errors.push(ValidationError::invalid(
                    format!("{}.apiGroup", field),
                    "apiGroup should be empty for ServiceAccount",
                ));
            }
        }
        "User" | "Group" => {
            // User and Group should not have namespace
            if !subject.namespace.is_empty() {
                errors.push(ValidationError::forbidden(
                    format!("{}.namespace", field),
                    "namespace should not be set for User or Group",
                ));
            }

            // apiGroup must be rbac.authorization.k8s.io
            if subject.api_group.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.apiGroup", field),
                    "apiGroup is required for User or Group",
                ));
            } else if subject.api_group != RBAC_API_GROUP {
                errors.push(ValidationError::invalid(
                    format!("{}.apiGroup", field),
                    format!("apiGroup must be '{}' for User or Group", RBAC_API_GROUP),
                ));
            }
        }
        _ => {}
    }

    errors
}

// =============================================================================
// RoleRef Validation
// =============================================================================

/// Validates a RoleRef.
pub fn validate_role_ref(role_ref: &RoleRef, field: &str, valid_kinds: &[&str]) -> ValidationResult {
    let mut errors = Vec::new();

    // apiGroup is required
    if role_ref.api_group.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.apiGroup", field),
            "apiGroup is required",
        ));
    } else if role_ref.api_group != RBAC_API_GROUP {
        errors.push(ValidationError::invalid(
            format!("{}.apiGroup", field),
            format!("apiGroup must be '{}'", RBAC_API_GROUP),
        ));
    }

    // Kind is required
    if role_ref.kind.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.kind", field),
            "kind is required",
        ));
    } else if !valid_kinds.contains(&role_ref.kind.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.kind", field),
            &role_ref.kind,
            valid_kinds,
        ));
    }

    // Name is required
    if role_ref.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "name is required",
        ));
    } else {
        errors.extend(validate_dns_subdomain_name(
            &role_ref.name,
            &format!("{}.name", field),
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_role_valid() {
        let role = Role {
            metadata: ObjectMeta::named("test-role"),
            rules: vec![PolicyRule {
                verbs: vec!["get".to_string(), "list".to_string()],
                api_groups: vec!["".to_string()],
                resources: vec!["pods".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validate_role(&role);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_role_empty_rules() {
        let role = Role {
            metadata: ObjectMeta::named("test-role"),
            rules: Vec::new(),
            ..Default::default()
        };

        let errors = validate_role(&role);
        assert!(errors.is_empty(), "Empty rules are valid");
    }

    #[test]
    fn test_validate_policy_rule_missing_verbs() {
        let rule = PolicyRule {
            verbs: Vec::new(),
            api_groups: vec!["".to_string()],
            resources: vec!["pods".to_string()],
            ..Default::default()
        };

        let errors = validate_policy_rule(&rule, "rule");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("verbs")));
    }

    #[test]
    fn test_validate_policy_rule_non_resource_urls() {
        let rule = PolicyRule {
            verbs: vec!["get".to_string()],
            non_resource_urls: vec!["/healthz".to_string()],
            ..Default::default()
        };

        let errors = validate_policy_rule(&rule, "rule");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_policy_rule_both_resources_and_non_resource_urls() {
        let rule = PolicyRule {
            verbs: vec!["get".to_string()],
            api_groups: vec!["".to_string()],
            resources: vec!["pods".to_string()],
            resource_names: Vec::new(),
            non_resource_urls: vec!["/healthz".to_string()],
        };

        let errors = validate_policy_rule(&rule, "rule");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("cannot both")));
    }

    #[test]
    fn test_validate_policy_rule_invalid_non_resource_url() {
        let rule = PolicyRule {
            verbs: vec!["get".to_string()],
            non_resource_urls: vec!["healthz".to_string()], // Missing leading /
            ..Default::default()
        };

        let errors = validate_policy_rule(&rule, "rule");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("nonResourceURLs")));
    }

    #[test]
    fn test_validate_role_binding_valid() {
        let binding = RoleBinding {
            metadata: ObjectMeta::named("test-binding"),
            subjects: vec![Subject {
                kind: "ServiceAccount".to_string(),
                name: "my-service-account".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            }],
            role_ref: RoleRef {
                api_group: RBAC_API_GROUP.to_string(),
                kind: "Role".to_string(),
                name: "test-role".to_string(),
            },
            ..Default::default()
        };

        let errors = validate_role_binding(&binding);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_role_binding_with_cluster_role() {
        let binding = RoleBinding {
            metadata: ObjectMeta::named("test-binding"),
            subjects: vec![Subject {
                kind: "User".to_string(),
                api_group: RBAC_API_GROUP.to_string(),
                name: "jane".to_string(),
                ..Default::default()
            }],
            role_ref: RoleRef {
                api_group: RBAC_API_GROUP.to_string(),
                kind: "ClusterRole".to_string(),
                name: "cluster-admin".to_string(),
            },
            ..Default::default()
        };

        let errors = validate_role_binding(&binding);
        assert!(errors.is_empty(), "RoleBinding can reference ClusterRole: {:?}", errors);
    }

    #[test]
    fn test_validate_cluster_role_binding_valid() {
        let binding = ClusterRoleBinding {
            metadata: ObjectMeta::named("test-binding"),
            subjects: vec![Subject {
                kind: "Group".to_string(),
                api_group: RBAC_API_GROUP.to_string(),
                name: "admin-group".to_string(),
                ..Default::default()
            }],
            role_ref: RoleRef {
                api_group: RBAC_API_GROUP.to_string(),
                kind: "ClusterRole".to_string(),
                name: "cluster-admin".to_string(),
            },
            ..Default::default()
        };

        let errors = validate_cluster_role_binding(&binding);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_cluster_role_binding_cannot_reference_role() {
        let binding = ClusterRoleBinding {
            metadata: ObjectMeta::named("test-binding"),
            subjects: vec![Subject {
                kind: "User".to_string(),
                api_group: RBAC_API_GROUP.to_string(),
                name: "jane".to_string(),
                ..Default::default()
            }],
            role_ref: RoleRef {
                api_group: RBAC_API_GROUP.to_string(),
                kind: "Role".to_string(), // Invalid - ClusterRoleBinding cannot reference Role
                name: "test-role".to_string(),
            },
            ..Default::default()
        };

        let errors = validate_cluster_role_binding(&binding);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("roleRef.kind")));
    }

    #[test]
    fn test_validate_subject_service_account() {
        let subject = Subject {
            kind: "ServiceAccount".to_string(),
            name: "my-sa".to_string(),
            namespace: "default".to_string(),
            ..Default::default()
        };

        let errors = validate_subject(&subject, "subject");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_subject_service_account_missing_namespace() {
        let subject = Subject {
            kind: "ServiceAccount".to_string(),
            name: "my-sa".to_string(),
            namespace: "".to_string(),
            ..Default::default()
        };

        let errors = validate_subject(&subject, "subject");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("namespace")));
    }

    #[test]
    fn test_validate_subject_user() {
        let subject = Subject {
            kind: "User".to_string(),
            api_group: RBAC_API_GROUP.to_string(),
            name: "jane@example.com".to_string(),
            ..Default::default()
        };

        let errors = validate_subject(&subject, "subject");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_subject_user_with_namespace() {
        let subject = Subject {
            kind: "User".to_string(),
            api_group: RBAC_API_GROUP.to_string(),
            name: "jane".to_string(),
            namespace: "default".to_string(), // Invalid for User
        };

        let errors = validate_subject(&subject, "subject");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("should not be set")));
    }

    #[test]
    fn test_validate_subject_invalid_kind() {
        let subject = Subject {
            kind: "Invalid".to_string(),
            name: "test".to_string(),
            ..Default::default()
        };

        let errors = validate_subject(&subject, "subject");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("kind")));
    }

    #[test]
    fn test_validate_role_ref_valid() {
        let role_ref = RoleRef {
            api_group: RBAC_API_GROUP.to_string(),
            kind: "Role".to_string(),
            name: "my-role".to_string(),
        };

        let errors = validate_role_ref(&role_ref, "roleRef", VALID_ROLE_REF_KINDS);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_role_ref_invalid_api_group() {
        let role_ref = RoleRef {
            api_group: "invalid".to_string(),
            kind: "Role".to_string(),
            name: "my-role".to_string(),
        };

        let errors = validate_role_ref(&role_ref, "roleRef", VALID_ROLE_REF_KINDS);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("apiGroup")));
    }

    #[test]
    fn test_validate_role_ref_missing_name() {
        let role_ref = RoleRef {
            api_group: RBAC_API_GROUP.to_string(),
            kind: "Role".to_string(),
            name: "".to_string(),
        };

        let errors = validate_role_ref(&role_ref, "roleRef", VALID_ROLE_REF_KINDS);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("name")));
    }

    #[test]
    fn test_validate_cluster_role_with_aggregation() {
        let cluster_role = ClusterRole {
            metadata: ObjectMeta::named("aggregate-role"),
            rules: Vec::new(),
            aggregation_rule: Some(k8s_api::rbac::v1::AggregationRule {
                cluster_role_selectors: vec![
                    k8s_apimachinery::apis::meta::v1::LabelSelector {
                        match_labels: {
                            let mut labels = std::collections::BTreeMap::new();
                            labels.insert("aggregate".to_string(), "true".to_string());
                            labels
                        },
                        ..Default::default()
                    },
                ],
            }),
            ..Default::default()
        };

        let errors = validate_cluster_role(&cluster_role);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }
}
