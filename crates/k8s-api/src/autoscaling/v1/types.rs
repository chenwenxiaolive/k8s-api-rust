//! Autoscaling v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// HorizontalPodAutoscaler
// =============================================================================

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscaler {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<HorizontalPodAutoscalerSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HorizontalPodAutoscalerStatus>,
}

/// HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<HorizontalPodAutoscaler>,
}

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerSpec {
    /// ScaleTargetRef points to the target resource to scale.
    pub scale_target_ref: CrossVersionObjectReference,
    /// MinReplicas is the lower limit for the number of replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// MaxReplicas is the upper limit for the number of replicas.
    pub max_replicas: i32,
    /// TargetCPUUtilizationPercentage is the target average CPU utilization over all pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cpu_utilization_percentage: Option<i32>,
}

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerStatus {
    /// ObservedGeneration is the most recent generation observed by this autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// LastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<String>,
    /// CurrentReplicas is current number of replicas of pods managed by this autoscaler.
    pub current_replicas: i32,
    /// DesiredReplicas is the desired number of replicas of pods managed by this autoscaler.
    pub desired_replicas: i32,
    /// CurrentCPUUtilizationPercentage is the current average CPU utilization over all pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_cpu_utilization_percentage: Option<i32>,
}

/// CrossVersionObjectReference contains enough information to identify the referred resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossVersionObjectReference {
    /// Kind of the referent.
    pub kind: String,
    /// Name of the referent.
    pub name: String,
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
}

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}

/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    /// Desired number of instances for the scaled object.
    #[serde(default)]
    pub replicas: i32,
}

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    /// Actual number of observed instances of the scaled object.
    pub replicas: i32,
    /// Label query over pods that should match the replicas count.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub selector: String,
}
