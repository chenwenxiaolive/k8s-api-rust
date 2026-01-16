use super::*;

impl InternalConversion for AdmissionReview {
    type Internal = crate::admission::internal::AdmissionReview;
}

impl InternalConversion for AdmissionRequest {
    type Internal = crate::admission::internal::AdmissionRequest;
}

impl InternalConversion for AdmissionResponse {
    type Internal = crate::admission::internal::AdmissionResponse;
}
