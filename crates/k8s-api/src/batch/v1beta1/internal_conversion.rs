use super::*;

impl InternalConversion for CronJob {
    type Internal = crate::batch::internal::CronJob;
}

impl InternalConversion for CronJobSpec {
    type Internal = crate::batch::internal::CronJobSpec;
}

impl InternalConversion for CronJobStatus {
    type Internal = crate::batch::internal::CronJobStatus;
}

impl InternalConversion for CronJobList {
    type Internal = crate::batch::internal::CronJobList;
}

impl InternalConversion for JobTemplateSpec {
    type Internal = crate::batch::internal::JobTemplateSpec;
}
