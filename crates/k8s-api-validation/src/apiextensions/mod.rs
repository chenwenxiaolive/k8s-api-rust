//! API Extensions validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

const VALID_SCOPES: &[&str] = &["Cluster", "Namespaced"];

pub mod v1 {
    use super::*;
    use k8s_api::apiextensions::v1 as api;

    pub fn validate_custom_resource_definition(
        crd: &api::CustomResourceDefinition,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&crd.metadata, "metadata", true));

        if crd.spec.group.is_empty() {
            errors.push(ValidationError::required(
                "spec.group",
                "group is required",
            ));
        }

        if crd.spec.scope.is_empty() {
            errors.push(ValidationError::required(
                "spec.scope",
                "scope is required",
            ));
        } else if !VALID_SCOPES.contains(&crd.spec.scope.as_str()) {
            errors.push(ValidationError::not_supported(
                "spec.scope",
                &crd.spec.scope,
                VALID_SCOPES,
            ));
        }

        if crd.spec.names.plural.is_empty() {
            errors.push(ValidationError::required(
                "spec.names.plural",
                "plural is required",
            ));
        }
        if crd.spec.names.kind.is_empty() {
            errors.push(ValidationError::required(
                "spec.names.kind",
                "kind is required",
            ));
        }

        if crd.spec.versions.is_empty() {
            errors.push(ValidationError::required(
                "spec.versions",
                "versions must not be empty",
            ));
        } else {
            let mut storage_found = false;
            for (idx, version) in crd.spec.versions.iter().enumerate() {
                if version.name.is_empty() {
                    errors.push(ValidationError::required(
                        format!("spec.versions[{}].name", idx),
                        "name is required",
                    ));
                }
                if version.storage {
                    storage_found = true;
                }
            }

            if !storage_found {
                errors.push(ValidationError::required(
                    "spec.versions",
                    "at least one version must be marked as storage",
                ));
            }
        }

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::apiextensions::internal as api;

    pub fn validate_custom_resource_definition(
        crd: &api::CustomResourceDefinition,
    ) -> ValidationResult {
        crate::internal::validate_with(
            crd,
            "customResourceDefinition",
            super::v1::validate_custom_resource_definition,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use k8s_api::apiextensions::v1 as api_v1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_crd_missing_group() {
        let crd = api_v1::CustomResourceDefinition {
            metadata: ObjectMeta::named("foos.example.com"),
            spec: api_v1::CustomResourceDefinitionSpec {
                group: String::new(),
                names: api_v1::CustomResourceDefinitionNames {
                    plural: "foos".to_string(),
                    kind: "Foo".to_string(),
                    ..Default::default()
                },
                scope: "Namespaced".to_string(),
                versions: vec![api_v1::CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_custom_resource_definition(&crd);
        assert!(errors.iter().any(|e| e.field.contains("spec.group")));
    }

    #[test]
    fn test_validate_crd_missing_storage_version() {
        let crd = api_v1::CustomResourceDefinition {
            metadata: ObjectMeta::named("foos.example.com"),
            spec: api_v1::CustomResourceDefinitionSpec {
                group: "example.com".to_string(),
                names: api_v1::CustomResourceDefinitionNames {
                    plural: "foos".to_string(),
                    kind: "Foo".to_string(),
                    ..Default::default()
                },
                scope: "Namespaced".to_string(),
                versions: vec![api_v1::CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: false,
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_custom_resource_definition(&crd);
        assert!(errors.iter().any(|e| e.field.contains("spec.versions")));
    }

    #[test]
    fn test_validate_crd_valid() {
        let crd = api_v1::CustomResourceDefinition {
            metadata: ObjectMeta::named("foos.example.com"),
            spec: api_v1::CustomResourceDefinitionSpec {
                group: "example.com".to_string(),
                names: api_v1::CustomResourceDefinitionNames {
                    plural: "foos".to_string(),
                    kind: "Foo".to_string(),
                    ..Default::default()
                },
                scope: "Namespaced".to_string(),
                versions: vec![api_v1::CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_custom_resource_definition(&crd);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
