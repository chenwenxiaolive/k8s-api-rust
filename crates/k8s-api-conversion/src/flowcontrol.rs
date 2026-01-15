//! Flowcontrol API conversions
//!
//! This module provides conversions between flowcontrol API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// FlowSchema: v1beta1/v1beta2/v1beta3 <-> v1
// =============================================================================

impl Convertible<k8s_api::flowcontrol::v1::FlowSchema>
    for k8s_api::flowcontrol::v1beta1::FlowSchema
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchema, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::FlowSchema = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "FlowSchema",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchema) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta1::FlowSchema = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta1",
            "FlowSchema",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::flowcontrol::v1::FlowSchema>
    for k8s_api::flowcontrol::v1beta2::FlowSchema
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchema, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::FlowSchema = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "FlowSchema",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchema) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta2::FlowSchema = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta2",
            "FlowSchema",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::flowcontrol::v1::FlowSchema>
    for k8s_api::flowcontrol::v1beta3::FlowSchema
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchema, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::FlowSchema = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "FlowSchema",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchema) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta3::FlowSchema = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta3",
            "FlowSchema",
        );
        Ok(converted)
    }
}

// =============================================================================
// FlowSchemaList: v1beta1/v1beta2/v1beta3 <-> v1
// =============================================================================

impl Convertible<k8s_api::flowcontrol::v1::FlowSchemaList>
    for k8s_api::flowcontrol::v1beta1::FlowSchemaList
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchemaList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::FlowSchemaList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "FlowSchemaList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchemaList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta1::FlowSchema::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta1",
                "FlowSchemaList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::flowcontrol::v1::FlowSchemaList>
    for k8s_api::flowcontrol::v1beta2::FlowSchemaList
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchemaList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::FlowSchemaList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "FlowSchemaList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchemaList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta2::FlowSchema::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta2",
                "FlowSchemaList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::flowcontrol::v1::FlowSchemaList>
    for k8s_api::flowcontrol::v1beta3::FlowSchemaList
{
    fn convert_to(&self) -> Result<k8s_api::flowcontrol::v1::FlowSchemaList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::FlowSchemaList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "FlowSchemaList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::flowcontrol::v1::FlowSchemaList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta3::FlowSchema::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta3",
                "FlowSchemaList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// PriorityLevelConfiguration: v1beta1/v1beta2/v1beta3 <-> v1
// =============================================================================

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfiguration>
    for k8s_api::flowcontrol::v1beta1::PriorityLevelConfiguration
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfiguration, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::PriorityLevelConfiguration =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfiguration,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta1::PriorityLevelConfiguration =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta1",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfiguration>
    for k8s_api::flowcontrol::v1beta2::PriorityLevelConfiguration
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfiguration, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::PriorityLevelConfiguration =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfiguration,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta2::PriorityLevelConfiguration =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta2",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfiguration>
    for k8s_api::flowcontrol::v1beta3::PriorityLevelConfiguration
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfiguration, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1::PriorityLevelConfiguration =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfiguration,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::flowcontrol::v1beta3::PriorityLevelConfiguration =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "flowcontrol.apiserver.k8s.io/v1beta3",
            "PriorityLevelConfiguration",
        );
        Ok(converted)
    }
}

// =============================================================================
// PriorityLevelConfigurationList: v1beta1/v1beta2/v1beta3 <-> v1
// =============================================================================

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList>
    for k8s_api::flowcontrol::v1beta1::PriorityLevelConfigurationList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::PriorityLevelConfigurationList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "PriorityLevelConfigurationList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta1::PriorityLevelConfiguration::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta1",
                "PriorityLevelConfigurationList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList>
    for k8s_api::flowcontrol::v1beta2::PriorityLevelConfigurationList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::PriorityLevelConfigurationList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "PriorityLevelConfigurationList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta2::PriorityLevelConfiguration::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta2",
                "PriorityLevelConfigurationList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList>
    for k8s_api::flowcontrol::v1beta3::PriorityLevelConfigurationList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::flowcontrol::v1::PriorityLevelConfigurationList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::flowcontrol::v1::PriorityLevelConfigurationList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1",
                "PriorityLevelConfigurationList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::flowcontrol::v1beta3::PriorityLevelConfiguration::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "flowcontrol.apiserver.k8s.io/v1beta3",
                "PriorityLevelConfigurationList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}
