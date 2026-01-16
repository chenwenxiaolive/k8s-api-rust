use super::*;

impl InternalConversion for SelfSubjectReview {
    type Internal = crate::authentication::internal::SelfSubjectReview;
}

impl InternalConversion for SelfSubjectReviewStatus {
    type Internal = crate::authentication::internal::SelfSubjectReviewStatus;
}
