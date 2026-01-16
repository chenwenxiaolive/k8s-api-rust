//! Batch API validation
//!
//! This module provides validation for batch/v1 API types (Job, CronJob).

use crate::common::{validate_labels, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::batch::v1::{CronJob, CronJobSpec, Job, JobSpec};

// =============================================================================
// Job Validation
// =============================================================================

/// Validates a Job.
pub fn validate_job(job: &Job) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&job.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &job.spec {
        errors.extend(validate_job_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a JobSpec.
pub fn validate_job_spec(spec: &JobSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Parallelism must be non-negative
    if let Some(parallelism) = spec.parallelism {
        if parallelism < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.parallelism", field_path),
                "parallelism must be non-negative",
            ));
        }
    }

    // Completions must be non-negative
    if let Some(completions) = spec.completions {
        if completions < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.completions", field_path),
                "completions must be non-negative",
            ));
        }
    }

    // Validate parallelism vs completions relationship
    if let (Some(parallelism), Some(completions)) = (spec.parallelism, spec.completions) {
        if parallelism > completions && completions > 0 {
            // This is allowed but unusual - parallelism > completions is valid
            // No error needed
        }
    }

    // BackoffLimit must be non-negative
    if let Some(backoff_limit) = spec.backoff_limit {
        if backoff_limit < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.backoffLimit", field_path),
                "backoffLimit must be non-negative",
            ));
        }
    }

    // BackoffLimitPerIndex must be non-negative
    if let Some(backoff_limit_per_index) = spec.backoff_limit_per_index {
        if backoff_limit_per_index < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.backoffLimitPerIndex", field_path),
                "backoffLimitPerIndex must be non-negative",
            ));
        }
    }

    // MaxFailedIndexes must be non-negative
    if let Some(max_failed_indexes) = spec.max_failed_indexes {
        if max_failed_indexes < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.maxFailedIndexes", field_path),
                "maxFailedIndexes must be non-negative",
            ));
        }
    }

    // ActiveDeadlineSeconds must be positive
    if let Some(deadline) = spec.active_deadline_seconds {
        if deadline <= 0 {
            errors.push(ValidationError::invalid(
                format!("{}.activeDeadlineSeconds", field_path),
                "activeDeadlineSeconds must be positive",
            ));
        }
    }

    // TTLSecondsAfterFinished must be non-negative
    if let Some(ttl) = spec.ttl_seconds_after_finished {
        if ttl < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.ttlSecondsAfterFinished", field_path),
                "ttlSecondsAfterFinished must be non-negative",
            ));
        }
    }

    // Validate completionMode
    if let Some(ref mode) = spec.completion_mode {
        let valid_modes = ["NonIndexed", "Indexed"];
        if !valid_modes.contains(&mode.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.completionMode", field_path),
                mode,
                &valid_modes,
            ));
        }

        // Indexed mode requires completions to be set
        if mode == "Indexed" && spec.completions.is_none() {
            errors.push(ValidationError::required(
                format!("{}.completions", field_path),
                "completions is required when completionMode is Indexed",
            ));
        }
    }

    // Validate podReplacementPolicy
    if let Some(ref policy) = spec.pod_replacement_policy {
        let valid_policies = ["TerminatingOrFailed", "Failed"];
        if !valid_policies.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.podReplacementPolicy", field_path),
                policy,
                &valid_policies,
            ));
        }
    }

    // Validate selector labels if present
    if let Some(ref selector) = spec.selector {
        errors.extend(validate_labels(
            &selector.match_labels,
            &format!("{}.selector.matchLabels", field_path),
        ));
    }

    // Validate pod template
    errors.extend(validate_job_pod_template(
        &spec.template,
        &format!("{}.template", field_path),
    ));

    errors
}

fn validate_job_pod_template(
    template: &k8s_api::core::v1::PodTemplateSpec,
    field_path: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate template metadata labels
    errors.extend(validate_labels(
        &template.metadata.labels,
        &format!("{}.metadata.labels", field_path),
    ));

    // Validate pod spec
    if let Some(spec) = &template.spec {
        errors.extend(crate::core::validate_pod_spec(
            spec,
            &format!("{}.spec", field_path),
        ));

        // Job-specific: restartPolicy must be Never or OnFailure
        let valid_policies = ["Never", "OnFailure"];
        if !valid_policies.contains(&spec.restart_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.spec.restartPolicy", field_path),
                &spec.restart_policy,
                &valid_policies,
            ));
        }
    } else {
        errors.push(ValidationError::required(
            format!("{}.spec", field_path),
            "pod template spec is required",
        ));
    }

    errors
}

// =============================================================================
// CronJob Validation
// =============================================================================

/// Validates a CronJob.
pub fn validate_cronjob(cronjob: &CronJob) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&cronjob.metadata, "metadata", true));

    // Validate spec
    if let Some(spec) = &cronjob.spec {
        errors.extend(validate_cronjob_spec(spec, "spec"));
    } else {
        errors.push(ValidationError::required("spec", "spec is required"));
    }

    errors
}

/// Validates a CronJobSpec.
pub fn validate_cronjob_spec(spec: &CronJobSpec, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Schedule is required and must be valid cron format
    if spec.schedule.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.schedule", field_path),
            "schedule is required",
        ));
    } else {
        errors.extend(validate_cron_schedule(
            &spec.schedule,
            &format!("{}.schedule", field_path),
        ));
    }

    // Validate concurrencyPolicy
    if !spec.concurrency_policy.is_empty() {
        let valid_policies = ["Allow", "Forbid", "Replace"];
        if !valid_policies.contains(&spec.concurrency_policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.concurrencyPolicy", field_path),
                &spec.concurrency_policy,
                &valid_policies,
            ));
        }
    }

    // StartingDeadlineSeconds must be positive if specified
    if let Some(deadline) = spec.starting_deadline_seconds {
        if deadline <= 0 {
            errors.push(ValidationError::invalid(
                format!("{}.startingDeadlineSeconds", field_path),
                "startingDeadlineSeconds must be positive",
            ));
        }
    }

    // SuccessfulJobsHistoryLimit must be non-negative
    if let Some(limit) = spec.successful_jobs_history_limit {
        if limit < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.successfulJobsHistoryLimit", field_path),
                "successfulJobsHistoryLimit must be non-negative",
            ));
        }
    }

    // FailedJobsHistoryLimit must be non-negative
    if let Some(limit) = spec.failed_jobs_history_limit {
        if limit < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.failedJobsHistoryLimit", field_path),
                "failedJobsHistoryLimit must be non-negative",
            ));
        }
    }

    // Validate job template
    errors.extend(validate_job_template_spec(
        &spec.job_template,
        &format!("{}.jobTemplate", field_path),
    ));

    errors
}

pub mod internal {
    use super::*;
    use k8s_api::batch::internal as api;

    pub fn validate_job(job: &api::Job) -> ValidationResult {
        crate::internal::validate_with(job, "job", super::validate_job)
    }

    pub fn validate_job_spec(spec: &api::JobSpec, field_path: &str) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_job_spec(external_spec, field_path)
        })
    }

    pub fn validate_cronjob(cronjob: &api::CronJob) -> ValidationResult {
        crate::internal::validate_with(cronjob, "cronJob", super::validate_cronjob)
    }

    pub fn validate_cronjob_spec(
        spec: &api::CronJobSpec,
        field_path: &str,
    ) -> ValidationResult {
        crate::internal::validate_with(spec, field_path, |external_spec| {
            super::validate_cronjob_spec(external_spec, field_path)
        })
    }
}

fn validate_job_template_spec(
    template: &k8s_api::batch::v1::JobTemplateSpec,
    field_path: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate template metadata
    errors.extend(validate_object_meta(
        &template.metadata,
        &format!("{}.metadata", field_path),
        false,
    ));

    // Validate job spec
    if let Some(spec) = &template.spec {
        errors.extend(validate_job_spec(spec, &format!("{}.spec", field_path)));
    } else {
        errors.push(ValidationError::required(
            format!("{}.spec", field_path),
            "job template spec is required",
        ));
    }

    errors
}

/// Validates a cron schedule expression.
fn validate_cron_schedule(schedule: &str, field_path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Basic cron format validation: should have 5 fields (minute hour day month weekday)
    // Or extended format with 6 fields (second minute hour day month weekday)
    let fields: Vec<&str> = schedule.split_whitespace().collect();

    if fields.len() < 5 || fields.len() > 6 {
        errors.push(ValidationError::invalid(
            field_path,
            format!(
                "invalid cron schedule: expected 5 or 6 fields, got {}",
                fields.len()
            ),
        ));
        return errors;
    }

    // Validate each field has valid characters
    for (i, field) in fields.iter().enumerate() {
        if !is_valid_cron_field(field) {
            errors.push(ValidationError::invalid(
                field_path,
                format!("invalid cron field at position {}: {}", i, field),
            ));
        }
    }

    errors
}

/// Checks if a cron field contains only valid characters.
fn is_valid_cron_field(field: &str) -> bool {
    // Valid cron field characters: 0-9, *, -, /, ,
    // Also special strings like @yearly, @monthly, etc.
    if field.starts_with('@') {
        let valid_specials = [
            "@yearly",
            "@annually",
            "@monthly",
            "@weekly",
            "@daily",
            "@midnight",
            "@hourly",
        ];
        return valid_specials.contains(&field.to_lowercase().as_str());
    }

    field
        .chars()
        .all(|c| c.is_ascii_digit() || c == '*' || c == '-' || c == '/' || c == ',' || c == '?')
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::batch::v1::{CronJob, CronJobSpec, Job, JobSpec, JobTemplateSpec};
    use k8s_api::core::v1::{Container, PodSpec, PodTemplateSpec};
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_job_missing_spec() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: None,
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_valid_job() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(JobSpec {
                parallelism: Some(2),
                completions: Some(5),
                backoff_limit: Some(3),
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container::new("worker", "busybox")],
                        restart_policy: "Never".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_job_negative_parallelism() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(JobSpec {
                parallelism: Some(-1),
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container::new("worker", "busybox")],
                        restart_policy: "Never".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("parallelism")));
    }

    #[test]
    fn test_validate_job_invalid_restart_policy() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(JobSpec {
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container::new("worker", "busybox")],
                        restart_policy: "Always".to_string(), // Invalid for Jobs
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("restartPolicy")));
    }

    #[test]
    fn test_validate_job_invalid_completion_mode() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(JobSpec {
                completion_mode: Some("Invalid".to_string()),
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container::new("worker", "busybox")],
                        restart_policy: "Never".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("completionMode")));
    }

    #[test]
    fn test_validate_job_indexed_mode_requires_completions() {
        let job = Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(JobSpec {
                completion_mode: Some("Indexed".to_string()),
                completions: None, // Required for Indexed mode
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container::new("worker", "busybox")],
                        restart_policy: "Never".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("completions")));
    }

    #[test]
    fn test_validate_cronjob_missing_spec() {
        let cronjob = CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: None,
            ..Default::default()
        };

        let errors = validate_cronjob(&cronjob);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_valid_cronjob() {
        let cronjob = CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: Some(CronJobSpec {
                schedule: "*/5 * * * *".to_string(),
                concurrency_policy: "Forbid".to_string(),
                job_template: JobTemplateSpec {
                    spec: Some(JobSpec {
                        template: PodTemplateSpec {
                            spec: Some(PodSpec {
                                containers: vec![Container::new("cron", "busybox")],
                                restart_policy: "OnFailure".to_string(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_cronjob(&cronjob);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_cronjob_empty_schedule() {
        let cronjob = CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: Some(CronJobSpec {
                schedule: "".to_string(),
                job_template: JobTemplateSpec {
                    spec: Some(JobSpec {
                        template: PodTemplateSpec {
                            spec: Some(PodSpec {
                                containers: vec![Container::new("cron", "busybox")],
                                restart_policy: "Never".to_string(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_cronjob(&cronjob);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("schedule")));
    }

    #[test]
    fn test_validate_cronjob_invalid_concurrency_policy() {
        let cronjob = CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: Some(CronJobSpec {
                schedule: "* * * * *".to_string(),
                concurrency_policy: "Invalid".to_string(),
                job_template: JobTemplateSpec {
                    spec: Some(JobSpec {
                        template: PodTemplateSpec {
                            spec: Some(PodSpec {
                                containers: vec![Container::new("cron", "busybox")],
                                restart_policy: "Never".to_string(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_cronjob(&cronjob);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("concurrencyPolicy")));
    }

    #[test]
    fn test_validate_cron_schedule_valid() {
        let valid_schedules = [
            "* * * * *",
            "*/5 * * * *",
            "0 0 * * *",
            "0 0 1 * *",
            "0 0 * * 0",
            "0 0 1 1 *",
            "0,30 * * * *",
            "0-30 * * * *",
        ];

        for schedule in &valid_schedules {
            let errors = validate_cron_schedule(schedule, "schedule");
            assert!(
                errors.is_empty(),
                "Schedule '{}' should be valid, got errors: {:?}",
                schedule,
                errors
            );
        }
    }

    #[test]
    fn test_validate_cron_schedule_invalid() {
        let invalid_schedules = [
            "* * *",         // Too few fields
            "* * * * * * *", // Too many fields
        ];

        for schedule in &invalid_schedules {
            let errors = validate_cron_schedule(schedule, "schedule");
            assert!(
                !errors.is_empty(),
                "Schedule '{}' should be invalid",
                schedule
            );
        }
    }
}
