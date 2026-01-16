use super::*;

impl InternalConversion for TokenReview {
    type Internal = crate::authentication::internal::TokenReview;
}

impl InternalConversion for TokenReviewSpec {
    type Internal = crate::authentication::internal::TokenReviewSpec;
}

impl InternalConversion for TokenReviewStatus {
    type Internal = crate::authentication::internal::TokenReviewStatus;
}

impl InternalConversion for UserInfo {
    type Internal = crate::authentication::internal::UserInfo;
}

impl InternalConversion for SelfSubjectReview {
    type Internal = crate::authentication::internal::SelfSubjectReview;
}

impl InternalConversion for SelfSubjectReviewStatus {
    type Internal = crate::authentication::internal::SelfSubjectReviewStatus;
}
