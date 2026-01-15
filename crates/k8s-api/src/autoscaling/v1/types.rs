//! Autoscaling v1 API type definitions

use k8s_api_core::resource::Quantity;
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

pub type MetricSourceType = String;
pub type HorizontalPodAutoscalerConditionType = String;

// MetricSourceType constants
pub const METRIC_SOURCE_TYPE_OBJECT: &str = "Object";
pub const METRIC_SOURCE_TYPE_PODS: &str = "Pods";
pub const METRIC_SOURCE_TYPE_RESOURCE: &str = "Resource";
pub const METRIC_SOURCE_TYPE_CONTAINER_RESOURCE: &str = "ContainerResource";
pub const METRIC_SOURCE_TYPE_EXTERNAL: &str = "External";

// HorizontalPodAutoscalerConditionType constants
pub const HPA_CONDITION_SCALING_ACTIVE: &str = "ScalingActive";
pub const HPA_CONDITION_ABLE_TO_SCALE: &str = "AbleToScale";
pub const HPA_CONDITION_SCALING_LIMITED: &str = "ScalingLimited";

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
    pub last_scale_time: Option<Time>,
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

// =============================================================================
// Metric types (alpha metrics annotation)
// =============================================================================

/// MetricSpec specifies how to scale based on a single metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricSpec {
    #[serde(rename = "type")]
    pub type_: MetricSourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricSource>,
}

/// ObjectMetricSource indicates how to scale on a metric describing a Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricSource {
    pub target: CrossVersionObjectReference,
    pub metric_name: String,
    pub target_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
}

/// PodsMetricSource indicates how to scale on a metric describing each pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSource {
    pub metric_name: String,
    pub target_average_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// ResourceMetricSource indicates how to scale on a resource metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricSource {
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// ContainerResourceMetricSource indicates how to scale on a per-container resource metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSource {
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
    pub container: String,
}

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricSource {
    pub metric_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricStatus {
    #[serde(rename = "type")]
    pub type_: MetricSourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricStatus>,
}

/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerCondition {
    #[serde(rename = "type")]
    pub condition_type: HorizontalPodAutoscalerConditionType,
    pub status: crate::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// ObjectMetricStatus indicates the current value of a metric describing a Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricStatus {
    pub target: CrossVersionObjectReference,
    pub metric_name: String,
    pub current_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
}

/// PodsMetricStatus indicates the current value of a metric describing each pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricStatus {
    pub metric_name: String,
    pub current_average_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// ResourceMetricStatus indicates the current value of a resource metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricStatus {
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
}

/// ContainerResourceMetricStatus indicates the current value of a container resource metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricStatus {
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
    pub container: String,
}

/// ExternalMetricStatus indicates the current value of a global metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricStatus {
    pub metric_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
}
