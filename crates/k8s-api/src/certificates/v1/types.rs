//! Certificates v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ExtraValue = Vec<String>;
pub type RequestConditionType = String;
pub type KeyUsage = String;

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
    pub usages: Vec<KeyUsage>,
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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
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
    pub type_: RequestConditionType,
    /// Status of the condition.
    pub status: crate::core::v1::ConditionStatus,
    /// Reason indicates a brief reason for the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Message contains a human readable message with details about the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// LastUpdateTime is the time last update was made to this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<Time>,
    /// LastTransitionTime is the time the condition last transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
}

// RequestConditionType constants
pub const CERTIFICATE_APPROVED: &str = "Approved";
pub const CERTIFICATE_DENIED: &str = "Denied";
pub const CERTIFICATE_FAILED: &str = "Failed";

// KeyUsage constants
pub const KEY_USAGE_SIGNING: &str = "signing";
pub const KEY_USAGE_DIGITAL_SIGNATURE: &str = "digital signature";
pub const KEY_USAGE_CONTENT_COMMITMENT: &str = "content commitment";
pub const KEY_USAGE_KEY_ENCIPHERMENT: &str = "key encipherment";
pub const KEY_USAGE_KEY_AGREEMENT: &str = "key agreement";
pub const KEY_USAGE_DATA_ENCIPHERMENT: &str = "data encipherment";
pub const KEY_USAGE_CERT_SIGN: &str = "cert sign";
pub const KEY_USAGE_CRL_SIGN: &str = "crl sign";
pub const KEY_USAGE_ENCIPHER_ONLY: &str = "encipher only";
pub const KEY_USAGE_DECIPHER_ONLY: &str = "decipher only";
pub const KEY_USAGE_ANY: &str = "any";
pub const KEY_USAGE_SERVER_AUTH: &str = "server auth";
pub const KEY_USAGE_CLIENT_AUTH: &str = "client auth";
pub const KEY_USAGE_CODE_SIGNING: &str = "code signing";
pub const KEY_USAGE_EMAIL_PROTECTION: &str = "email protection";
pub const KEY_USAGE_SMIME: &str = "s/mime";
pub const KEY_USAGE_IPSEC_END_SYSTEM: &str = "ipsec end system";
pub const KEY_USAGE_IPSEC_TUNNEL: &str = "ipsec tunnel";
pub const KEY_USAGE_IPSEC_USER: &str = "ipsec user";
pub const KEY_USAGE_TIMESTAMPING: &str = "timestamping";
pub const KEY_USAGE_OCSP_SIGNING: &str = "ocsp signing";
pub const KEY_USAGE_MICROSOFT_SGC: &str = "microsoft sgc";
pub const KEY_USAGE_NETSCAPE_SGC: &str = "netscape sgc";
