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
