//! Batch API conversions
//!
//! This module provides conversions between batch API versions.

use crate::{ConversionError, Convertible};

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
        time_zone: spec.time_zone.clone(),
        starting_deadline_seconds: spec.starting_deadline_seconds,
        concurrency_policy: spec.concurrency_policy.clone(),
        suspend: spec.suspend,
        job_template: k8s_api::batch::v1::JobTemplateSpec {
            metadata: spec.job_template.metadata.clone(),
            spec: spec.job_template.spec.clone(),
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
        time_zone: spec.time_zone.clone(),
        starting_deadline_seconds: spec.starting_deadline_seconds,
        concurrency_policy: spec.concurrency_policy.clone(),
        suspend: spec.suspend,
        job_template: k8s_api::batch::v1beta1::JobTemplateSpec {
            metadata: spec.job_template.metadata.clone(),
            spec: spec.job_template.spec.clone(),
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
    fn test_cronjob_conversion_roundtrip() {
        use k8s_apimachinery::apis::meta::v1::ObjectMeta;

        let v1beta1_cronjob = k8s_api::batch::v1beta1::CronJob {
            metadata: ObjectMeta::named("test-cronjob"),
            spec: Some(k8s_api::batch::v1beta1::CronJobSpec {
                schedule: "*/5 * * * *".to_string(),
                time_zone: Some("UTC".to_string()),
                concurrency_policy: "Forbid".to_string(),
                job_template: k8s_api::batch::v1beta1::JobTemplateSpec {
                    spec: Some(k8s_api::batch::v1::JobSpec {
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
}
