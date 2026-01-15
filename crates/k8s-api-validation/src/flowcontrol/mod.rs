//! Flow control API validation

use crate::common::{validate_dns_label, validate_dns_subdomain_name, validate_object_meta};
use crate::{ValidationError, ValidationResult};

const MAX_MATCHING_PRECEDENCE: i32 = 10000;
const PRIORITY_LEVEL_NAME_EXEMPT: &str = "exempt";

const VALID_DISTINGUISHER_METHODS: &[&str] = &["ByUser", "ByNamespace"];
const VALID_PRIORITY_LEVEL_TYPES: &[&str] = &["Exempt", "Limited"];
const VALID_LIMIT_RESPONSE_TYPES: &[&str] = &["Queue", "Reject"];
const VALID_SUBJECT_KINDS: &[&str] = &["User", "Group", "ServiceAccount"];
const SUPPORTED_VERBS: &[&str] = &[
    "get",
    "list",
    "watch",
    "create",
    "update",
    "patch",
    "delete",
    "deletecollection",
    "proxy",
];

fn has_wildcard(values: &[String]) -> bool {
    values.iter().any(|value| value == "*")
}

fn validate_matching_precedence(value: i32, field: &str, flow_schema_name: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if value <= 0 {
        errors.push(ValidationError::invalid(field, "must be a positive value"));
    }
    if value > MAX_MATCHING_PRECEDENCE {
        errors.push(ValidationError::invalid(
            field,
            format!("must not be greater than {}", MAX_MATCHING_PRECEDENCE),
        ));
    }
    if value == 1 && flow_schema_name != PRIORITY_LEVEL_NAME_EXEMPT {
        errors.push(ValidationError::invalid(
            field,
            "only the schema named 'exempt' may have matchingPrecedence 1",
        ));
    }

    errors
}

fn validate_flow_distinguisher_method(
    method_type: &str,
    field: &str,
) -> ValidationResult {
    if method_type.is_empty() {
        vec![ValidationError::required(field, "type is required")]
    } else if !VALID_DISTINGUISHER_METHODS.contains(&method_type) {
        vec![ValidationError::not_supported(
            field,
            method_type,
            VALID_DISTINGUISHER_METHODS,
        )]
    } else {
        Vec::new()
    }
}

fn validate_priority_level_reference(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "must reference a priority level"));
        return errors;
    }

    errors.extend(validate_dns_subdomain_name(name, field));
    errors
}

fn validate_subject_kind(kind: &str, field: &str) -> ValidationResult {
    if kind.is_empty() {
        vec![ValidationError::required(field, "kind is required")]
    } else if !VALID_SUBJECT_KINDS.contains(&kind) {
        vec![ValidationError::not_supported(
            field,
            kind,
            VALID_SUBJECT_KINDS,
        )]
    } else {
        Vec::new()
    }
}

fn validate_subject_name(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, "name is required")]
    } else if value == "*" {
        Vec::new()
    } else {
        validate_dns_label(value, field)
    }
}

fn validate_verbs(values: &[String], field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if values.is_empty() {
        errors.push(ValidationError::required(
            field,
            "verbs must contain at least one value",
        ));
        return errors;
    }

    if has_wildcard(values) {
        if values.len() > 1 {
            errors.push(ValidationError::invalid(
                field,
                "if '*' is present, must not specify other verbs",
            ));
        }
        return errors;
    }

    for (i, verb) in values.iter().enumerate() {
        if !SUPPORTED_VERBS.contains(&verb.as_str()) {
            errors.push(ValidationError::not_supported(
                &format!("{}[{}]", field, i),
                verb,
                SUPPORTED_VERBS,
            ));
        }
    }

    errors
}

fn validate_non_resource_url(path: &str, field: &str) -> ValidationResult {
    if path.is_empty() {
        return vec![ValidationError::invalid(field, "must not be empty")];
    }
    if path == "/" {
        return Vec::new();
    }
    if !path.starts_with('/') {
        return vec![ValidationError::invalid(field, "must start with slash")];
    }
    if path.contains(' ') {
        return vec![ValidationError::invalid(field, "must not contain white-space")];
    }
    if path.contains("//") {
        return vec![ValidationError::invalid(field, "must not contain double slash")];
    }
    let wildcard_count = path.matches('*').count();
    if wildcard_count > 1 || (wildcard_count == 1 && !path.ends_with("/*")) {
        return vec![ValidationError::invalid(
            field,
            "wildcard can only do suffix matching",
        )];
    }
    Vec::new()
}

fn validate_resource_rule_lists(values: &[String], field: &str, label: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if values.is_empty() {
        errors.push(ValidationError::required(field, label));
        return errors;
    }

    if has_wildcard(values) && values.len() > 1 {
        errors.push(ValidationError::invalid(
            field,
            "if '*' is present, must not specify other values",
        ));
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

fn validate_namespaces(namespaces: &[String], cluster_scope: bool, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if namespaces.is_empty() && !cluster_scope {
        errors.push(ValidationError::required(
            field,
            "resource rules that are not cluster scoped must supply at least one namespace",
        ));
        return errors;
    }

    if has_wildcard(namespaces) && namespaces.len() > 1 {
        errors.push(ValidationError::invalid(
            field,
            "if '*' is present, must not specify other namespaces",
        ));
        return errors;
    }

    for (i, ns) in namespaces.iter().enumerate() {
        if ns == "*" {
            continue;
        }
        errors.extend(validate_dns_label(ns, &format!("{}[{}]", field, i)));
    }

    errors
}

fn validate_limit_response_type(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, "type is required")]
    } else if !VALID_LIMIT_RESPONSE_TYPES.contains(&value) {
        vec![ValidationError::not_supported(
            field,
            value,
            VALID_LIMIT_RESPONSE_TYPES,
        )]
    } else {
        Vec::new()
    }
}

pub mod v1 {
    use super::*;
    use k8s_api::flowcontrol::v1 as api;

    pub fn validate_flow_schema(schema: &api::FlowSchema) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&schema.metadata, "metadata", true));

        if let Some(spec) = &schema.spec {
            errors.extend(validate_flow_schema_spec(
                spec,
                &schema.metadata.name,
                "spec",
            ));
        } else {
            errors.push(ValidationError::required("spec", "spec is required"));
        }

        errors
    }

    pub fn validate_priority_level_configuration(
        configuration: &api::PriorityLevelConfiguration,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&configuration.metadata, "metadata", true));

        if let Some(spec) = &configuration.spec {
            errors.extend(validate_priority_level_configuration_spec(
                spec,
                &configuration.metadata.name,
                "spec",
            ));
        } else {
            errors.push(ValidationError::required("spec", "spec is required"));
        }

        errors
    }

    fn validate_flow_schema_spec(
        spec: &api::FlowSchemaSpec,
        schema_name: &str,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_priority_level_reference(
            &spec.priority_level_configuration.name,
            &format!("{}.priorityLevelConfiguration.name", field),
        ));

        if let Some(value) = spec.matching_precedence {
            errors.extend(validate_matching_precedence(
                value,
                &format!("{}.matchingPrecedence", field),
                schema_name,
            ));
        }

        if let Some(method) = &spec.distinguisher_method {
            errors.extend(validate_flow_distinguisher_method(
                &method.type_,
                &format!("{}.distinguisherMethod.type", field),
            ));
        }

        if spec.rules.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.rules", field),
                "rules must contain at least one item",
            ));
        } else {
            for (i, rule) in spec.rules.iter().enumerate() {
                errors.extend(validate_policy_rules_with_subjects(
                    rule,
                    &format!("{}.rules[{}]", field, i),
                ));
            }
        }

        errors
    }

    fn validate_policy_rules_with_subjects(
        rules: &api::PolicyRulesWithSubjects,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if rules.subjects.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.subjects", field),
                "subjects must contain at least one value",
            ));
        } else {
            for (i, subject) in rules.subjects.iter().enumerate() {
                errors.extend(validate_subject(
                    subject,
                    &format!("{}.subjects[{}]", field, i),
                ));
            }
        }

        if rules.resource_rules.is_empty() && rules.non_resource_rules.is_empty() {
            errors.push(ValidationError::required(
                field,
                "at least one of resourceRules and nonResourceRules has to be non-empty",
            ));
        }

        for (i, rule) in rules.resource_rules.iter().enumerate() {
            errors.extend(validate_resource_policy_rule(
                rule,
                &format!("{}.resourceRules[{}]", field, i),
            ));
        }

        for (i, rule) in rules.non_resource_rules.iter().enumerate() {
            errors.extend(validate_non_resource_policy_rule(
                rule,
                &format!("{}.nonResourceRules[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_subject(subject: &api::Subject, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_kind(&subject.kind, &format!("{}.kind", field)));

        match subject.kind.as_str() {
            "ServiceAccount" => {
                if let Some(service_account) = &subject.service_account {
                    errors.extend(validate_service_account_subject(
                        service_account,
                        &format!("{}.serviceAccount", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is required when subject kind is 'ServiceAccount'",
                    ));
                }
                if subject.user.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.user", field),
                        "user is forbidden when subject kind is not 'User'",
                    ));
                }
                if subject.group.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.group", field),
                        "group is forbidden when subject kind is not 'Group'",
                    ));
                }
            }
            "User" => {
                if let Some(user) = &subject.user {
                    errors.extend(validate_user_subject(user, &format!("{}.user", field)));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.user", field),
                        "user is required when subject kind is 'User'",
                    ));
                }
                if subject.service_account.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                    ));
                }
                if subject.group.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.group", field),
                        "group is forbidden when subject kind is not 'Group'",
                    ));
                }
            }
            "Group" => {
                if let Some(group) = &subject.group {
                    errors.extend(validate_group_subject(group, &format!("{}.group", field)));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.group", field),
                        "group is required when subject kind is 'Group'",
                    ));
                }
                if subject.service_account.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                    ));
                }
                if subject.user.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.user", field),
                        "user is forbidden when subject kind is not 'User'",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_user_subject(subject: &api::UserSubject, field: &str) -> ValidationResult {
        validate_subject_name(&subject.name, &format!("{}.name", field))
    }

    fn validate_group_subject(subject: &api::GroupSubject, field: &str) -> ValidationResult {
        validate_subject_name(&subject.name, &format!("{}.name", field))
    }

    fn validate_service_account_subject(
        subject: &api::ServiceAccountSubject,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if subject.namespace.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.namespace", field),
                "must specify namespace for service account",
            ));
        } else {
            errors.extend(validate_dns_label(
                &subject.namespace,
                &format!("{}.namespace", field),
            ));
        }

        errors.extend(validate_subject_name(&subject.name, &format!("{}.name", field)));

        errors
    }

    fn validate_resource_policy_rule(
        rule: &api::ResourcePolicyRule,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_verbs(&rule.verbs, &format!("{}.verbs", field)));
        errors.extend(validate_resource_rule_lists(
            &rule.api_groups,
            &format!("{}.apiGroups", field),
            "resource rules must supply at least one api group",
        ));
        errors.extend(validate_resource_rule_lists(
            &rule.resources,
            &format!("{}.resources", field),
            "resource rules must supply at least one resource",
        ));
        errors.extend(validate_namespaces(
            &rule.namespaces,
            rule.cluster_scope.unwrap_or(false),
            &format!("{}.namespaces", field),
        ));

        errors
    }

    fn validate_non_resource_policy_rule(
        rule: &api::NonResourcePolicyRule,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_verbs(&rule.verbs, &format!("{}.verbs", field)));

        if rule.non_resource_urls.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.nonResourceURLs", field),
                "nonResourceURLs must contain at least one value",
            ));
            return errors;
        }

        if has_wildcard(&rule.non_resource_urls) && rule.non_resource_urls.len() > 1 {
            errors.push(ValidationError::invalid(
                &format!("{}.nonResourceURLs", field),
                "if '*' is present, must not specify other non-resource URLs",
            ));
            return errors;
        }

        for (i, url) in rule.non_resource_urls.iter().enumerate() {
            errors.extend(validate_non_resource_url(
                url,
                &format!("{}.nonResourceURLs[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_priority_level_configuration_spec(
        spec: &api::PriorityLevelConfigurationSpec,
        name: &str,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if (name == PRIORITY_LEVEL_NAME_EXEMPT) != (spec.type_ == "Exempt") {
            errors.push(ValidationError::invalid(
                &format!("{}.type", field),
                "must be 'Exempt' if and only if name is 'exempt'",
            ));
        }

        if spec.type_.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.type", field),
                "type is required",
            ));
            return errors;
        }

        if !VALID_PRIORITY_LEVEL_TYPES.contains(&spec.type_.as_str()) {
            errors.push(ValidationError::not_supported(
                &format!("{}.type", field),
                &spec.type_,
                VALID_PRIORITY_LEVEL_TYPES,
            ));
            return errors;
        }

        match spec.type_.as_str() {
            "Exempt" => {
                if spec.limited.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.limited", field),
                        "must be nil if the type is not Limited",
                    ));
                }
                if let Some(exempt) = &spec.exempt {
                    errors.extend(validate_exempt_priority_level(
                        exempt,
                        &format!("{}.exempt", field),
                    ));
                }
            }
            "Limited" => {
                if spec.exempt.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.exempt", field),
                        "must be nil if the type is Limited",
                    ));
                }
                if let Some(limited) = &spec.limited {
                    errors.extend(validate_limited_priority_level(
                        limited,
                        &format!("{}.limited", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.limited", field),
                        "must not be empty when type is Limited",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_limited_priority_level(
        limited: &api::LimitedPriorityLevelConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = limited.nominal_concurrency_shares {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.nominalConcurrencyShares", field),
                    "must be a non-negative integer",
                ));
            }
        }

        if let Some(limit_response) = &limited.limit_response {
            errors.extend(validate_limit_response(
                limit_response,
                &format!("{}.limitResponse", field),
            ));
        }

        if let Some(value) = limited.lendable_percent {
            if value < 0 || value > 100 {
                errors.push(ValidationError::invalid(
                    &format!("{}.lendablePercent", field),
                    "must be between 0 and 100, inclusive",
                ));
            }
        }

        if let Some(value) = limited.borrowing_limit_percent {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.borrowingLimitPercent", field),
                    "if specified, must be a non-negative integer",
                ));
            }
        }

        errors
    }

    fn validate_exempt_priority_level(
        exempt: &api::ExemptPriorityLevelConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = exempt.nominal_concurrency_shares {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.nominalConcurrencyShares", field),
                    "must be a non-negative integer",
                ));
            }
        }

        if let Some(value) = exempt.lendable_percent {
            if value < 0 || value > 100 {
                errors.push(ValidationError::invalid(
                    &format!("{}.lendablePercent", field),
                    "must be between 0 and 100, inclusive",
                ));
            }
        }

        errors
    }

    fn validate_limit_response(limit_response: &api::LimitResponse, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_limit_response_type(
            &limit_response.type_,
            &format!("{}.type", field),
        ));

        match limit_response.type_.as_str() {
            "Reject" => {
                if limit_response.queuing.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.queuing", field),
                        "must be nil if limitResponse.type is Reject",
                    ));
                }
            }
            "Queue" => {
                if let Some(queuing) = &limit_response.queuing {
                    errors.extend(validate_queuing_configuration(
                        queuing,
                        &format!("{}.queuing", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.queuing", field),
                        "must not be empty if limitResponse.type is Queue",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_queuing_configuration(
        queuing: &api::QueuingConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = queuing.queue_length_limit {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.queueLengthLimit", field),
                    "must be positive",
                ));
            }
        }

        if let Some(value) = queuing.queues {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.queues", field),
                    "must be positive",
                ));
            }
        }

        if let Some(value) = queuing.hand_size {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.handSize", field),
                    "must be positive",
                ));
            }
        }

        if let (Some(queues), Some(hand_size)) = (queuing.queues, queuing.hand_size) {
            if hand_size > queues {
                errors.push(ValidationError::invalid(
                    &format!("{}.handSize", field),
                    "should not be greater than queues",
                ));
            }
        }

        errors
    }
}

pub mod v1beta3 {
    use super::*;
    use k8s_api::flowcontrol::v1beta3 as api;

    pub fn validate_flow_schema(schema: &api::FlowSchema) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&schema.metadata, "metadata", true));

        if let Some(spec) = &schema.spec {
            errors.extend(validate_flow_schema_spec(
                spec,
                &schema.metadata.name,
                "spec",
            ));
        } else {
            errors.push(ValidationError::required("spec", "spec is required"));
        }

        errors
    }

    pub fn validate_priority_level_configuration(
        configuration: &api::PriorityLevelConfiguration,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&configuration.metadata, "metadata", true));

        if let Some(spec) = &configuration.spec {
            errors.extend(validate_priority_level_configuration_spec(
                spec,
                &configuration.metadata.name,
                "spec",
            ));
        } else {
            errors.push(ValidationError::required("spec", "spec is required"));
        }

        errors
    }

    fn validate_flow_schema_spec(
        spec: &api::FlowSchemaSpec,
        schema_name: &str,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_priority_level_reference(
            &spec.priority_level_configuration.name,
            &format!("{}.priorityLevelConfiguration.name", field),
        ));
        if let Some(value) = spec.matching_precedence {
            errors.extend(validate_matching_precedence(
                value,
                &format!("{}.matchingPrecedence", field),
                schema_name,
            ));
        }

        if let Some(method) = &spec.distinguisher_method {
            errors.extend(validate_flow_distinguisher_method(
                &method.type_,
                &format!("{}.distinguisherMethod.type", field),
            ));
        }

        if spec.rules.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.rules", field),
                "rules must contain at least one item",
            ));
        } else {
            for (i, rule) in spec.rules.iter().enumerate() {
                errors.extend(validate_policy_rules_with_subjects(
                    rule,
                    &format!("{}.rules[{}]", field, i),
                ));
            }
        }

        errors
    }

    fn validate_policy_rules_with_subjects(
        rules: &api::PolicyRulesWithSubjects,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if rules.subjects.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.subjects", field),
                "subjects must contain at least one value",
            ));
        } else {
            for (i, subject) in rules.subjects.iter().enumerate() {
                errors.extend(validate_subject(
                    subject,
                    &format!("{}.subjects[{}]", field, i),
                ));
            }
        }

        if rules.resource_rules.is_empty() && rules.non_resource_rules.is_empty() {
            errors.push(ValidationError::required(
                field,
                "at least one of resourceRules and nonResourceRules has to be non-empty",
            ));
        }

        for (i, rule) in rules.resource_rules.iter().enumerate() {
            errors.extend(validate_resource_policy_rule(
                rule,
                &format!("{}.resourceRules[{}]", field, i),
            ));
        }

        for (i, rule) in rules.non_resource_rules.iter().enumerate() {
            errors.extend(validate_non_resource_policy_rule(
                rule,
                &format!("{}.nonResourceRules[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_subject(subject: &api::Subject, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_kind(&subject.kind, &format!("{}.kind", field)));

        match subject.kind.as_str() {
            "ServiceAccount" => {
                if let Some(service_account) = &subject.service_account {
                    errors.extend(validate_service_account_subject(
                        service_account,
                        &format!("{}.serviceAccount", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is required when subject kind is 'ServiceAccount'",
                    ));
                }
                if subject.user.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.user", field),
                        "user is forbidden when subject kind is not 'User'",
                    ));
                }
                if subject.group.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.group", field),
                        "group is forbidden when subject kind is not 'Group'",
                    ));
                }
            }
            "User" => {
                if let Some(user) = &subject.user {
                    errors.extend(validate_user_subject(user, &format!("{}.user", field)));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.user", field),
                        "user is required when subject kind is 'User'",
                    ));
                }
                if subject.service_account.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                    ));
                }
                if subject.group.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.group", field),
                        "group is forbidden when subject kind is not 'Group'",
                    ));
                }
            }
            "Group" => {
                if let Some(group) = &subject.group {
                    errors.extend(validate_group_subject(group, &format!("{}.group", field)));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.group", field),
                        "group is required when subject kind is 'Group'",
                    ));
                }
                if subject.service_account.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.serviceAccount", field),
                        "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                    ));
                }
                if subject.user.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.user", field),
                        "user is forbidden when subject kind is not 'User'",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_user_subject(subject: &api::UserSubject, field: &str) -> ValidationResult {
        validate_subject_name(&subject.name, &format!("{}.name", field))
    }

    fn validate_group_subject(subject: &api::GroupSubject, field: &str) -> ValidationResult {
        validate_subject_name(&subject.name, &format!("{}.name", field))
    }

    fn validate_service_account_subject(
        subject: &api::ServiceAccountSubject,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if subject.namespace.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.namespace", field),
                "must specify namespace for service account",
            ));
        } else {
            errors.extend(validate_dns_label(
                &subject.namespace,
                &format!("{}.namespace", field),
            ));
        }

        errors.extend(validate_subject_name(&subject.name, &format!("{}.name", field)));

        errors
    }

    fn validate_resource_policy_rule(
        rule: &api::ResourcePolicyRule,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_verbs(&rule.verbs, &format!("{}.verbs", field)));
        errors.extend(validate_resource_rule_lists(
            &rule.api_groups,
            &format!("{}.apiGroups", field),
            "resource rules must supply at least one api group",
        ));
        errors.extend(validate_resource_rule_lists(
            &rule.resources,
            &format!("{}.resources", field),
            "resource rules must supply at least one resource",
        ));
        errors.extend(validate_namespaces(
            &rule.namespaces,
            rule.cluster_scope.unwrap_or(false),
            &format!("{}.namespaces", field),
        ));

        errors
    }

    fn validate_non_resource_policy_rule(
        rule: &api::NonResourcePolicyRule,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_verbs(&rule.verbs, &format!("{}.verbs", field)));

        if rule.non_resource_urls.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.nonResourceURLs", field),
                "nonResourceURLs must contain at least one value",
            ));
            return errors;
        }

        if has_wildcard(&rule.non_resource_urls) && rule.non_resource_urls.len() > 1 {
            errors.push(ValidationError::invalid(
                &format!("{}.nonResourceURLs", field),
                "if '*' is present, must not specify other non-resource URLs",
            ));
            return errors;
        }

        for (i, url) in rule.non_resource_urls.iter().enumerate() {
            errors.extend(validate_non_resource_url(
                url,
                &format!("{}.nonResourceURLs[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_priority_level_configuration_spec(
        spec: &api::PriorityLevelConfigurationSpec,
        name: &str,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if (name == PRIORITY_LEVEL_NAME_EXEMPT) != (spec.type_ == "Exempt") {
            errors.push(ValidationError::invalid(
                &format!("{}.type", field),
                "must be 'Exempt' if and only if name is 'exempt'",
            ));
        }

        if spec.type_.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.type", field),
                "type is required",
            ));
            return errors;
        }

        if !VALID_PRIORITY_LEVEL_TYPES.contains(&spec.type_.as_str()) {
            errors.push(ValidationError::not_supported(
                &format!("{}.type", field),
                &spec.type_,
                VALID_PRIORITY_LEVEL_TYPES,
            ));
            return errors;
        }

        match spec.type_.as_str() {
            "Exempt" => {
                if spec.limited.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.limited", field),
                        "must be nil if the type is not Limited",
                    ));
                }
                if let Some(exempt) = &spec.exempt {
                    errors.extend(validate_exempt_priority_level(
                        exempt,
                        &format!("{}.exempt", field),
                    ));
                }
            }
            "Limited" => {
                if spec.exempt.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.exempt", field),
                        "must be nil if the type is Limited",
                    ));
                }
                if let Some(limited) = &spec.limited {
                    errors.extend(validate_limited_priority_level(
                        limited,
                        &format!("{}.limited", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.limited", field),
                        "must not be empty when type is Limited",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_limited_priority_level(
        limited: &api::LimitedPriorityLevelConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = limited.nominal_concurrency_shares {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.nominalConcurrencyShares", field),
                    "must be a non-negative integer",
                ));
            }
        }

        if let Some(limit_response) = &limited.limit_response {
            errors.extend(validate_limit_response(
                limit_response,
                &format!("{}.limitResponse", field),
            ));
        }

        if let Some(value) = limited.lendable_percent {
            if value < 0 || value > 100 {
                errors.push(ValidationError::invalid(
                    &format!("{}.lendablePercent", field),
                    "must be between 0 and 100, inclusive",
                ));
            }
        }

        if let Some(value) = limited.borrowing_limit_percent {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.borrowingLimitPercent", field),
                    "if specified, must be a non-negative integer",
                ));
            }
        }

        errors
    }

    fn validate_exempt_priority_level(
        exempt: &api::ExemptPriorityLevelConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = exempt.nominal_concurrency_shares {
            if value < 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.nominalConcurrencyShares", field),
                    "must be a non-negative integer",
                ));
            }
        }

        if let Some(value) = exempt.lendable_percent {
            if value < 0 || value > 100 {
                errors.push(ValidationError::invalid(
                    &format!("{}.lendablePercent", field),
                    "must be between 0 and 100, inclusive",
                ));
            }
        }

        errors
    }

    fn validate_limit_response(limit_response: &api::LimitResponse, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_limit_response_type(
            &limit_response.type_,
            &format!("{}.type", field),
        ));

        match limit_response.type_.as_str() {
            "Reject" => {
                if limit_response.queuing.is_some() {
                    errors.push(ValidationError::forbidden(
                        &format!("{}.queuing", field),
                        "must be nil if limitResponse.type is Reject",
                    ));
                }
            }
            "Queue" => {
                if let Some(queuing) = &limit_response.queuing {
                    errors.extend(validate_queuing_configuration(
                        queuing,
                        &format!("{}.queuing", field),
                    ));
                } else {
                    errors.push(ValidationError::required(
                        &format!("{}.queuing", field),
                        "must not be empty if limitResponse.type is Queue",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    fn validate_queuing_configuration(
        queuing: &api::QueuingConfiguration,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(value) = queuing.queue_length_limit {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.queueLengthLimit", field),
                    "must be positive",
                ));
            }
        }

        if let Some(value) = queuing.queues {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.queues", field),
                    "must be positive",
                ));
            }
        }

        if let Some(value) = queuing.hand_size {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    &format!("{}.handSize", field),
                    "must be positive",
                ));
            }
        }

        if let (Some(queues), Some(hand_size)) = (queuing.queues, queuing.hand_size) {
            if hand_size > queues {
                errors.push(ValidationError::invalid(
                    &format!("{}.handSize", field),
                    "should not be greater than queues",
                ));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1beta3 as validation_v1beta3;
    use k8s_api::flowcontrol::v1beta3 as api_v1beta3;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_flow_schema_missing_priority_level() {
        let schema = api_v1beta3::FlowSchema {
            metadata: ObjectMeta::named("schema"),
            spec: Some(api_v1beta3::FlowSchemaSpec {
                priority_level_configuration: api_v1beta3::PriorityLevelConfigurationReference {
                    name: "".to_string(),
                },
                matching_precedence: Some(10),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1beta3::validate_flow_schema(&schema);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.field.contains("priorityLevelConfiguration")));
    }

    #[test]
    fn test_validate_priority_level_exempt_mismatch() {
        let config = api_v1beta3::PriorityLevelConfiguration {
            metadata: ObjectMeta::named("exempt"),
            spec: Some(api_v1beta3::PriorityLevelConfigurationSpec {
                type_: "Limited".to_string(),
                limited: Some(api_v1beta3::LimitedPriorityLevelConfiguration {
                    nominal_concurrency_shares: Some(10),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1beta3::validate_priority_level_configuration(&config);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|error| error.field.contains("type")));
    }

    #[test]
    fn test_validate_non_resource_rule_invalid_url() {
        let rule = api_v1beta3::NonResourcePolicyRule {
            verbs: vec!["get".to_string()],
            non_resource_urls: vec!["bad".to_string()],
        };

        let schema = api_v1beta3::FlowSchema {
            metadata: ObjectMeta::named("schema"),
            spec: Some(api_v1beta3::FlowSchemaSpec {
                priority_level_configuration: api_v1beta3::PriorityLevelConfigurationReference {
                    name: "pl".to_string(),
                },
                matching_precedence: Some(10),
                rules: vec![api_v1beta3::PolicyRulesWithSubjects {
                    subjects: vec![api_v1beta3::Subject {
                        kind: "User".to_string(),
                        user: Some(api_v1beta3::UserSubject {
                            name: "test".to_string(),
                        }),
                        ..Default::default()
                    }],
                    non_resource_rules: vec![rule],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1beta3::validate_flow_schema(&schema);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.field.contains("nonResourceURLs")));
    }
}
