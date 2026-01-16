use super::*;

impl InternalConversion for CertificateSigningRequest {
    type Internal = crate::certificates::internal::CertificateSigningRequest;
}

impl InternalConversion for CertificateSigningRequestList {
    type Internal = crate::certificates::internal::CertificateSigningRequestList;
}

impl InternalConversion for CertificateSigningRequestSpec {
    type Internal = crate::certificates::internal::CertificateSigningRequestSpec;
}

impl InternalConversion for CertificateSigningRequestStatus {
    type Internal = crate::certificates::internal::CertificateSigningRequestStatus;
}

impl InternalConversion for CertificateSigningRequestCondition {
    type Internal = crate::certificates::internal::CertificateSigningRequestCondition;
}
