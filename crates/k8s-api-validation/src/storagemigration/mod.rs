//! Storage migration validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

pub mod v1alpha1 {
    use super::*;
    use k8s_api::storagemigration::v1alpha1 as api;

    pub fn validate_storage_version_migration(
        migration: &api::StorageVersionMigration,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&migration.metadata, "metadata", true));

        let Some(spec) = &migration.spec else {
            errors.push(ValidationError::required("spec", "spec is required"));
            return errors;
        };

        if spec.resource.version.is_empty() {
            errors.push(ValidationError::required(
                "spec.resource.version",
                "version is required",
            ));
        }
        if spec.resource.resource.is_empty() {
            errors.push(ValidationError::required(
                "spec.resource.resource",
                "resource is required",
            ));
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1alpha1 as validation_v1alpha1;
    use k8s_api::storagemigration::v1alpha1 as api_v1alpha1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_storage_version_migration_missing_spec() {
        let migration = api_v1alpha1::StorageVersionMigration {
            metadata: ObjectMeta::named("migrate"),
            spec: None,
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_storage_version_migration(&migration);
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_storage_version_migration_valid() {
        let migration = api_v1alpha1::StorageVersionMigration {
            metadata: ObjectMeta::named("migrate"),
            spec: Some(api_v1alpha1::StorageVersionMigrationSpec {
                resource: api_v1alpha1::GroupVersionResource {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    resource: "deployments".to_string(),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_storage_version_migration(&migration);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
