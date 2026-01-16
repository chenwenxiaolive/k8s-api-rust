use super::*;

impl InternalConversion for Job {
    type Internal = crate::batch::internal::Job;
}

impl InternalConversion for JobSpec {
    type Internal = crate::batch::internal::JobSpec;
}

impl InternalConversion for PodFailurePolicy {
    type Internal = crate::batch::internal::PodFailurePolicy;
}

impl InternalConversion for SuccessPolicy {
    type Internal = crate::batch::internal::SuccessPolicy;
}

impl InternalConversion for SuccessPolicyRule {
    type Internal = crate::batch::internal::SuccessPolicyRule;
}

impl InternalConversion for PodFailurePolicyRule {
    type Internal = crate::batch::internal::PodFailurePolicyRule;
}

impl InternalConversion for PodFailurePolicyOnExitCodesRequirement {
    type Internal = crate::batch::internal::PodFailurePolicyOnExitCodesRequirement;
}

impl InternalConversion for PodFailurePolicyOnPodConditionsPattern {
    type Internal = crate::batch::internal::PodFailurePolicyOnPodConditionsPattern;
}

impl InternalConversion for JobStatus {
    type Internal = crate::batch::internal::JobStatus;
}

impl InternalConversion for JobCondition {
    type Internal = crate::batch::internal::JobCondition;
}

impl InternalConversion for UncountedTerminatedPods {
    type Internal = crate::batch::internal::UncountedTerminatedPods;
}

impl InternalConversion for JobList {
    type Internal = crate::batch::internal::JobList;
}

impl InternalConversion for CronJob {
    type Internal = crate::batch::internal::CronJob;
}

impl InternalConversion for CronJobSpec {
    type Internal = crate::batch::internal::CronJobSpec;
}

impl InternalConversion for JobTemplateSpec {
    type Internal = crate::batch::internal::JobTemplateSpec;
}

impl InternalConversion for CronJobStatus {
    type Internal = crate::batch::internal::CronJobStatus;
}

impl InternalConversion for CronJobList {
    type Internal = crate::batch::internal::CronJobList;
}
