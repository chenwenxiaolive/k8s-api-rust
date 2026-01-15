//! Scheduling API conversions
//!
//! This module provides conversions between scheduling API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// PriorityClass: v1beta1/v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::scheduling::v1::PriorityClass>
    for k8s_api::scheduling::v1beta1::PriorityClass
{
    fn convert_to(&self) -> Result<k8s_api::scheduling::v1::PriorityClass, ConversionError> {
        let mut converted: k8s_api::scheduling::v1::PriorityClass = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "scheduling.k8s.io/v1",
            "PriorityClass",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::scheduling::v1::PriorityClass,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::scheduling::v1beta1::PriorityClass = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "scheduling.k8s.io/v1beta1",
            "PriorityClass",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::scheduling::v1::PriorityClass>
    for k8s_api::scheduling::v1alpha1::PriorityClass
{
    fn convert_to(&self) -> Result<k8s_api::scheduling::v1::PriorityClass, ConversionError> {
        let mut converted: k8s_api::scheduling::v1::PriorityClass = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "scheduling.k8s.io/v1",
            "PriorityClass",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::scheduling::v1::PriorityClass,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::scheduling::v1alpha1::PriorityClass = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "scheduling.k8s.io/v1alpha1",
            "PriorityClass",
        );
        Ok(converted)
    }
}

// =============================================================================
// PriorityClassList: v1beta1/v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::scheduling::v1::PriorityClassList>
    for k8s_api::scheduling::v1beta1::PriorityClassList
{
    fn convert_to(&self) -> Result<k8s_api::scheduling::v1::PriorityClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::scheduling::v1::PriorityClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "scheduling.k8s.io/v1",
                "PriorityClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::scheduling::v1::PriorityClassList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::scheduling::v1beta1::PriorityClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "scheduling.k8s.io/v1beta1",
                "PriorityClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::scheduling::v1::PriorityClassList>
    for k8s_api::scheduling::v1alpha1::PriorityClassList
{
    fn convert_to(&self) -> Result<k8s_api::scheduling::v1::PriorityClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::scheduling::v1::PriorityClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "scheduling.k8s.io/v1",
                "PriorityClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::scheduling::v1::PriorityClassList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::scheduling::v1alpha1::PriorityClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "scheduling.k8s.io/v1alpha1",
                "PriorityClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}
