use super::*;

impl InternalConversion for SubjectAccessReview {
    type Internal = crate::authorization::internal::SubjectAccessReview;
}

impl InternalConversion for SubjectAccessReviewSpec {
    type Internal = crate::authorization::internal::SubjectAccessReviewSpec;
}

impl InternalConversion for ResourceAttributes {
    type Internal = crate::authorization::internal::ResourceAttributes;
}

impl InternalConversion for NonResourceAttributes {
    type Internal = crate::authorization::internal::NonResourceAttributes;
}

impl InternalConversion for SubjectAccessReviewStatus {
    type Internal = crate::authorization::internal::SubjectAccessReviewStatus;
}

impl InternalConversion for SelfSubjectAccessReview {
    type Internal = crate::authorization::internal::SelfSubjectAccessReview;
}

impl InternalConversion for SelfSubjectAccessReviewSpec {
    type Internal = crate::authorization::internal::SelfSubjectAccessReviewSpec;
}

impl InternalConversion for LocalSubjectAccessReview {
    type Internal = crate::authorization::internal::LocalSubjectAccessReview;
}

impl InternalConversion for SelfSubjectRulesReview {
    type Internal = crate::authorization::internal::SelfSubjectRulesReview;
}

impl InternalConversion for SelfSubjectRulesReviewSpec {
    type Internal = crate::authorization::internal::SelfSubjectRulesReviewSpec;
}

impl InternalConversion for SubjectRulesReviewStatus {
    type Internal = crate::authorization::internal::SubjectRulesReviewStatus;
}

impl InternalConversion for ResourceRule {
    type Internal = crate::authorization::internal::ResourceRule;
}

impl InternalConversion for NonResourceRule {
    type Internal = crate::authorization::internal::NonResourceRule;
}
