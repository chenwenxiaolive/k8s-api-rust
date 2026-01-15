//! Autoscaling v2beta1 API type definitions (deprecated)

use k8s_api_core::resource::Quantity;
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
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
    pub metadata: ListMeta,
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
    /// Metrics contains the specifications for which to use to calculate the desired replica count.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSpec>,
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
    #[serde(default)]
    pub current_replicas: i32,
    /// DesiredReplicas is the desired number of replicas of pods managed by this autoscaler.
    pub desired_replicas: i32,
    /// CurrentMetrics is the last read state of the metrics used by this autoscaler.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_metrics: Vec<MetricStatus>,
    /// Conditions is the set of conditions required for this autoscaler to scale its target.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<HorizontalPodAutoscalerCondition>,
}

/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerCondition {
    /// Type describes the current condition.
    #[serde(rename = "type")]
    pub condition_type: HorizontalPodAutoscalerConditionType,
    /// Status is the status of the condition (True, False, Unknown).
    pub status: crate::core::v1::ConditionStatus,
    /// LastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    /// Reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Message is a human-readable explanation containing details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
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

// =============================================================================
// Metrics
// =============================================================================

/// MetricSpec specifies how to scale based on a single metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricSpec {
    /// Type is the type of metric source.
    #[serde(rename = "type")]
    pub type_: MetricSourceType,
    /// Object refers to a metric describing a single kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricSource>,
    /// Pods refers to a metric describing each pod in the current scale target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricSource>,
    /// Resource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricSource>,
    /// ContainerResource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricSource>,
    /// External refers to a global metric that is not associated with any Kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricSource>,
}

/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricStatus {
    /// Type is the type of metric source.
    #[serde(rename = "type")]
    pub type_: MetricSourceType,
    /// Object refers to a metric describing a single kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricStatus>,
    /// Pods refers to a metric describing each pod in the current scale target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricStatus>,
    /// Resource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricStatus>,
    /// ContainerResource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricStatus>,
    /// External refers to a global metric that is not associated with any Kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricStatus>,
}

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object.
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

/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object.
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

/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSource {
    pub metric_name: String,
    pub target_average_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricStatus {
    pub metric_name: String,
    pub current_average_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricSource {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricStatus {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
}

/// ContainerResourceMetricSource indicates how to scale on a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSource {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    /// Container is the name of the container in the pods of the scaling target.
    pub container: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricStatus {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    /// Container is the name of the container in the pods of the scaling target.
    pub container: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
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

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricStatus {
    pub metric_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<LabelSelector>,
    pub current_value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
}
