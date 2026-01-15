//! API discovery conversions
//!
//! This module provides conversions between apidiscovery API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// APIGroupDiscoveryList: v2beta1 <-> v2
// =============================================================================

impl Convertible<k8s_api::apidiscovery::v2::APIGroupDiscoveryList>
    for k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::apidiscovery::v2::APIGroupDiscoveryList, ConversionError> {
        let mut converted: k8s_api::apidiscovery::v2::APIGroupDiscoveryList =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "apidiscovery.k8s.io/v2",
            "APIGroupDiscoveryList",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::apidiscovery::v2::APIGroupDiscoveryList,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "apidiscovery.k8s.io/v2beta1",
            "APIGroupDiscoveryList",
        );
        Ok(converted)
    }
}

// =============================================================================
// APIGroupDiscovery: v2beta1 <-> v2
// =============================================================================

impl Convertible<k8s_api::apidiscovery::v2::APIGroupDiscovery>
    for k8s_api::apidiscovery::v2beta1::APIGroupDiscovery
{
    fn convert_to(&self) -> Result<k8s_api::apidiscovery::v2::APIGroupDiscovery, ConversionError> {
        let mut converted: k8s_api::apidiscovery::v2::APIGroupDiscovery =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "apidiscovery.k8s.io/v2",
            "APIGroupDiscovery",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::apidiscovery::v2::APIGroupDiscovery,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::apidiscovery::v2beta1::APIGroupDiscovery =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "apidiscovery.k8s.io/v2beta1",
            "APIGroupDiscovery",
        );
        Ok(converted)
    }
}
