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

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    #[test]
    fn test_lease_list_roundtrip() {
        let list = k8s_api::coordination::v1beta1::LeaseList {
            metadata: ListMeta {
                resource_version: "5".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::coordination::v1beta1::Lease {
                metadata: ObjectMeta::named("lease"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::coordination::v1::LeaseList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "5");
        assert_eq!(v1_list.items[0].metadata.name, "lease");

        let roundtrip: k8s_api::coordination::v1beta1::LeaseList =
            k8s_api::coordination::v1beta1::LeaseList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }

    #[test]
    fn test_lease_candidate_list_empty() {
        let list = k8s_api::coordination::v1alpha2::LeaseCandidateList {
            metadata: ListMeta {
                continue_token: "next".to_string(),
                ..Default::default()
            },
            items: Vec::new(),
            ..Default::default()
        };

        let v1beta1_list: k8s_api::coordination::v1beta1::LeaseCandidateList =
            list.convert_to().unwrap();
        assert_eq!(v1beta1_list.items.len(), 0);
        assert_eq!(v1beta1_list.metadata.continue_token, "next");
    }
}
