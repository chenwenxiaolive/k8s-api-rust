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

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    #[test]
    fn test_flow_schema_list_roundtrip() {
        let list = k8s_api::flowcontrol::v1beta3::FlowSchemaList {
            metadata: ListMeta {
                resource_version: "1".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::flowcontrol::v1beta3::FlowSchema {
                metadata: ObjectMeta::named("flow"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::flowcontrol::v1::FlowSchemaList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "1");
        assert_eq!(v1_list.items[0].metadata.name, "flow");

        let roundtrip: k8s_api::flowcontrol::v1beta3::FlowSchemaList =
            k8s_api::flowcontrol::v1beta3::FlowSchemaList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }

    #[test]
    fn test_priority_level_list_empty() {
        let list = k8s_api::flowcontrol::v1beta3::PriorityLevelConfigurationList {
            metadata: ListMeta {
                continue_token: "next".to_string(),
                ..Default::default()
            },
            items: Vec::new(),
            ..Default::default()
        };

        let v1_list: k8s_api::flowcontrol::v1::PriorityLevelConfigurationList =
            list.convert_to().unwrap();
        assert_eq!(v1_list.items.len(), 0);
        assert_eq!(v1_list.metadata.continue_token, "next");
    }
}
