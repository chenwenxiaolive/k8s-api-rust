//! Certificates v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// CertificateSigningRequest
// =============================================================================

/// CertificateSigningRequest objects provide a mechanism to obtain x509 certificates by submitting a certificate signing request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequest {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: CertificateSigningRequestSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateSigningRequestStatus>,
}

/// CertificateSigningRequestList is a collection of CertificateSigningRequest objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<CertificateSigningRequest>,
}

/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestSpec {
    /// Request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block.
    pub request: String,
    /// SignerName indicates the requested signer.
    pub signer_name: String,
    /// ExpirationSeconds is the requested duration of validity of the issued certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i32>,
    /// Usages specifies a set of key usages requested in the issued certificate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<String>,
    /// Username contains the name of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// UID contains the uid of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// Groups contains group membership of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Extra contains extra attributes of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub extra: std::collections::BTreeMap<String, Vec<String>>,
}

/// CertificateSigningRequestStatus contains conditions used to indicate approved/denied/failed status of the request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestStatus {
    /// Conditions applied to the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<CertificateSigningRequestCondition>,
    /// Certificate is populated with an issued certificate by the signer after an Approved condition is present.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub certificate: String,
}

/// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestCondition {
    /// Type of the condition.
    #[serde(rename = "type")]
    pub type_: String,
    /// Status of the condition.
    pub status: String,
    /// Reason indicates a brief reason for the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Message contains a human readable message with details about the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// LastUpdateTime is the time last update was made to this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    /// LastTransitionTime is the time the condition last transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
}
