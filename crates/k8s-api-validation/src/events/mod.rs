//! Events API validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

const VALID_EVENT_TYPES: &[&str] = &["Normal", "Warning"];

fn validate_event_type(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        Vec::new()
    } else if !VALID_EVENT_TYPES.contains(&value) {
        vec![ValidationError::not_supported(field, value, VALID_EVENT_TYPES)]
    } else {
        Vec::new()
    }
}

pub mod v1 {
    use super::*;
    use k8s_api::events::v1 as api;

    pub fn validate_event(event: &api::Event) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&event.metadata, "metadata", false));

        if event.event_time.is_empty() {
            errors.push(ValidationError::required(
                "eventTime",
                "eventTime is required",
            ));
        }

        errors.extend(validate_event_type(&event.type_, "type"));

        if let Some(series) = &event.series {
            if series.count <= 0 {
                errors.push(ValidationError::invalid(
                    "series.count",
                    "must be positive",
                ));
            }
            if series.last_observed_time.is_empty() {
                errors.push(ValidationError::required(
                    "series.lastObservedTime",
                    "lastObservedTime is required",
                ));
            }
        }

        if let Some(count) = event.deprecated_count {
            if count < 0 {
                errors.push(ValidationError::invalid(
                    "deprecatedCount",
                    "must be non-negative",
                ));
            }
        }

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::events::v1beta1 as api;

    pub fn validate_event(event: &api::Event) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&event.metadata, "metadata", false));

        errors.extend(validate_event_type(&event.event_type, "type"));

        if let Some(series) = &event.series {
            if series.count <= 0 {
                errors.push(ValidationError::invalid(
                    "series.count",
                    "must be positive",
                ));
            }
        }

        if let Some(count) = event.deprecated_count {
            if count < 0 {
                errors.push(ValidationError::invalid(
                    "deprecatedCount",
                    "must be non-negative",
                ));
            }
        }

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::events::internal as api;

    pub fn validate_event(event: &api::Event) -> ValidationResult {
        crate::internal::validate_with(event, "event", super::v1::validate_event)
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use k8s_api::events::v1 as api_v1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_event_missing_event_time() {
        let event = api_v1::Event {
            metadata: ObjectMeta::named("evt"),
            event_time: String::new(),
            ..Default::default()
        };

        let errors = validation_v1::validate_event(&event);
        assert!(errors.iter().any(|e| e.field.contains("eventTime")));
    }

    #[test]
    fn test_validate_event_invalid_type() {
        let event = api_v1::Event {
            metadata: ObjectMeta::named("evt"),
            event_time: "now".to_string(),
            type_: "Info".to_string(),
            ..Default::default()
        };

        let errors = validation_v1::validate_event(&event);
        assert!(errors.iter().any(|e| e.field.contains("type")));
    }

    #[test]
    fn test_validate_event_valid() {
        let event = api_v1::Event {
            metadata: ObjectMeta::named("evt"),
            event_time: "now".to_string(),
            type_: "Normal".to_string(),
            reason: "Started".to_string(),
            ..Default::default()
        };

        let errors = validation_v1::validate_event(&event);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
