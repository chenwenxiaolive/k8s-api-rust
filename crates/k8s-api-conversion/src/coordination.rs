//! Coordination API conversions
//!
//! This module provides conversions between coordination API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// Lease: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::coordination::v1::Lease> for k8s_api::coordination::v1beta1::Lease {
    fn convert_to(&self) -> Result<k8s_api::coordination::v1::Lease, ConversionError> {
        let mut converted: k8s_api::coordination::v1::Lease = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "coordination.k8s.io/v1",
            "Lease",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::coordination::v1::Lease) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::coordination::v1beta1::Lease = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "coordination.k8s.io/v1beta1",
            "Lease",
        );
        Ok(converted)
    }
}

// =============================================================================
// LeaseList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::coordination::v1::LeaseList>
    for k8s_api::coordination::v1beta1::LeaseList
{
    fn convert_to(&self) -> Result<k8s_api::coordination::v1::LeaseList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::coordination::v1::LeaseList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "coordination.k8s.io/v1",
                "LeaseList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::coordination::v1::LeaseList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::coordination::v1beta1::Lease::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "coordination.k8s.io/v1beta1",
                "LeaseList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// LeaseCandidate: v1alpha2 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::coordination::v1beta1::LeaseCandidate>
    for k8s_api::coordination::v1alpha2::LeaseCandidate
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::coordination::v1beta1::LeaseCandidate, ConversionError> {
        let mut converted: k8s_api::coordination::v1beta1::LeaseCandidate =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "coordination.k8s.io/v1beta1",
            "LeaseCandidate",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::coordination::v1beta1::LeaseCandidate,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::coordination::v1alpha2::LeaseCandidate =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "coordination.k8s.io/v1alpha2",
            "LeaseCandidate",
        );
        Ok(converted)
    }
}

// =============================================================================
// LeaseCandidateList: v1alpha2 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::coordination::v1beta1::LeaseCandidateList>
    for k8s_api::coordination::v1alpha2::LeaseCandidateList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::coordination::v1beta1::LeaseCandidateList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::coordination::v1beta1::LeaseCandidateList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "coordination.k8s.io/v1beta1",
                "LeaseCandidateList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::coordination::v1beta1::LeaseCandidateList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::coordination::v1alpha2::LeaseCandidate::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "coordination.k8s.io/v1alpha2",
                "LeaseCandidateList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}
