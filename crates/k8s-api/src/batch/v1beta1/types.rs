//! Batch v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

use crate::batch::v1::JobSpec;

pub type ConcurrencyPolicy = String;

// =============================================================================
// CronJob
// =============================================================================

/// CronJob represents the configuration of a single cron job.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<CronJobSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CronJobStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    /// The schedule in Cron format.
    pub schedule: String,
    /// The time zone name for the given schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Optional deadline in seconds for starting the job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub concurrency_policy: ConcurrencyPolicy,
    /// This flag tells the controller to suspend subsequent executions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    pub job_template: JobTemplateSpec,
    /// The number of successful finished jobs to retain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    /// The number of failed finished jobs to retain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active: Vec<crate::core::v1::ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<Time>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<CronJob>,
}

// =============================================================================
// JobTemplateSpec
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplateSpec {
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<JobSpec>,
}

// ConcurrencyPolicy constants
pub const CONCURRENCY_POLICY_ALLOW: &str = "Allow";
pub const CONCURRENCY_POLICY_FORBID: &str = "Forbid";
pub const CONCURRENCY_POLICY_REPLACE: &str = "Replace";
