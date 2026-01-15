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
