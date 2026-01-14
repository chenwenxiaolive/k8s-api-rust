//! Certificates v1alpha1 type definitions
//!
//! This module provides alpha-level certificate types including ClusterTrustBundle
//! and PodCertificateRequest.

use k8s_apimachinery::apis::meta::v1::{Condition, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// ClusterTrustBundle (K8s 1.26+)
// =============================================================================

/// ClusterTrustBundle is a cluster-scoped container for X.509 trust anchors (root certificates).
///
/// ClusterTrustBundle objects are considered to be readable by any authenticated user in the cluster,
/// because they can be mounted by pods using the `clusterTrustBundle` projection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundle {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec contains the signer (if any) and trust anchors.
    pub spec: ClusterTrustBundleSpec,
}

/// ClusterTrustBundleList is a collection of ClusterTrustBundle objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    /// Items is a collection of ClusterTrustBundle objects.
    pub items: Vec<ClusterTrustBundle>,
}

/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleSpec {
    /// SignerName indicates the associated signer, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,
    /// TrustBundle contains the individual X.509 trust anchors for this bundle,
    /// as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.
    pub trust_bundle: String,
}

// =============================================================================
// PodCertificateRequest (K8s 1.34+)
// =============================================================================

/// PodCertificateRequest encodes a pod requesting a certificate from a given signer.
///
/// Kubelets use this API to implement podCertificate projected volumes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequest {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec contains the details about the certificate being requested.
    pub spec: PodCertificateRequestSpec,
    /// Status contains the issued certificate, and a standard set of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodCertificateRequestStatus>,
}

/// PodCertificateRequestList is a collection of PodCertificateRequest objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    /// Items is a collection of PodCertificateRequest objects.
    pub items: Vec<PodCertificateRequest>,
}

/// PodCertificateRequestSpec describes the certificate request. All fields are immutable after creation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestSpec {
    /// SignerName indicates the requested signer.
    pub signer_name: String,
    /// PodName is the name of the pod into which the certificate will be mounted.
    pub pod_name: String,
    /// PodUID is the UID of the pod into which the certificate will be mounted.
    pub pod_uid: String,
    /// ServiceAccountName is the name of the service account the pod is running as.
    pub service_account_name: String,
    /// ServiceAccountUID is the UID of the service account the pod is running as.
    pub service_account_uid: String,
    /// NodeName is the name of the node the pod is assigned to.
    pub node_name: String,
    /// NodeUID is the UID of the node the pod is assigned to.
    pub node_uid: String,
    /// MaxExpirationSeconds is the maximum lifetime permitted for the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    /// PKIXPublicKey is the PKIX-serialized public key the signer will issue the certificate to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pkix_public_key: Vec<u8>,
    /// ProofOfPossession proves that the requesting kubelet holds the private key.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proof_of_possession: Vec<u8>,
}

/// PodCertificateRequestStatus describes the status of the request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestStatus {
    /// Conditions applied to the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    /// CertificateChain is populated with an issued certificate by the signer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub certificate_chain: String,
    /// NotBefore is the time at which the certificate becomes valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// BeginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin_refresh_at: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// NotAfter is the time at which the certificate expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<k8s_apimachinery::apis::meta::v1::Time>,
}

// Well-known condition types for PodCertificateRequests
pub const CONDITION_TYPE_DENIED: &str = "Denied";
pub const CONDITION_TYPE_FAILED: &str = "Failed";
pub const CONDITION_TYPE_ISSUED: &str = "Issued";

// Well-known condition reasons
pub const CONDITION_UNSUPPORTED_KEY_TYPE: &str = "UnsupportedKeyType";
