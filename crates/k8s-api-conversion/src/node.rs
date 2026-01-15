//! Node API conversions
//!
//! This module provides conversions between node API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// RuntimeClass: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::node::v1::RuntimeClass> for k8s_api::node::v1beta1::RuntimeClass {
    fn convert_to(&self) -> Result<k8s_api::node::v1::RuntimeClass, ConversionError> {
        let mut converted: k8s_api::node::v1::RuntimeClass = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "node.k8s.io/v1",
            "RuntimeClass",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::node::v1::RuntimeClass) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::node::v1beta1::RuntimeClass = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "node.k8s.io/v1beta1",
            "RuntimeClass",
        );
        Ok(converted)
    }
}

// =============================================================================
// RuntimeClass: v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::node::v1::RuntimeClass> for k8s_api::node::v1alpha1::RuntimeClass {
    fn convert_to(&self) -> Result<k8s_api::node::v1::RuntimeClass, ConversionError> {
        let mut converted: k8s_api::node::v1::RuntimeClass = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "node.k8s.io/v1",
            "RuntimeClass",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::node::v1::RuntimeClass) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::node::v1alpha1::RuntimeClass = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "node.k8s.io/v1alpha1",
            "RuntimeClass",
        );
        Ok(converted)
    }
}

// =============================================================================
// RuntimeClassList: v1beta1/v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::node::v1::RuntimeClassList> for k8s_api::node::v1beta1::RuntimeClassList {
    fn convert_to(&self) -> Result<k8s_api::node::v1::RuntimeClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::node::v1::RuntimeClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("node.k8s.io/v1", "RuntimeClassList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::node::v1::RuntimeClassList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::node::v1beta1::RuntimeClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("node.k8s.io/v1beta1", "RuntimeClassList"),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::node::v1::RuntimeClassList> for k8s_api::node::v1alpha1::RuntimeClassList {
    fn convert_to(&self) -> Result<k8s_api::node::v1::RuntimeClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::node::v1::RuntimeClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("node.k8s.io/v1", "RuntimeClassList"),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::node::v1::RuntimeClassList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::node::v1alpha1::RuntimeClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("node.k8s.io/v1alpha1", "RuntimeClassList"),
            metadata: other.metadata.clone(),
            items,
        })
    }
}
