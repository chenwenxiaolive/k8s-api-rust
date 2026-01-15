//! Scheduling v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// PriorityClass
// =============================================================================

/// PriorityClass defines mapping from a priority class name to the priority integer value.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Value represents the integer value of this priority class.
    pub value: i32,
    /// GlobalDefault specifies whether this PriorityClass should be considered as the default priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_default: Option<bool>,
    /// Description is an arbitrary string that usually provides guidelines on when this priority class should be used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<String>,
}

/// PriorityClassList is a collection of priority classes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<PriorityClass>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_class_serialization_roundtrip() {
        let class = PriorityClass {
            type_meta: TypeMeta::new("scheduling.k8s.io/v1", "PriorityClass"),
            metadata: ObjectMeta::named("high-priority"),
            value: 1000,
            global_default: Some(false),
            description: "critical workloads".to_string(),
            preemption_policy: Some("PreemptLowerPriority".to_string()),
        };

        let json = serde_json::to_string(&class).unwrap();
        let parsed: PriorityClass = serde_json::from_str(&json).unwrap();

        assert_eq!(class.metadata.name, parsed.metadata.name);
        assert_eq!(class.value, parsed.value);
        assert_eq!(class.preemption_policy, parsed.preemption_policy);
    }
}
