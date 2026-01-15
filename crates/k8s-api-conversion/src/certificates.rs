//! Certificates API conversions
//!
//! This module provides conversions between certificates API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};
use chrono::{DateTime, Utc};

// =============================================================================
// CertificateSigningRequest: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::certificates::v1::CertificateSigningRequest>
    for k8s_api::certificates::v1beta1::CertificateSigningRequest
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::certificates::v1::CertificateSigningRequest, ConversionError> {
        Ok(k8s_api::certificates::v1::CertificateSigningRequest {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1",
                "CertificateSigningRequest",
            ),
            metadata: self.metadata.clone(),
            spec: convert_csr_spec_to_v1(&self.spec)?,
            status: self.status.as_ref().map(convert_csr_status_to_v1),
        })
    }

    fn convert_from(
        other: &k8s_api::certificates::v1::CertificateSigningRequest,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1beta1",
                "CertificateSigningRequest",
            ),
            metadata: other.metadata.clone(),
            spec: convert_csr_spec_from_v1(&other.spec),
            status: other.status.as_ref().map(convert_csr_status_from_v1),
        })
    }
}

// =============================================================================
// CertificateSigningRequestList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::certificates::v1::CertificateSigningRequestList>
    for k8s_api::certificates::v1beta1::CertificateSigningRequestList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::certificates::v1::CertificateSigningRequestList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::certificates::v1::CertificateSigningRequestList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1",
                "CertificateSigningRequestList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::certificates::v1::CertificateSigningRequestList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::certificates::v1beta1::CertificateSigningRequest::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1beta1",
                "CertificateSigningRequestList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_csr_spec_to_v1(
    spec: &k8s_api::certificates::v1beta1::CertificateSigningRequestSpec,
) -> Result<k8s_api::certificates::v1::CertificateSigningRequestSpec, ConversionError> {
    let signer_name = spec
        .signer_name
        .clone()
        .ok_or_else(|| ConversionError::MissingField("spec.signerName".to_string()))?;

    Ok(k8s_api::certificates::v1::CertificateSigningRequestSpec {
        request: spec.request.clone(),
        signer_name,
        expiration_seconds: spec.expiration_seconds,
        usages: spec.usages.clone(),
        username: spec.username.clone(),
        uid: spec.uid.clone(),
        groups: spec.groups.clone(),
        extra: spec.extra.clone(),
    })
}

fn convert_csr_spec_from_v1(
    spec: &k8s_api::certificates::v1::CertificateSigningRequestSpec,
) -> k8s_api::certificates::v1beta1::CertificateSigningRequestSpec {
    k8s_api::certificates::v1beta1::CertificateSigningRequestSpec {
        request: spec.request.clone(),
        signer_name: if spec.signer_name.is_empty() {
            None
        } else {
            Some(spec.signer_name.clone())
        },
        expiration_seconds: spec.expiration_seconds,
        usages: spec.usages.clone(),
        username: spec.username.clone(),
        uid: spec.uid.clone(),
        groups: spec.groups.clone(),
        extra: spec.extra.clone(),
    }
}

fn convert_csr_status_to_v1(
    status: &k8s_api::certificates::v1beta1::CertificateSigningRequestStatus,
) -> k8s_api::certificates::v1::CertificateSigningRequestStatus {
    k8s_api::certificates::v1::CertificateSigningRequestStatus {
        conditions: status
            .conditions
            .iter()
            .map(|condition| k8s_api::certificates::v1::CertificateSigningRequestCondition {
                type_: condition.type_.clone(),
                status: condition.status.clone(),
                reason: condition.reason.clone(),
                message: condition.message.clone(),
                last_update_time: parse_time(&condition.last_update_time),
                last_transition_time: parse_time(&condition.last_transition_time),
            })
            .collect(),
        certificate: status.certificate.clone(),
    }
}

fn convert_csr_status_from_v1(
    status: &k8s_api::certificates::v1::CertificateSigningRequestStatus,
) -> k8s_api::certificates::v1beta1::CertificateSigningRequestStatus {
    k8s_api::certificates::v1beta1::CertificateSigningRequestStatus {
        conditions: status
            .conditions
            .iter()
            .map(|condition| {
                k8s_api::certificates::v1beta1::CertificateSigningRequestCondition {
                    type_: condition.type_.clone(),
                    status: condition.status.clone(),
                    reason: condition.reason.clone(),
                    message: condition.message.clone(),
                    last_update_time: format_time(&condition.last_update_time),
                    last_transition_time: format_time(&condition.last_transition_time),
                }
            })
            .collect(),
        certificate: status.certificate.clone(),
    }
}

fn parse_time(time: &Option<String>) -> Option<k8s_apimachinery::apis::meta::v1::Time> {
    time.as_ref().and_then(|value| {
        if value.is_empty() {
            return None;
        }
        DateTime::parse_from_rfc3339(value)
            .ok()
            .map(|dt| k8s_apimachinery::apis::meta::v1::Time(Some(dt.with_timezone(&Utc))))
    })
}

fn format_time(time: &Option<k8s_apimachinery::apis::meta::v1::Time>) -> Option<String> {
    time.as_ref().and_then(|t| t.0.as_ref().map(|dt| dt.to_rfc3339()))
}

// =============================================================================
// ClusterTrustBundle: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::certificates::v1beta1::ClusterTrustBundle>
    for k8s_api::certificates::v1alpha1::ClusterTrustBundle
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::certificates::v1beta1::ClusterTrustBundle, ConversionError> {
        let mut converted: k8s_api::certificates::v1beta1::ClusterTrustBundle =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "certificates.k8s.io/v1beta1",
            "ClusterTrustBundle",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::certificates::v1beta1::ClusterTrustBundle,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::certificates::v1alpha1::ClusterTrustBundle =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "certificates.k8s.io/v1alpha1",
            "ClusterTrustBundle",
        );
        Ok(converted)
    }
}

// =============================================================================
// ClusterTrustBundleList: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::certificates::v1beta1::ClusterTrustBundleList>
    for k8s_api::certificates::v1alpha1::ClusterTrustBundleList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::certificates::v1beta1::ClusterTrustBundleList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::certificates::v1beta1::ClusterTrustBundleList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1beta1",
                "ClusterTrustBundleList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::certificates::v1beta1::ClusterTrustBundleList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::certificates::v1alpha1::ClusterTrustBundle::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "certificates.k8s.io/v1alpha1",
                "ClusterTrustBundleList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_cluster_trust_bundle_alpha_to_beta() {
        let alpha = k8s_api::certificates::v1alpha1::ClusterTrustBundle {
            metadata: ObjectMeta::named("trust-bundle"),
            spec: k8s_api::certificates::v1alpha1::ClusterTrustBundleSpec {
                signer_name: "example.com/signer".to_string(),
                trust_bundle: "-----BEGIN CERTIFICATE-----\n...".to_string(),
            },
            ..Default::default()
        };

        let beta: k8s_api::certificates::v1beta1::ClusterTrustBundle =
            alpha.convert_to().unwrap();
        assert_eq!(beta.metadata.name, "trust-bundle");
        assert_eq!(beta.spec.signer_name, "example.com/signer");
    }
}
