//! Policy API conversions
//!
//! This module provides conversions between policy API versions.

use crate::{ConversionError, Convertible};
use k8s_api_core::IntOrString;

// =============================================================================
// PodDisruptionBudget: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::policy::v1::PodDisruptionBudget>
    for k8s_api::policy::v1beta1::PodDisruptionBudget
{
    fn convert_to(&self) -> Result<k8s_api::policy::v1::PodDisruptionBudget, ConversionError> {
        Ok(k8s_api::policy::v1::PodDisruptionBudget {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "policy/v1",
                "PodDisruptionBudget",
            ),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_pdb_spec_to_v1(s)),
            status: self.status.as_ref().map(|s| convert_pdb_status_to_v1(s)),
        })
    }

    fn convert_from(
        other: &k8s_api::policy::v1::PodDisruptionBudget,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "policy/v1beta1",
                "PodDisruptionBudget",
            ),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_pdb_spec_from_v1(s)),
            status: other.status.as_ref().map(|s| convert_pdb_status_from_v1(s)),
        })
    }
}

// =============================================================================
// PodDisruptionBudgetList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::policy::v1::PodDisruptionBudgetList>
    for k8s_api::policy::v1beta1::PodDisruptionBudgetList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::policy::v1::PodDisruptionBudgetList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::policy::v1::PodDisruptionBudgetList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "policy/v1",
                "PodDisruptionBudgetList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::policy::v1::PodDisruptionBudgetList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::policy::v1beta1::PodDisruptionBudget::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "policy/v1beta1",
                "PodDisruptionBudgetList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_pdb_spec_to_v1(
    spec: &k8s_api::policy::v1beta1::PodDisruptionBudgetSpec,
) -> k8s_api::policy::v1::PodDisruptionBudgetSpec {
    k8s_api::policy::v1::PodDisruptionBudgetSpec {
        min_available: spec.min_available.as_ref().map(|v| int_or_string_to_json(v)),
        max_unavailable: spec.max_unavailable.as_ref().map(|v| int_or_string_to_json(v)),
        selector: spec.selector.clone(),
        unhealthy_pod_eviction_policy: spec.unhealthy_pod_eviction_policy.clone(),
    }
}

fn convert_pdb_spec_from_v1(
    spec: &k8s_api::policy::v1::PodDisruptionBudgetSpec,
) -> k8s_api::policy::v1beta1::PodDisruptionBudgetSpec {
    k8s_api::policy::v1beta1::PodDisruptionBudgetSpec {
        min_available: spec.min_available.as_ref().and_then(|v| json_to_int_or_string(v)),
        max_unavailable: spec.max_unavailable.as_ref().and_then(|v| json_to_int_or_string(v)),
        selector: spec.selector.clone(),
        unhealthy_pod_eviction_policy: spec.unhealthy_pod_eviction_policy.clone(),
    }
}

fn convert_pdb_status_to_v1(
    status: &k8s_api::policy::v1beta1::PodDisruptionBudgetStatus,
) -> k8s_api::policy::v1::PodDisruptionBudgetStatus {
    k8s_api::policy::v1::PodDisruptionBudgetStatus {
        observed_generation: status.observed_generation,
        // v1beta1 uses Time as value, v1 uses String - convert Time to RFC3339 string
        disrupted_pods: status
            .disrupted_pods
            .iter()
            .map(|(k, v)| {
                let time_str = v.0
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default();
                (k.clone(), time_str)
            })
            .collect(),
        disruptions_allowed: status.disruptions_allowed,
        current_healthy: status.current_healthy,
        desired_healthy: status.desired_healthy,
        expected_pods: status.expected_pods,
        conditions: status.conditions.clone(),
    }
}

fn convert_pdb_status_from_v1(
    status: &k8s_api::policy::v1::PodDisruptionBudgetStatus,
) -> k8s_api::policy::v1beta1::PodDisruptionBudgetStatus {
    use chrono::DateTime;

    k8s_api::policy::v1beta1::PodDisruptionBudgetStatus {
        observed_generation: status.observed_generation,
        // v1 uses String as value, v1beta1 uses Time - parse string to DateTime
        disrupted_pods: status
            .disrupted_pods
            .iter()
            .map(|(k, v)| {
                let time = if v.is_empty() {
                    k8s_apimachinery::apis::meta::v1::Time(None)
                } else {
                    let dt = DateTime::parse_from_rfc3339(v)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc));
                    k8s_apimachinery::apis::meta::v1::Time(dt)
                };
                (k.clone(), time)
            })
            .collect(),
        disruptions_allowed: status.disruptions_allowed,
        current_healthy: status.current_healthy,
        desired_healthy: status.desired_healthy,
        expected_pods: status.expected_pods,
        conditions: status.conditions.clone(),
    }
}

// Helper functions for IntOrString <-> serde_json::Value conversion

fn int_or_string_to_json(value: &IntOrString) -> serde_json::Value {
    match value {
        IntOrString::Int(i) => serde_json::Value::Number((*i).into()),
        IntOrString::String(s) => serde_json::Value::String(s.clone()),
    }
}

fn json_to_int_or_string(value: &serde_json::Value) -> Option<IntOrString> {
    match value {
        serde_json::Value::Number(n) => n.as_i64().map(|i| IntOrString::Int(i as i32)),
        serde_json::Value::String(s) => Some(IntOrString::String(s.clone())),
        _ => None,
    }
}

// =============================================================================
// Eviction: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::policy::v1::Eviction> for k8s_api::policy::v1beta1::Eviction {
    fn convert_to(&self) -> Result<k8s_api::policy::v1::Eviction, ConversionError> {
        Ok(k8s_api::policy::v1::Eviction {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new("policy/v1", "Eviction"),
            metadata: self.metadata.clone(),
            delete_options: self.delete_options.clone(),
        })
    }

    fn convert_from(other: &k8s_api::policy::v1::Eviction) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "policy/v1beta1",
                "Eviction",
            ),
            metadata: other.metadata.clone(),
            delete_options: other.delete_options.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_pdb_conversion_roundtrip() {
        let mut match_labels = BTreeMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());

        let v1beta1_pdb = k8s_api::policy::v1beta1::PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(k8s_api::policy::v1beta1::PodDisruptionBudgetSpec {
                min_available: Some(IntOrString::Int(2)),
                selector: Some(k8s_apimachinery::apis::meta::v1::LabelSelector {
                    match_labels,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_pdb: k8s_api::policy::v1::PodDisruptionBudget = v1beta1_pdb.convert_to().unwrap();
        assert_eq!(v1_pdb.metadata.name, "test-pdb");
        assert_eq!(
            v1_pdb.spec.as_ref().unwrap().min_available,
            Some(serde_json::Value::Number(2.into()))
        );

        // Convert back to v1beta1
        let converted_back: k8s_api::policy::v1beta1::PodDisruptionBudget =
            k8s_api::policy::v1beta1::PodDisruptionBudget::convert_from(&v1_pdb).unwrap();
        assert_eq!(converted_back.metadata.name, "test-pdb");
        assert_eq!(
            converted_back.spec.as_ref().unwrap().min_available,
            Some(IntOrString::Int(2))
        );
    }

    #[test]
    fn test_pdb_with_percentage() {
        let v1beta1_pdb = k8s_api::policy::v1beta1::PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(k8s_api::policy::v1beta1::PodDisruptionBudgetSpec {
                max_unavailable: Some(IntOrString::String("50%".to_string())),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_pdb: k8s_api::policy::v1::PodDisruptionBudget = v1beta1_pdb.convert_to().unwrap();
        assert_eq!(
            v1_pdb.spec.as_ref().unwrap().max_unavailable,
            Some(serde_json::Value::String("50%".to_string()))
        );

        // Convert back
        let converted_back: k8s_api::policy::v1beta1::PodDisruptionBudget =
            k8s_api::policy::v1beta1::PodDisruptionBudget::convert_from(&v1_pdb).unwrap();
        assert_eq!(
            converted_back.spec.as_ref().unwrap().max_unavailable,
            Some(IntOrString::String("50%".to_string()))
        );
    }

    #[test]
    fn test_eviction_conversion_roundtrip() {
        let v1beta1_eviction = k8s_api::policy::v1beta1::Eviction {
            metadata: ObjectMeta::named("test-pod"),
            delete_options: Some(serde_json::json!({ "gracePeriodSeconds": 30 })),
            ..Default::default()
        };

        // Convert to v1
        let v1_eviction: k8s_api::policy::v1::Eviction = v1beta1_eviction.convert_to().unwrap();
        assert_eq!(v1_eviction.metadata.name, "test-pod");

        // Convert back
        let converted_back: k8s_api::policy::v1beta1::Eviction =
            k8s_api::policy::v1beta1::Eviction::convert_from(&v1_eviction).unwrap();
        assert_eq!(converted_back.metadata.name, "test-pod");
    }

    #[test]
    fn test_pdb_list_roundtrip() {
        let list = k8s_api::policy::v1beta1::PodDisruptionBudgetList {
            metadata: ListMeta {
                resource_version: "12".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::policy::v1beta1::PodDisruptionBudget {
                metadata: ObjectMeta::named("pdb-list"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::policy::v1::PodDisruptionBudgetList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "12");
        assert_eq!(v1_list.items[0].metadata.name, "pdb-list");

        let roundtrip: k8s_api::policy::v1beta1::PodDisruptionBudgetList =
            k8s_api::policy::v1beta1::PodDisruptionBudgetList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
