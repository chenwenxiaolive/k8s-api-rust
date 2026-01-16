use super::*;

impl InternalConversion for ClusterTrustBundle {
    type Internal = crate::certificates::internal::ClusterTrustBundle;
}

impl InternalConversion for ClusterTrustBundleList {
    type Internal = crate::certificates::internal::ClusterTrustBundleList;
}

impl InternalConversion for ClusterTrustBundleSpec {
    type Internal = crate::certificates::internal::ClusterTrustBundleSpec;
}

impl InternalConversion for PodCertificateRequest {
    type Internal = crate::certificates::internal::PodCertificateRequest;
}

impl InternalConversion for PodCertificateRequestList {
    type Internal = crate::certificates::internal::PodCertificateRequestList;
}

impl InternalConversion for PodCertificateRequestSpec {
    type Internal = crate::certificates::internal::PodCertificateRequestSpec;
}

impl InternalConversion for PodCertificateRequestStatus {
    type Internal = crate::certificates::internal::PodCertificateRequestStatus;
}
