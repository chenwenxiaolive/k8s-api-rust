//! Batch v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

use crate::core::v1::PodTemplateSpec;

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
    /// Optional deadline in seconds for starting the job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub concurrency_policy: String,
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

// =============================================================================
// Job
// =============================================================================

/// Job represents the configuration of a single job.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<JobSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should run at any given time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    /// Specifies the desired number of successfully finished pods the job should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    /// Specifies the duration in seconds relative to the startTime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Specifies the number of retries before marking this job failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    /// A label query over pods that should match the pod count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// manualSelector controls generation of pod labels and pod selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    /// Describes the pod that will be created when executing a job.
    pub template: PodTemplateSpec,
    /// ttlSecondsAfterFinished limits the lifetime of a Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<JobCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<Job>,
}
