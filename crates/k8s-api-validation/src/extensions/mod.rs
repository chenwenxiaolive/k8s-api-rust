//! Extensions API validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

pub mod v1beta1 {
    use super::*;
    use k8s_api::extensions::v1beta1 as api;

    pub fn validate_ingress(ingress: &api::Ingress) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&ingress.metadata, "metadata", true));

        let Some(spec) = &ingress.spec else {
            errors.push(ValidationError::required("spec", "spec is required"));
            return errors;
        };

        if spec.backend.is_none() && spec.rules.is_empty() {
            errors.push(ValidationError::required(
                "spec",
                "must specify a default backend or rules",
            ));
        }

        for (idx, rule) in spec.rules.iter().enumerate() {
            if let Some(http) = &rule.ingress_rule_value.http {
                if http.paths.is_empty() {
                    errors.push(ValidationError::required(
                        format!("spec.rules[{}].http.paths", idx),
                        "paths must not be empty",
                    ));
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1beta1 as validation_v1beta1;
    use k8s_api::extensions::v1beta1 as api_v1beta1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_ingress_missing_spec() {
        let ingress = api_v1beta1::Ingress {
            metadata: ObjectMeta::named("ing"),
            spec: None,
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_ingress(&ingress);
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_ingress_missing_backend_and_rules() {
        let ingress = api_v1beta1::Ingress {
            metadata: ObjectMeta::named("ing"),
            spec: Some(api_v1beta1::IngressSpec::default()),
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_ingress(&ingress);
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_ingress_valid() {
        let ingress = api_v1beta1::Ingress {
            metadata: ObjectMeta::named("ing"),
            spec: Some(api_v1beta1::IngressSpec {
                backend: Some(api_v1beta1::IngressBackend {
                    service_name: "default".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_ingress(&ingress);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
