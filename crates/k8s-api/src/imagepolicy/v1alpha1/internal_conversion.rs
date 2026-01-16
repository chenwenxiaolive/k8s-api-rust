use super::*;

impl InternalConversion for ImageReview {
    type Internal = crate::imagepolicy::internal::ImageReview;
}

impl InternalConversion for ImageReviewSpec {
    type Internal = crate::imagepolicy::internal::ImageReviewSpec;
}

impl InternalConversion for ImageReviewContainerSpec {
    type Internal = crate::imagepolicy::internal::ImageReviewContainerSpec;
}

impl InternalConversion for ImageReviewStatus {
    type Internal = crate::imagepolicy::internal::ImageReviewStatus;
}
