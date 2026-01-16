//! Batch API conversions
//!
//! This module provides conversions between batch API versions.

use crate::{ConversionError, Convertible};

// =============================================================================
// Job: v1beta1 <-> v1 (mostly same structure)
// =============================================================================

impl Convertible<k8s_api::batch::v1::Job> for k8s_api::batch::v1beta1::Job {
    fn convert_to(&self) -> Result<k8s_api::batch::v1::Job, ConversionError> {
        Ok(k8s_api::batch::v1::Job {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1", "Job"),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_job_spec_to_v1(s)).transpose()?,
            status: self.status.as_ref().map(|s| convert_job_status_to_v1(s)),
        })
    }

    fn convert_from(other: &k8s_api::batch::v1::Job) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1beta1", "Job"),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_job_spec_from_v1(s)).transpose()?,
            status: other.status.as_ref().map(|s| convert_job_status_from_v1(s)),
        })
    }
}

// =============================================================================
// JobList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::batch::v1::JobList> for k8s_api::batch::v1beta1::JobList {
    fn convert_to(&self) -> Result<k8s_api::batch::v1::JobList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::batch::v1::JobList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1", "JobList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::batch::v1::JobList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::batch::v1beta1::Job::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1beta1", "JobList"),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_job_spec_to_v1(
    spec: &k8s_api::batch::v1beta1::JobSpec,
) -> Result<k8s_api::batch::v1::JobSpec, ConversionError> {
    Ok(k8s_api::batch::v1::JobSpec {
        parallelism: spec.parallelism,
        completions: spec.completions,
        active_deadline_seconds: spec.active_deadline_seconds,
        pod_failure_policy: None, // v1beta1 doesn't have this
        success_policy: None,     // v1beta1 doesn't have this
        backoff_limit: spec.backoff_limit,
        backoff_limit_per_index: None,
        max_failed_indexes: None,
        selector: spec.selector.clone(),
        manual_selector: spec.manual_selector,
        template: spec.template.clone(),
        ttl_seconds_after_finished: spec.ttl_seconds_after_finished,
        completion_mode: None, // v1beta1 doesn't have this
        suspend: None,         // v1beta1 doesn't have this
        pod_replacement_policy: None,
        managed_by: None,
    })
}

fn convert_job_spec_from_v1(
    spec: &k8s_api::batch::v1::JobSpec,
) -> Result<k8s_api::batch::v1beta1::JobSpec, ConversionError> {
    Ok(k8s_api::batch::v1beta1::JobSpec {
        parallelism: spec.parallelism,
        completions: spec.completions,
        active_deadline_seconds: spec.active_deadline_seconds,
        backoff_limit: spec.backoff_limit,
        selector: spec.selector.clone(),
        manual_selector: spec.manual_selector,
        template: spec.template.clone(),
        ttl_seconds_after_finished: spec.ttl_seconds_after_finished,
    })
}

fn convert_job_status_to_v1(
    status: &k8s_api::batch::v1beta1::JobStatus,
) -> k8s_api::batch::v1::JobStatus {
    k8s_api::batch::v1::JobStatus {
        conditions: status.conditions.iter().map(|c| {
            k8s_api::batch::v1::JobCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_probe_time: c.last_probe_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
        start_time: status.start_time.clone(),
        completion_time: status.completion_time.clone(),
        active: status.active,
        succeeded: status.succeeded,
        failed: status.failed,
        terminating: None,
        completed_indexes: None,
        failed_indexes: None,
        uncounted_terminated_pods: None,
        ready: None,
    }
}

fn convert_job_status_from_v1(
    status: &k8s_api::batch::v1::JobStatus,
) -> k8s_api::batch::v1beta1::JobStatus {
    k8s_api::batch::v1beta1::JobStatus {
        conditions: status.conditions.iter().map(|c| {
            k8s_api::batch::v1beta1::JobCondition {
                condition_type: c.condition_type.clone(),
                status: c.status.clone(),
                last_probe_time: c.last_probe_time.clone(),
                last_transition_time: c.last_transition_time.clone(),
                reason: c.reason.clone(),
                message: c.message.clone(),
            }
        }).collect(),
        start_time: status.start_time.clone(),
        completion_time: status.completion_time.clone(),
        active: status.active,
        succeeded: status.succeeded,
        failed: status.failed,
    }
}

// =============================================================================
// CronJob: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::batch::v1::CronJob> for k8s_api::batch::v1beta1::CronJob {
    fn convert_to(&self) -> Result<k8s_api::batch::v1::CronJob, ConversionError> {
        Ok(k8s_api::batch::v1::CronJob {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1", "CronJob"),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_cronjob_spec_to_v1(s)).transpose()?,
            status: self.status.as_ref().map(|s| convert_cronjob_status_to_v1(s)),
        })
    }

    fn convert_from(other: &k8s_api::batch::v1::CronJob) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1beta1", "CronJob"),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_cronjob_spec_from_v1(s)).transpose()?,
            status: other.status.as_ref().map(|s| convert_cronjob_status_from_v1(s)),
        })
    }
}

// =============================================================================
// CronJobList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::batch::v1::CronJobList> for k8s_api::batch::v1beta1::CronJobList {
    fn convert_to(&self) -> Result<k8s_api::batch::v1::CronJobList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::batch::v1::CronJobList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("batch/v1", "CronJobList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::batch::v1::CronJobList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::batch::v1beta1::CronJob::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "batch/v1beta1",
                "CronJobList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_cronjob_spec_to_v1(
    spec: &k8s_api::batch::v1beta1::CronJobSpec,
) -> Result<k8s_api::batch::v1::CronJobSpec, ConversionError> {
    Ok(k8s_api::batch::v1::CronJobSpec {
        schedule: spec.schedule.clone(),
        time_zone: None, // v1beta1 doesn't have this
        starting_deadline_seconds: spec.starting_deadline_seconds,
        concurrency_policy: spec.concurrency_policy.clone(),
        suspend: spec.suspend,
        job_template: k8s_api::batch::v1::JobTemplateSpec {
            metadata: spec.job_template.metadata.clone(),
            spec: spec.job_template.spec.as_ref().map(|s| convert_job_spec_to_v1(s)).transpose()?,
        },
        successful_jobs_history_limit: spec.successful_jobs_history_limit,
        failed_jobs_history_limit: spec.failed_jobs_history_limit,
    })
}

fn convert_cronjob_spec_from_v1(
    spec: &k8s_api::batch::v1::CronJobSpec,
) -> Result<k8s_api::batch::v1beta1::CronJobSpec, ConversionError> {
    Ok(k8s_api::batch::v1beta1::CronJobSpec {
        schedule: spec.schedule.clone(),
        starting_deadline_seconds: spec.starting_deadline_seconds,
        concurrency_policy: spec.concurrency_policy.clone(),
        suspend: spec.suspend,
        job_template: k8s_api::batch::v1beta1::JobTemplateSpec {
            metadata: spec.job_template.metadata.clone(),
            spec: spec.job_template.spec.as_ref().map(|s| convert_job_spec_from_v1(s)).transpose()?,
        },
        successful_jobs_history_limit: spec.successful_jobs_history_limit,
        failed_jobs_history_limit: spec.failed_jobs_history_limit,
    })
}

fn convert_cronjob_status_to_v1(
    status: &k8s_api::batch::v1beta1::CronJobStatus,
) -> k8s_api::batch::v1::CronJobStatus {
    k8s_api::batch::v1::CronJobStatus {
        active: status.active.iter().map(|r| k8s_api::core::v1::ObjectReference {
            api_version: r.api_version.clone(),
            kind: r.kind.clone(),
            name: r.name.clone(),
            namespace: r.namespace.clone(),
            uid: r.uid.clone(),
            resource_version: r.resource_version.clone(),
            field_path: r.field_path.clone(),
        }).collect(),
        last_schedule_time: status.last_schedule_time.clone(),
        last_successful_time: None, // v1beta1 doesn't have this
    }
}

fn convert_cronjob_status_from_v1(
    status: &k8s_api::batch::v1::CronJobStatus,
) -> k8s_api::batch::v1beta1::CronJobStatus {
    k8s_api::batch::v1beta1::CronJobStatus {
        active: status.active.iter().map(|r| k8s_api::core::v1::ObjectReference {
            api_version: r.api_version.clone(),
            kind: r.kind.clone(),
            name: r.name.clone(),
            namespace: r.namespace.clone(),
            uid: r.uid.clone(),
            resource_version: r.resource_version.clone(),
            field_path: r.field_path.clone(),
        }).collect(),
        last_schedule_time: status.last_schedule_time.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_conversion_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta1_job = k8s_api::batch::v1beta1::Job {
            metadata: ObjectMeta::named("test-job"),
            spec: Some(k8s_api::batch::v1beta1::JobSpec {
                parallelism: Some(2),
                completions: Some(5),
                backoff_limit: Some(3),
                template: k8s_api::core::v1::PodTemplateSpec {
                    spec: Some(k8s_api::core::v1::PodSpec {
                        containers: vec![k8s_api::core::v1::Container::new("worker", "busybox")],
                        restart_policy: "Never".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_job: k8s_api::batch::v1::Job = v1beta1_job.convert_to().unwrap();
        assert_eq!(v1_job.metadata.name, "test-job");
        assert_eq!(v1_job.spec.as_ref().unwrap().parallelism, Some(2));

        // Convert back to v1beta1
        let converted_back: k8s_api::batch::v1beta1::Job =
            k8s_api::batch::v1beta1::Job::convert_from(&v1_job).unwrap();
        assert_eq!(converted_back.metadata.name, "test-job");
        assert_eq!(converted_back.spec.as_ref().unwrap().completions, Some(5));
    }

    #[test]
    fn test_cronjob_conversion_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta1_cronjob = k8s_api::batch::v1beta1::CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: Some(k8s_api::batch::v1beta1::CronJobSpec {
                schedule: "*/5 * * * *".to_string(),
                concurrency_policy: "Forbid".to_string(),
                job_template: k8s_api::batch::v1beta1::JobTemplateSpec {
                    spec: Some(k8s_api::batch::v1beta1::JobSpec {
                        template: k8s_api::core::v1::PodTemplateSpec {
                            spec: Some(k8s_api::core::v1::PodSpec {
                                containers: vec![k8s_api::core::v1::Container::new("cron", "busybox")],
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

        // Convert to v1
        let v1_cronjob: k8s_api::batch::v1::CronJob = v1beta1_cronjob.convert_to().unwrap();
        assert_eq!(v1_cronjob.metadata.name, "test-cronjob");
        assert_eq!(v1_cronjob.spec.as_ref().unwrap().schedule, "*/5 * * * *");

        // Convert back to v1beta1
        let converted_back: k8s_api::batch::v1beta1::CronJob =
            k8s_api::batch::v1beta1::CronJob::convert_from(&v1_cronjob).unwrap();
        assert_eq!(converted_back.metadata.name, "test-cronjob");
        assert_eq!(
            converted_back.spec.as_ref().unwrap().concurrency_policy,
            "Forbid"
        );
    }

    #[test]
    fn test_job_list_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

        let list = k8s_api::batch::v1beta1::JobList {
            metadata: ListMeta {
                resource_version: "11".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::batch::v1beta1::Job {
                metadata: ObjectMeta::named("job-list"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::batch::v1::JobList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "11");
        assert_eq!(v1_list.items[0].metadata.name, "job-list");

        let roundtrip: k8s_api::batch::v1beta1::JobList =
            k8s_api::batch::v1beta1::JobList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
