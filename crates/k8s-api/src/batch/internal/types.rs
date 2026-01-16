//! Internal type definitions for batch.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, Time, TypeMeta, LabelSelector};
use serde::{Deserialize, Serialize};
use crate::core::v1::PodTemplateSpec;

pub type CompletionMode = String;
pub type ConcurrencyPolicy = String;
pub type JobConditionType = String;
pub type PodFailurePolicyAction = String;
pub type PodFailurePolicyOnExitCodesOperator = String;
pub type PodReplacementPolicy = String;

pub const COMPLETION_MODE_INDEXED: &str = "Indexed";
pub const COMPLETION_MODE_NON_INDEXED: &str = "NonIndexed";
pub const CONCURRENCY_POLICY_ALLOW: &str = "Allow";
pub const CONCURRENCY_POLICY_FORBID: &str = "Forbid";
pub const CONCURRENCY_POLICY_REPLACE: &str = "Replace";
pub const JOB_CONDITION_COMPLETE: &str = "Complete";
pub const JOB_CONDITION_FAILED: &str = "Failed";
pub const JOB_CONDITION_FAILURE_TARGET: &str = "FailureTarget";
pub const JOB_CONDITION_SUCCESS_CRITERIA_MET: &str = "SuccessCriteriaMet";
pub const JOB_CONDITION_SUSPENDED: &str = "Suspended";
pub const POD_FAILURE_POLICY_ACTION_COUNT: &str = "Count";
pub const POD_FAILURE_POLICY_ACTION_FAIL_INDEX: &str = "FailIndex";
pub const POD_FAILURE_POLICY_ACTION_FAIL_JOB: &str = "FailJob";
pub const POD_FAILURE_POLICY_ACTION_IGNORE: &str = "Ignore";
pub const POD_FAILURE_POLICY_ON_EXIT_CODES_OP_IN: &str = "In";
pub const POD_FAILURE_POLICY_ON_EXIT_CODES_OP_NOT_IN: &str = "NotIn";
pub const POD_REPLACEMENT_POLICY_FAILED: &str = "Failed";
pub const POD_REPLACEMENT_POLICY_TERMINATING_OR_FAILED: &str = "TerminatingOrFailed";


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


/// CronJobList is a collection of cron jobs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<CronJob>,
}


/// CronJobSpec describes how the job execution will look like and when it will actually run.
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


/// CronJobStatus represents the current state of a cron job.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active: Vec<crate::core::v1::ObjectReference>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<Time>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<Time>,
}


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
pub struct JobCondition {
    #[serde(rename = "type")]
    pub condition_type: JobConditionType,
    pub status: crate::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// JobList is a collection of jobs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Job>,
}


/// JobSpec describes how the job execution will look like.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should run at any given time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,

    /// Specifies the desired number of successfully finished pods the job should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,

    /// Specifies the duration in seconds relative to the startTime that the job may be active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    /// Specifies the policy of handling pod failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: Option<PodFailurePolicy>,

    /// Specifies the policy when the Job can be declared as succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_policy: Option<SuccessPolicy>,

    /// Specifies the number of retries before marking this job failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,

    /// Specifies the limit for the number of retries within an index before marking the index as failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: Option<i32>,

    /// Specifies the maximal number of failed indexes before marking the Job as failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: Option<i32>,

    /// A label query over pods that should match the pod count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// manualSelector controls generation of pod labels and pod selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,

    /// Describes the pod that will be created when executing a job.
    pub template: PodTemplateSpec,

    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,

    /// completionMode specifies how Pod completions are tracked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_mode: Option<CompletionMode>,

    /// suspend specifies whether the Job controller should create Pods or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,

    /// podReplacementPolicy specifies when to create replacement Pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: Option<PodReplacementPolicy>,

    /// ManagedBy field indicates the controller that manages a Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}


/// JobStatus represents the current state of a Job.
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_indexes: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_indexes: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncounted_terminated_pods: Option<UncountedTerminatedPods>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
}


/// JobTemplateSpec describes the data a Job should have when created from a template.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplateSpec {
    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<JobSpec>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicy {
    pub rules: Vec<PodFailurePolicyRule>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnExitCodesRequirement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    pub operator: PodFailurePolicyOnExitCodesOperator,
    pub values: Vec<i32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnPodConditionsPattern {
    #[serde(rename = "type")]
    pub condition_type: crate::core::v1::PodConditionType,
    pub status: crate::core::v1::ConditionStatus,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyRule {
    pub action: PodFailurePolicyAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exit_codes: Option<PodFailurePolicyOnExitCodesRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_pod_conditions: Vec<PodFailurePolicyOnPodConditionsPattern>,
}


/// SuccessPolicy describes when a Job can be declared as succeeded.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicy {
    pub rules: Vec<SuccessPolicyRule>,
}


/// SuccessPolicyRule describes rule for declaring a Job as succeeded.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_indexes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UncountedTerminatedPods {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub succeeded: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<String>,
}
