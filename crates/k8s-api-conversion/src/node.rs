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
        let overhead = convert_via_json(&self.spec.overhead)?;
        let scheduling = convert_via_json(&self.spec.scheduling)?;

        Ok(k8s_api::node::v1::RuntimeClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "node.k8s.io/v1",
                "RuntimeClass",
            ),
            metadata: self.metadata.clone(),
            handler: self.spec.runtime_handler.clone(),
            overhead,
            scheduling,
        })
    }

    fn convert_from(other: &k8s_api::node::v1::RuntimeClass) -> Result<Self, ConversionError> {
        let overhead = convert_via_json(&other.overhead)?;
        let scheduling = convert_via_json(&other.scheduling)?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "node.k8s.io/v1alpha1",
                "RuntimeClass",
            ),
            metadata: other.metadata.clone(),
            spec: k8s_api::node::v1alpha1::RuntimeClassSpec {
                runtime_handler: other.handler.clone(),
                overhead,
                scheduling,
            },
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    #[test]
    fn test_runtime_class_list_roundtrip() {
        let list = k8s_api::node::v1beta1::RuntimeClassList {
            metadata: ListMeta {
                resource_version: "6".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::node::v1beta1::RuntimeClass {
                metadata: ObjectMeta::named("runc"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::node::v1::RuntimeClassList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "6");
        assert_eq!(v1_list.items[0].metadata.name, "runc");

        let roundtrip: k8s_api::node::v1beta1::RuntimeClassList =
            k8s_api::node::v1beta1::RuntimeClassList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
