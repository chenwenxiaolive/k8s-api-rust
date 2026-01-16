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

impl InternalConversion for TokenRequest {
    type Internal = crate::authentication::internal::TokenRequest;
}

impl InternalConversion for TokenRequestSpec {
    type Internal = crate::authentication::internal::TokenRequestSpec;
}

impl InternalConversion for TokenRequestStatus {
    type Internal = crate::authentication::internal::TokenRequestStatus;
}

impl InternalConversion for BoundObjectReference {
    type Internal = crate::authentication::internal::BoundObjectReference;
}

impl InternalConversion for SelfSubjectReview {
    type Internal = crate::authentication::internal::SelfSubjectReview;
}

impl InternalConversion for SelfSubjectReviewStatus {
    type Internal = crate::authentication::internal::SelfSubjectReviewStatus;
}
