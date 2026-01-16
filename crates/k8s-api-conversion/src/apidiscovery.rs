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

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    #[test]
    fn test_api_group_discovery_list_roundtrip() {
        let list = k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList {
            metadata: ListMeta {
                resource_version: "7".to_string(),
                continue_token: "next".to_string(),
                remaining_item_count: Some(1),
                ..Default::default()
            },
            items: vec![k8s_api::apidiscovery::v2beta1::APIGroupDiscovery {
                metadata: ObjectMeta::named("apps"),
                versions: vec![k8s_api::apidiscovery::v2beta1::APIVersionDiscovery {
                    version: "v1".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };

        let v2_list: k8s_api::apidiscovery::v2::APIGroupDiscoveryList =
            list.convert_to().unwrap();
        assert_eq!(v2_list.metadata.resource_version, "7");
        assert_eq!(v2_list.metadata.continue_token, "next");
        assert_eq!(v2_list.items.len(), 1);
        assert_eq!(v2_list.items[0].metadata.name, "apps");
        assert_eq!(v2_list.type_meta.api_version, "apidiscovery.k8s.io/v2");

        let roundtrip: k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList =
            k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList::convert_from(&v2_list)
                .unwrap();
        assert_eq!(roundtrip.items[0].metadata.name, "apps");
        assert_eq!(roundtrip.metadata.remaining_item_count, Some(1));
    }
}
