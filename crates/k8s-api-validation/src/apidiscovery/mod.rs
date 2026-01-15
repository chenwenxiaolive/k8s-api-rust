//! API discovery validation

use crate::{ValidationError, ValidationResult};

const VALID_SCOPES: &[&str] = &["Cluster", "Namespaced"];
const VALID_FRESHNESS: &[&str] = &["Current", "Stale"];

fn validate_scope(scope: &str, field: &str) -> ValidationResult {
    if scope.is_empty() {
        vec![ValidationError::required(field, "scope is required")]
    } else if !VALID_SCOPES.contains(&scope) {
        vec![ValidationError::not_supported(field, scope, VALID_SCOPES)]
    } else {
        Vec::new()
    }
}

fn validate_freshness(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        Vec::new()
    } else if !VALID_FRESHNESS.contains(&value) {
        vec![ValidationError::not_supported(field, value, VALID_FRESHNESS)]
    } else {
        Vec::new()
    }
}

fn validate_group_version_kind(version: &str, kind: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if version.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.version", field),
            "version is required",
        ));
    }
    if kind.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.kind", field),
            "kind is required",
        ));
    }

    errors
}

fn validate_api_resource(
    resource: &str,
    singular: &str,
    scope: &str,
    verbs: &[String],
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if resource.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.resource", field),
            "resource is required",
        ));
    }
    if singular.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.singularResource", field),
            "singularResource is required",
        ));
    }
    errors.extend(validate_scope(scope, &format!("{}.scope", field)));
    if verbs.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.verbs", field),
            "verbs must not be empty",
        ));
    }

    errors
}

pub mod v2 {
    use super::*;
    use k8s_api::apidiscovery::v2 as api;
    pub fn validate_api_group_discovery(group: &api::APIGroupDiscovery) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(crate::common::validate_object_meta(
            &group.metadata,
            "metadata",
            false,
        ));

        if group.versions.is_empty() {
            errors.push(ValidationError::required(
                "versions",
                "versions must not be empty",
            ));
        }

        for (version_idx, version) in group.versions.iter().enumerate() {
            let version_field = format!("versions[{}]", version_idx);
            if version.version.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.version", version_field),
                    "version is required",
                ));
            }
            errors.extend(validate_freshness(
                &version.freshness,
                &format!("{}.freshness", version_field),
            ));

            if version.resources.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.resources", version_field),
                    "resources must not be empty",
                ));
            }

            for (resource_idx, resource) in version.resources.iter().enumerate() {
                let resource_field = format!("{}.resources[{}]", version_field, resource_idx);
                errors.extend(validate_api_resource(
                    &resource.resource,
                    &resource.singular_resource,
                    &resource.scope,
                    &resource.verbs,
                    &resource_field,
                ));

                if let Some(kind) = &resource.response_kind {
                    errors.extend(validate_group_version_kind(
                        &kind.version,
                        &kind.kind,
                        &format!("{}.responseKind", resource_field),
                    ));
                }

                for (sub_idx, subresource) in resource.subresources.iter().enumerate() {
                    let sub_field = format!("{}.subresources[{}]", resource_field, sub_idx);
                    if subresource.subresource.is_empty() {
                        errors.push(ValidationError::required(
                            format!("{}.subresource", sub_field),
                            "subresource is required",
                        ));
                    }
                    if subresource.verbs.is_empty() {
                        errors.push(ValidationError::required(
                            format!("{}.verbs", sub_field),
                            "verbs must not be empty",
                        ));
                    }
                    if let Some(kind) = &subresource.response_kind {
                        errors.extend(validate_group_version_kind(
                            &kind.version,
                            &kind.kind,
                            &format!("{}.responseKind", sub_field),
                        ));
                    }
                }
            }
        }

        errors
    }

    pub fn validate_api_group_discovery_list(
        list: &api::APIGroupDiscoveryList,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        for (idx, item) in list.items.iter().enumerate() {
            let field = format!("items[{}]", idx);
            for error in validate_api_group_discovery(item) {
                errors.push(ValidationError {
                    field: format!("{}.{}", field, error.field),
                    ..error
                });
            }
        }

        errors
    }
}

pub mod v2beta1 {
    use super::*;
    use k8s_api::apidiscovery::v2beta1 as api;
    pub fn validate_api_group_discovery(group: &api::APIGroupDiscovery) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(crate::common::validate_object_meta(
            &group.metadata,
            "metadata",
            false,
        ));

        if group.versions.is_empty() {
            errors.push(ValidationError::required(
                "versions",
                "versions must not be empty",
            ));
        }

        for (version_idx, version) in group.versions.iter().enumerate() {
            let version_field = format!("versions[{}]", version_idx);
            if version.version.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.version", version_field),
                    "version is required",
                ));
            }
            errors.extend(validate_freshness(
                &version.freshness,
                &format!("{}.freshness", version_field),
            ));

            if version.resources.is_empty() {
                errors.push(ValidationError::required(
                    format!("{}.resources", version_field),
                    "resources must not be empty",
                ));
            }

            for (resource_idx, resource) in version.resources.iter().enumerate() {
                let resource_field = format!("{}.resources[{}]", version_field, resource_idx);
                errors.extend(validate_api_resource(
                    &resource.resource,
                    &resource.singular_resource,
                    &resource.scope,
                    &resource.verbs,
                    &resource_field,
                ));

                if let Some(kind) = &resource.response_kind {
                    errors.extend(validate_group_version_kind(
                        &kind.version,
                        &kind.kind,
                        &format!("{}.responseKind", resource_field),
                    ));
                }

                for (sub_idx, subresource) in resource.subresources.iter().enumerate() {
                    let sub_field = format!("{}.subresources[{}]", resource_field, sub_idx);
                    if subresource.subresource.is_empty() {
                        errors.push(ValidationError::required(
                            format!("{}.subresource", sub_field),
                            "subresource is required",
                        ));
                    }
                    if subresource.verbs.is_empty() {
                        errors.push(ValidationError::required(
                            format!("{}.verbs", sub_field),
                            "verbs must not be empty",
                        ));
                    }
                    if let Some(kind) = &subresource.response_kind {
                        errors.extend(validate_group_version_kind(
                            &kind.version,
                            &kind.kind,
                            &format!("{}.responseKind", sub_field),
                        ));
                    }
                }
            }
        }

        errors
    }

    pub fn validate_api_group_discovery_list(
        list: &api::APIGroupDiscoveryList,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        for (idx, item) in list.items.iter().enumerate() {
            let field = format!("items[{}]", idx);
            for error in validate_api_group_discovery(item) {
                errors.push(ValidationError {
                    field: format!("{}.{}", field, error.field),
                    ..error
                });
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v2 as validation_v2;
    use k8s_api::apidiscovery::v2 as api_v2;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_api_group_discovery_missing_version() {
        let group = api_v2::APIGroupDiscovery {
            metadata: ObjectMeta::named("apps"),
            versions: vec![api_v2::APIVersionDiscovery {
                version: String::new(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validation_v2::validate_api_group_discovery(&group);
        assert!(errors.iter().any(|e| e.field.contains("versions[0].version")));
    }

    #[test]
    fn test_validate_api_group_discovery_valid() {
        let group = api_v2::APIGroupDiscovery {
            metadata: ObjectMeta::named("apps"),
            versions: vec![api_v2::APIVersionDiscovery {
                version: "v1".to_string(),
                resources: vec![api_v2::APIResourceDiscovery {
                    resource: "deployments".to_string(),
                    scope: "Namespaced".to_string(),
                    singular_resource: "deployment".to_string(),
                    verbs: vec!["get".to_string()],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };

        let errors = validation_v2::validate_api_group_discovery(&group);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
