//! Admission registration validation
//!
//! This module provides validation for admissionregistration API types.

use crate::common::{validate_dns_label, validate_dns_subdomain_name, validate_label_key, validate_object_meta};
use crate::{ValidationError, ValidationResult};

const VALID_FAILURE_POLICIES: &[&str] = &["Ignore", "Fail"];
const VALID_MATCH_POLICIES: &[&str] = &["Exact", "Equivalent"];
const VALID_SIDE_EFFECTS: &[&str] = &["None", "NoneOnDryRun", "Some", "Unknown"];
const VALID_REINVOCATION_POLICIES: &[&str] = &["Never", "IfNeeded"];
const VALID_OPERATIONS: &[&str] = &["CREATE", "UPDATE", "DELETE", "CONNECT", "*"];
const VALID_SCOPES: &[&str] = &["Cluster", "Namespaced", "*"];
const VALID_VALIDATION_ACTIONS: &[&str] = &["Deny", "Warn", "Audit"];
const VALID_PATCH_TYPES: &[&str] = &["ApplyConfiguration", "JSONPatch"];
const VALID_PARAMETER_NOT_FOUND_ACTIONS: &[&str] = &["Deny", "Allow"];

fn validate_required_value(value: &str, field: &str, message: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, message)]
    } else {
        Vec::new()
    }
}

fn validate_required_allowed(value: &str, field: &str, allowed: &[&str]) -> ValidationResult {
    if value.is_empty() {
        return vec![ValidationError::required(field, "value is required")];
    }
    if !allowed.contains(&value) {
        return vec![ValidationError::not_supported(field, value, allowed)];
    }
    Vec::new()
}

fn validate_optional_allowed(
    value: &Option<String>,
    field: &str,
    allowed: &[&str],
) -> ValidationResult {
    if let Some(value) = value {
        return validate_required_allowed(value, field, allowed);
    }
    Vec::new()
}

fn validate_timeout_seconds(value: i32, field: &str) -> ValidationResult {
    if value < 1 || value > 30 {
        return vec![ValidationError::out_of_range(
            field,
            1,
            30,
            value as i64,
        )];
    }
    Vec::new()
}

fn validate_admission_review_versions(values: &[String], field: &str) -> ValidationResult {
    let mut errors = Vec::new();
    if values.is_empty() {
        errors.push(ValidationError::required(
            field,
            "admissionReviewVersions is required",
        ));
    }
    for (i, value) in values.iter().enumerate() {
        if value.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}[{}]", field, i),
                "version is required",
            ));
        }
    }
    errors
}

fn validate_operations(values: &[String], field: &str) -> ValidationResult {
    let mut errors = Vec::new();
    if values.is_empty() {
        errors.push(ValidationError::required(
            field,
            "operations is required",
        ));
    }
    for (i, value) in values.iter().enumerate() {
        if value.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}[{}]", field, i),
                "operation is required",
            ));
            continue;
        }
        if !VALID_OPERATIONS.contains(&value.as_str()) {
            errors.push(ValidationError::not_supported(
                &format!("{}[{}]", field, i),
                value,
                VALID_OPERATIONS,
            ));
        }
    }
    errors
}

fn validate_rule_parts(values: &[String], field: &str, label: &str) -> ValidationResult {
    let mut errors = Vec::new();
    if values.is_empty() {
        errors.push(ValidationError::required(field, label));
    }
    for (i, value) in values.iter().enumerate() {
        if value.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}[{}]", field, i),
                "value is required",
            ));
        }
    }
    errors
}

fn validate_match_condition_fields(name: &str, expression: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();
    errors.extend(validate_required_value(
        name,
        &format!("{}.name", field),
        "name is required",
    ));
    if !name.is_empty() {
        errors.extend(validate_dns_label(name, &format!("{}.name", field)));
    }
    errors.extend(validate_required_value(
        expression,
        &format!("{}.expression", field),
        "expression is required",
    ));
    errors
}

pub mod v1 {
    use super::*;
    use k8s_api::admissionregistration::v1 as api;

    pub fn validate_mutating_webhook_configuration(
        config: &api::MutatingWebhookConfiguration,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&config.metadata, "metadata", true));

        for (i, webhook) in config.webhooks.iter().enumerate() {
            errors.extend(validate_mutating_webhook(
                webhook,
                &format!("webhooks[{}]", i),
            ));
        }

        errors
    }

    pub fn validate_validating_webhook_configuration(
        config: &api::ValidatingWebhookConfiguration,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&config.metadata, "metadata", true));

        for (i, webhook) in config.webhooks.iter().enumerate() {
            errors.extend(validate_validating_webhook(
                webhook,
                &format!("webhooks[{}]", i),
            ));
        }

        errors
    }

    fn validate_mutating_webhook(webhook: &api::MutatingWebhook, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &webhook.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !webhook.name.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                &webhook.name,
                &format!("{}.name", field),
            ));
        }

        errors.extend(validate_webhook_client_config(
            &webhook.client_config,
            &format!("{}.clientConfig", field),
        ));

        for (i, rule) in webhook.rules.iter().enumerate() {
            errors.extend(validate_rule_with_operations(
                rule,
                &format!("{}.rules[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &webhook.failure_policy,
            &format!("{}.failurePolicy", field),
            VALID_FAILURE_POLICIES,
        ));
        errors.extend(validate_optional_allowed(
            &webhook.match_policy,
            &format!("{}.matchPolicy", field),
            VALID_MATCH_POLICIES,
        ));

        errors.extend(validate_required_allowed(
            &webhook.side_effects,
            &format!("{}.sideEffects", field),
            VALID_SIDE_EFFECTS,
        ));

        if let Some(timeout) = webhook.timeout_seconds {
            errors.extend(validate_timeout_seconds(
                timeout,
                &format!("{}.timeoutSeconds", field),
            ));
        }

        errors.extend(validate_admission_review_versions(
            &webhook.admission_review_versions,
            &format!("{}.admissionReviewVersions", field),
        ));

        errors.extend(validate_optional_allowed(
            &webhook.reinvocation_policy,
            &format!("{}.reinvocationPolicy", field),
            VALID_REINVOCATION_POLICIES,
        ));

        for (i, condition) in webhook.match_conditions.iter().enumerate() {
            errors.extend(validate_match_condition_fields(
                &condition.name,
                &condition.expression,
                &format!("{}.matchConditions[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_validating_webhook(
        webhook: &api::ValidatingWebhook,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &webhook.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !webhook.name.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                &webhook.name,
                &format!("{}.name", field),
            ));
        }

        errors.extend(validate_webhook_client_config(
            &webhook.client_config,
            &format!("{}.clientConfig", field),
        ));

        for (i, rule) in webhook.rules.iter().enumerate() {
            errors.extend(validate_rule_with_operations(
                rule,
                &format!("{}.rules[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &webhook.failure_policy,
            &format!("{}.failurePolicy", field),
            VALID_FAILURE_POLICIES,
        ));
        errors.extend(validate_optional_allowed(
            &webhook.match_policy,
            &format!("{}.matchPolicy", field),
            VALID_MATCH_POLICIES,
        ));

        errors.extend(validate_required_allowed(
            &webhook.side_effects,
            &format!("{}.sideEffects", field),
            VALID_SIDE_EFFECTS,
        ));

        if let Some(timeout) = webhook.timeout_seconds {
            errors.extend(validate_timeout_seconds(
                timeout,
                &format!("{}.timeoutSeconds", field),
            ));
        }

        errors.extend(validate_admission_review_versions(
            &webhook.admission_review_versions,
            &format!("{}.admissionReviewVersions", field),
        ));

        for (i, condition) in webhook.match_conditions.iter().enumerate() {
            errors.extend(validate_match_condition_fields(
                &condition.name,
                &condition.expression,
                &format!("{}.matchConditions[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_webhook_client_config(
        config: &api::WebhookClientConfig,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        let has_url = config.url.is_some();
        let has_service = config.service.is_some();
        if has_url == has_service {
            errors.push(ValidationError::invalid(
                field,
                "exactly one of url or service is required",
            ));
        }

        if let Some(url) = &config.url {
            if !url.starts_with("https://") {
                errors.push(ValidationError::invalid(
                    &format!("{}.url", field),
                    "url must start with https://",
                ));
            }
        }

        if let Some(service) = &config.service {
            errors.extend(validate_service_reference(
                service,
                &format!("{}.service", field),
            ));
        }

        errors
    }

    fn validate_service_reference(
        service: &api::ServiceReference,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &service.namespace,
            &format!("{}.namespace", field),
            "namespace is required",
        ));
        if !service.namespace.is_empty() {
            errors.extend(validate_dns_label(
                &service.namespace,
                &format!("{}.namespace", field),
            ));
        }

        errors.extend(validate_required_value(
            &service.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !service.name.is_empty() {
            errors.extend(validate_dns_label(
                &service.name,
                &format!("{}.name", field),
            ));
        }

        if let Some(port) = service.port {
            if port < 1 || port > 65535 {
                errors.push(ValidationError::out_of_range(
                    &format!("{}.port", field),
                    1,
                    65535,
                    port as i64,
                ));
            }
        }

        errors
    }

    fn validate_rule_with_operations(rule: &api::RuleWithOperations, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_operations(
            &rule.operations,
            &format!("{}.operations", field),
        ));
        errors.extend(validate_rule_parts(
            &rule.api_groups,
            &format!("{}.apiGroups", field),
            "apiGroups is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.api_versions,
            &format!("{}.apiVersions", field),
            "apiVersions is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.resources,
            &format!("{}.resources", field),
            "resources is required",
        ));

        if let Some(scope) = &rule.scope {
            errors.extend(validate_required_allowed(
                scope,
                &format!("{}.scope", field),
                VALID_SCOPES,
            ));
        }

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::admissionregistration::v1beta1 as api;

    pub fn validate_validating_admission_policy(
        policy: &api::ValidatingAdmissionPolicy,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&policy.metadata, "metadata", true));

        if let Some(spec) = &policy.spec {
            errors.extend(validate_validating_admission_policy_spec(
                spec,
                "spec",
            ));
        }

        errors
    }

    pub fn validate_validating_admission_policy_binding(
        binding: &api::ValidatingAdmissionPolicyBinding,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&binding.metadata, "metadata", true));

        if let Some(spec) = &binding.spec {
            errors.extend(validate_validating_admission_policy_binding_spec(
                spec,
                "spec",
            ));
        }

        errors
    }

    fn validate_validating_admission_policy_spec(
        spec: &api::ValidatingAdmissionPolicySpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(param_kind) = &spec.param_kind {
            errors.extend(validate_param_kind(param_kind, &format!("{}.paramKind", field)));
        }

        if let Some(match_constraints) = &spec.match_constraints {
            errors.extend(validate_match_resources(
                match_constraints,
                &format!("{}.matchConstraints", field),
            ));
        }

        for (i, validation) in spec.validations.iter().enumerate() {
            errors.extend(validate_validation(
                validation,
                &format!("{}.validations[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &spec.failure_policy,
            &format!("{}.failurePolicy", field),
            VALID_FAILURE_POLICIES,
        ));

        for (i, annotation) in spec.audit_annotations.iter().enumerate() {
            errors.extend(validate_audit_annotation(
                annotation,
                &format!("{}.auditAnnotations[{}]", field, i),
            ));
        }

        for (i, condition) in spec.match_conditions.iter().enumerate() {
            errors.extend(validate_match_condition_fields(
                &condition.name,
                &condition.expression,
                &format!("{}.matchConditions[{}]", field, i),
            ));
        }

        for (i, variable) in spec.variables.iter().enumerate() {
            errors.extend(validate_variable(
                variable,
                &format!("{}.variables[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_validating_admission_policy_binding_spec(
        spec: &api::ValidatingAdmissionPolicyBindingSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &spec.policy_name,
            &format!("{}.policyName", field),
            "policyName is required",
        ));

        if let Some(param_ref) = &spec.param_ref {
            errors.extend(validate_param_ref(param_ref, &format!("{}.paramRef", field)));
        }

        if let Some(match_resources) = &spec.match_resources {
            errors.extend(validate_match_resources(
                match_resources,
                &format!("{}.matchResources", field),
            ));
        }

        for (i, action) in spec.validation_actions.iter().enumerate() {
            if !VALID_VALIDATION_ACTIONS.contains(&action.as_str()) {
                errors.push(ValidationError::not_supported(
                    &format!("{}.validationActions[{}]", field, i),
                    action,
                    VALID_VALIDATION_ACTIONS,
                ));
            }
        }

        errors
    }

    fn validate_param_kind(kind: &api::ParamKind, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &kind.api_version,
            &format!("{}.apiVersion", field),
            "apiVersion is required",
        ));
        errors.extend(validate_required_value(
            &kind.kind,
            &format!("{}.kind", field),
            "kind is required",
        ));

        errors
    }

    fn validate_match_resources(resources: &api::MatchResources, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (i, rule) in resources.resource_rules.iter().enumerate() {
            errors.extend(validate_named_rule_with_operations(
                rule,
                &format!("{}.resourceRules[{}]", field, i),
            ));
        }

        for (i, rule) in resources.exclude_resource_rules.iter().enumerate() {
            errors.extend(validate_named_rule_with_operations(
                rule,
                &format!("{}.excludeResourceRules[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &resources.match_policy,
            &format!("{}.matchPolicy", field),
            VALID_MATCH_POLICIES,
        ));

        errors
    }

    fn validate_named_rule_with_operations(
        rule: &api::NamedRuleWithOperations,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_operations(
            &rule.operations,
            &format!("{}.operations", field),
        ));
        errors.extend(validate_rule_parts(
            &rule.api_groups,
            &format!("{}.apiGroups", field),
            "apiGroups is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.api_versions,
            &format!("{}.apiVersions", field),
            "apiVersions is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.resources,
            &format!("{}.resources", field),
            "resources is required",
        ));

        for (i, name) in rule.resource_names.iter().enumerate() {
            if name.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.resourceNames[{}]", field, i),
                    "resource name is required",
                ));
            } else {
                errors.extend(validate_dns_subdomain_name(
                    name,
                    &format!("{}.resourceNames[{}]", field, i),
                ));
            }
        }

        if let Some(scope) = &rule.scope {
            errors.extend(validate_required_allowed(
                scope,
                &format!("{}.scope", field),
                VALID_SCOPES,
            ));
        }

        errors
    }

    fn validate_validation(validation: &api::Validation, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &validation.expression,
            &format!("{}.expression", field),
            "expression is required",
        ));

        errors
    }

    fn validate_audit_annotation(
        annotation: &api::AuditAnnotation,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &annotation.key,
            &format!("{}.key", field),
            "key is required",
        ));
        if !annotation.key.is_empty() {
            errors.extend(validate_label_key(
                &annotation.key,
                &format!("{}.key", field),
            ));
        }

        errors.extend(validate_required_value(
            &annotation.value_expression,
            &format!("{}.valueExpression", field),
            "valueExpression is required",
        ));

        errors
    }

    fn validate_variable(variable: &api::Variable, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &variable.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !variable.name.is_empty() {
            errors.extend(validate_dns_label(
                &variable.name,
                &format!("{}.name", field),
            ));
        }

        errors.extend(validate_required_value(
            &variable.expression,
            &format!("{}.expression", field),
            "expression is required",
        ));

        errors
    }

    fn validate_param_ref(param_ref: &api::ParamRef, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &param_ref.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !param_ref.name.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                &param_ref.name,
                &format!("{}.name", field),
            ));
        }

        if !param_ref.namespace.is_empty() {
            errors.extend(validate_dns_label(
                &param_ref.namespace,
                &format!("{}.namespace", field),
            ));
        }

        errors.extend(validate_optional_allowed(
            &param_ref.parameter_not_found_action,
            &format!("{}.parameterNotFoundAction", field),
            VALID_PARAMETER_NOT_FOUND_ACTIONS,
        ));

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::admissionregistration::internal as api;

    pub fn validate_mutating_webhook_configuration(
        config: &api::MutatingWebhookConfiguration,
    ) -> ValidationResult {
        crate::internal::validate_with(
            config,
            "mutatingWebhookConfiguration",
            super::v1::validate_mutating_webhook_configuration,
        )
    }

    pub fn validate_validating_webhook_configuration(
        config: &api::ValidatingWebhookConfiguration,
    ) -> ValidationResult {
        crate::internal::validate_with(
            config,
            "validatingWebhookConfiguration",
            super::v1::validate_validating_webhook_configuration,
        )
    }

    pub fn validate_validating_admission_policy(
        policy: &api::ValidatingAdmissionPolicy,
    ) -> ValidationResult {
        crate::internal::validate_with(
            policy,
            "validatingAdmissionPolicy",
            super::v1beta1::validate_validating_admission_policy,
        )
    }

    pub fn validate_validating_admission_policy_binding(
        binding: &api::ValidatingAdmissionPolicyBinding,
    ) -> ValidationResult {
        crate::internal::validate_with(
            binding,
            "validatingAdmissionPolicyBinding",
            super::v1beta1::validate_validating_admission_policy_binding,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use k8s_api::admissionregistration::v1 as api;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    fn valid_webhook() -> api::ValidatingWebhook {
        api::ValidatingWebhook {
            name: "webhook.k8s.io".to_string(),
            client_config: api::WebhookClientConfig {
                url: Some("https://example.com".to_string()),
                ..Default::default()
            },
            rules: vec![api::RuleWithOperations {
                operations: vec!["CREATE".to_string()],
                api_groups: vec!["apps".to_string()],
                api_versions: vec!["v1".to_string()],
                resources: vec!["deployments".to_string()],
                ..Default::default()
            }],
            side_effects: "None".to_string(),
            admission_review_versions: vec!["v1".to_string()],
            ..Default::default()
        }
    }

    fn valid_config() -> api::ValidatingWebhookConfiguration {
        api::ValidatingWebhookConfiguration {
            metadata: ObjectMeta::named("config"),
            webhooks: vec![valid_webhook()],
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_validating_webhook_requires_admission_review_versions() {
        let mut config = valid_config();
        config.webhooks[0].admission_review_versions.clear();

        let errors = validation_v1::validate_validating_webhook_configuration(&config);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("admissionReviewVersions")));
    }

    #[test]
    fn test_validate_validating_webhook_invalid_operation() {
        let mut config = valid_config();
        config.webhooks[0].rules[0].operations = vec!["PATCH".to_string()];

        let errors = validation_v1::validate_validating_webhook_configuration(&config);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("operations")));
    }

    #[test]
    fn test_validate_validating_webhook_client_config_missing() {
        let mut config = valid_config();
        config.webhooks[0].client_config.url = None;

        let errors = validation_v1::validate_validating_webhook_configuration(&config);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("clientConfig")));
    }

    #[test]
    fn test_validate_validating_webhook_url_requires_https() {
        let mut config = valid_config();
        config.webhooks[0].client_config.url = Some("http://example.com".to_string());

        let errors = validation_v1::validate_validating_webhook_configuration(&config);
        assert!(errors.iter().any(|error| error.field.contains("url")));
    }

    #[test]
    fn test_validate_validating_webhook_valid() {
        let config = valid_config();
        let errors = validation_v1::validate_validating_webhook_configuration(&config);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}

pub mod v1alpha1 {
    use super::*;
    use k8s_api::admissionregistration::v1alpha1 as api;

    pub fn validate_mutating_admission_policy(
        policy: &api::MutatingAdmissionPolicy,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&policy.metadata, "metadata", true));

        if let Some(spec) = &policy.spec {
            errors.extend(validate_mutating_admission_policy_spec(spec, "spec"));
        }

        errors
    }

    pub fn validate_mutating_admission_policy_binding(
        binding: &api::MutatingAdmissionPolicyBinding,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&binding.metadata, "metadata", true));

        if let Some(spec) = &binding.spec {
            errors.extend(validate_mutating_admission_policy_binding_spec(
                spec,
                "spec",
            ));
        }

        errors
    }

    pub fn validate_validating_admission_policy(
        policy: &api::ValidatingAdmissionPolicy,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&policy.metadata, "metadata", true));

        if let Some(spec) = &policy.spec {
            errors.extend(validate_validating_admission_policy_spec(
                spec,
                "spec",
            ));
        }

        errors
    }

    pub fn validate_validating_admission_policy_binding(
        binding: &api::ValidatingAdmissionPolicyBinding,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&binding.metadata, "metadata", true));

        if let Some(spec) = &binding.spec {
            errors.extend(validate_validating_admission_policy_binding_spec(
                spec,
                "spec",
            ));
        }

        errors
    }

    fn validate_mutating_admission_policy_spec(
        spec: &api::MutatingAdmissionPolicySpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(param_kind) = &spec.param_kind {
            errors.extend(validate_param_kind(param_kind, &format!("{}.paramKind", field)));
        }

        if let Some(match_constraints) = &spec.match_constraints {
            errors.extend(validate_match_resources(
                match_constraints,
                &format!("{}.matchConstraints", field),
            ));
        }

        for (i, variable) in spec.variables.iter().enumerate() {
            errors.extend(validate_variable(
                variable,
                &format!("{}.variables[{}]", field, i),
            ));
        }

        for (i, mutation) in spec.mutations.iter().enumerate() {
            errors.extend(validate_mutation(
                mutation,
                &format!("{}.mutations[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &spec.failure_policy,
            &format!("{}.failurePolicy", field),
            VALID_FAILURE_POLICIES,
        ));

        for (i, condition) in spec.match_conditions.iter().enumerate() {
            errors.extend(validate_match_condition_fields(
                &condition.name,
                &condition.expression,
                &format!("{}.matchConditions[{}]", field, i),
            ));
        }

        if !spec.reinvocation_policy.is_empty() {
            errors.extend(validate_required_allowed(
                &spec.reinvocation_policy,
                &format!("{}.reinvocationPolicy", field),
                VALID_REINVOCATION_POLICIES,
            ));
        }

        errors
    }

    fn validate_mutating_admission_policy_binding_spec(
        spec: &api::MutatingAdmissionPolicyBindingSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &spec.policy_name,
            &format!("{}.policyName", field),
            "policyName is required",
        ));

        if let Some(param_ref) = &spec.param_ref {
            errors.extend(validate_param_ref(param_ref, &format!("{}.paramRef", field)));
        }

        if let Some(match_resources) = &spec.match_resources {
            errors.extend(validate_match_resources(
                match_resources,
                &format!("{}.matchResources", field),
            ));
        }

        errors
    }

    fn validate_validating_admission_policy_spec(
        spec: &api::ValidatingAdmissionPolicySpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(param_kind) = &spec.param_kind {
            errors.extend(validate_param_kind(param_kind, &format!("{}.paramKind", field)));
        }

        if let Some(match_constraints) = &spec.match_constraints {
            errors.extend(validate_match_resources(
                match_constraints,
                &format!("{}.matchConstraints", field),
            ));
        }

        for (i, validation) in spec.validations.iter().enumerate() {
            errors.extend(validate_validation(
                validation,
                &format!("{}.validations[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &spec.failure_policy,
            &format!("{}.failurePolicy", field),
            VALID_FAILURE_POLICIES,
        ));

        for (i, annotation) in spec.audit_annotations.iter().enumerate() {
            errors.extend(validate_audit_annotation(
                annotation,
                &format!("{}.auditAnnotations[{}]", field, i),
            ));
        }

        for (i, condition) in spec.match_conditions.iter().enumerate() {
            errors.extend(validate_match_condition_fields(
                &condition.name,
                &condition.expression,
                &format!("{}.matchConditions[{}]", field, i),
            ));
        }

        for (i, variable) in spec.variables.iter().enumerate() {
            errors.extend(validate_variable(
                variable,
                &format!("{}.variables[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_validating_admission_policy_binding_spec(
        spec: &api::ValidatingAdmissionPolicyBindingSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &spec.policy_name,
            &format!("{}.policyName", field),
            "policyName is required",
        ));

        if let Some(param_ref) = &spec.param_ref {
            errors.extend(validate_param_ref(param_ref, &format!("{}.paramRef", field)));
        }

        if let Some(match_resources) = &spec.match_resources {
            errors.extend(validate_match_resources(
                match_resources,
                &format!("{}.matchResources", field),
            ));
        }

        for (i, action) in spec.validation_actions.iter().enumerate() {
            if !VALID_VALIDATION_ACTIONS.contains(&action.as_str()) {
                errors.push(ValidationError::not_supported(
                    &format!("{}.validationActions[{}]", field, i),
                    action,
                    VALID_VALIDATION_ACTIONS,
                ));
            }
        }

        errors
    }

    fn validate_mutation(mutation: &api::Mutation, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_allowed(
            &mutation.patch_type,
            &format!("{}.patchType", field),
            VALID_PATCH_TYPES,
        ));

        let has_apply = mutation.apply_configuration.is_some();
        let has_json = mutation.json_patch.is_some();
        if has_apply == has_json {
            errors.push(ValidationError::invalid(
                field,
                "exactly one of applyConfiguration or jsonPatch is required",
            ));
        }

        if let Some(apply) = &mutation.apply_configuration {
            errors.extend(validate_required_value(
                &apply.expression,
                &format!("{}.applyConfiguration.expression", field),
                "expression is required",
            ));
        }

        if let Some(json_patch) = &mutation.json_patch {
            errors.extend(validate_required_value(
                &json_patch.expression,
                &format!("{}.jsonPatch.expression", field),
                "expression is required",
            ));
        }

        errors
    }

    fn validate_param_kind(kind: &api::ParamKind, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &kind.api_version,
            &format!("{}.apiVersion", field),
            "apiVersion is required",
        ));
        errors.extend(validate_required_value(
            &kind.kind,
            &format!("{}.kind", field),
            "kind is required",
        ));

        errors
    }

    fn validate_match_resources(resources: &api::MatchResources, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (i, rule) in resources.resource_rules.iter().enumerate() {
            errors.extend(validate_named_rule_with_operations(
                rule,
                &format!("{}.resourceRules[{}]", field, i),
            ));
        }

        for (i, rule) in resources.exclude_resource_rules.iter().enumerate() {
            errors.extend(validate_named_rule_with_operations(
                rule,
                &format!("{}.excludeResourceRules[{}]", field, i),
            ));
        }

        errors.extend(validate_optional_allowed(
            &resources.match_policy,
            &format!("{}.matchPolicy", field),
            VALID_MATCH_POLICIES,
        ));

        errors
    }

    fn validate_named_rule_with_operations(
        rule: &api::NamedRuleWithOperations,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_operations(
            &rule.operations,
            &format!("{}.operations", field),
        ));
        errors.extend(validate_rule_parts(
            &rule.api_groups,
            &format!("{}.apiGroups", field),
            "apiGroups is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.api_versions,
            &format!("{}.apiVersions", field),
            "apiVersions is required",
        ));
        errors.extend(validate_rule_parts(
            &rule.resources,
            &format!("{}.resources", field),
            "resources is required",
        ));

        for (i, name) in rule.resource_names.iter().enumerate() {
            if name.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.resourceNames[{}]", field, i),
                    "resource name is required",
                ));
            } else {
                errors.extend(validate_dns_subdomain_name(
                    name,
                    &format!("{}.resourceNames[{}]", field, i),
                ));
            }
        }

        if let Some(scope) = &rule.scope {
            errors.extend(validate_required_allowed(
                scope,
                &format!("{}.scope", field),
                VALID_SCOPES,
            ));
        }

        errors
    }

    fn validate_validation(validation: &api::Validation, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &validation.expression,
            &format!("{}.expression", field),
            "expression is required",
        ));

        errors
    }

    fn validate_audit_annotation(
        annotation: &api::AuditAnnotation,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &annotation.key,
            &format!("{}.key", field),
            "key is required",
        ));
        if !annotation.key.is_empty() {
            errors.extend(validate_label_key(
                &annotation.key,
                &format!("{}.key", field),
            ));
        }

        errors.extend(validate_required_value(
            &annotation.value_expression,
            &format!("{}.valueExpression", field),
            "valueExpression is required",
        ));

        errors
    }

    fn validate_variable(variable: &api::Variable, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &variable.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !variable.name.is_empty() {
            errors.extend(validate_dns_label(
                &variable.name,
                &format!("{}.name", field),
            ));
        }

        errors.extend(validate_required_value(
            &variable.expression,
            &format!("{}.expression", field),
            "expression is required",
        ));

        errors
    }

    fn validate_param_ref(param_ref: &api::ParamRef, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_value(
            &param_ref.name,
            &format!("{}.name", field),
            "name is required",
        ));
        if !param_ref.name.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                &param_ref.name,
                &format!("{}.name", field),
            ));
        }

        if !param_ref.namespace.is_empty() {
            errors.extend(validate_dns_label(
                &param_ref.namespace,
                &format!("{}.namespace", field),
            ));
        }

        errors.extend(validate_optional_allowed(
            &param_ref.parameter_not_found_action,
            &format!("{}.parameterNotFoundAction", field),
            VALID_PARAMETER_NOT_FOUND_ACTIONS,
        ));

        errors
    }
}
