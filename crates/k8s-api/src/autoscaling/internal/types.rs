//! Internal type definitions for autoscaling.

use k8s_api_core::resource::Quantity;
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};

pub type HPAScalingPolicyType = String;
pub type HorizontalPodAutoscalerConditionType = String;
pub type MetricSourceType = String;
pub type MetricTargetType = String;
pub type ScalingPolicySelect = String;

pub const HPA_CONDITION_ABLE_TO_SCALE: &str = "AbleToScale";
pub const HPA_CONDITION_SCALING_ACTIVE: &str = "ScalingActive";
pub const HPA_CONDITION_SCALING_LIMITED: &str = "ScalingLimited";
pub const HPA_SCALING_POLICY_PERCENT: &str = "Percent";
pub const HPA_SCALING_POLICY_PODS: &str = "Pods";
pub const METRIC_SOURCE_TYPE_CONTAINER_RESOURCE: &str = "ContainerResource";
pub const METRIC_SOURCE_TYPE_EXTERNAL: &str = "External";
pub const METRIC_SOURCE_TYPE_OBJECT: &str = "Object";
pub const METRIC_SOURCE_TYPE_PODS: &str = "Pods";
pub const METRIC_SOURCE_TYPE_RESOURCE: &str = "Resource";
pub const METRIC_TARGET_TYPE_AVERAGE_VALUE: &str = "AverageValue";
pub const METRIC_TARGET_TYPE_UTILIZATION: &str = "Utilization";
pub const METRIC_TARGET_TYPE_VALUE: &str = "Value";
pub const SCALING_POLICY_SELECT_DISABLED: &str = "Disabled";
pub const SCALING_POLICY_SELECT_MAX: &str = "Max";
pub const SCALING_POLICY_SELECT_MIN: &str = "Min";


/// ContainerResourceMetricSource indicates how to scale on a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSource {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    /// Container is the name of the container in the pods of the scaling target.
    pub container: String,
    pub target: MetricTarget,
}


/// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricStatus {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    /// Container is the name of the container in the pods of the scaling target.
    pub container: String,
    pub current: MetricValueStatus,
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


/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricSource {
    pub metric: MetricIdentifier,
    pub target: MetricTarget,
}


/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricStatus {
    pub metric: MetricIdentifier,
    pub current: MetricValueStatus,
}


/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HPAScalingPolicy {
    /// Type is used to specify the scaling policy.
    #[serde(rename = "type")]
    pub type_: HPAScalingPolicyType,
    /// Value contains the amount of change which is permitted by the policy.
    pub value: i32,
    /// PeriodSeconds specifies the window of time for which the policy should hold true.
    pub period_seconds: i32,
}


/// HPAScalingRules configures the scaling behavior for one direction.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HPAScalingRules {
    /// StabilizationWindowSeconds is the number of seconds for which past recommendations should be considered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: Option<i32>,
    /// SelectPolicy is used to specify which policy should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select_policy: Option<ScalingPolicySelect>,
    /// Policies is a list of potential scaling polices which can be used during scaling.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<HPAScalingPolicy>,
    /// Tolerance is the tolerance on the ratio between the current and desired metric value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<Quantity>,
}


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


/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerBehavior {
    /// ScaleUp is scaling policy for scaling Up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_up: Option<HPAScalingRules>,
    /// ScaleDown is scaling policy for scaling Down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_down: Option<HPAScalingRules>,
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
    /// Behavior configures the scaling behavior of the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<HorizontalPodAutoscalerBehavior>,
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


/// MetricIdentifier defines the name and optionally selector for a metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricIdentifier {
    /// Name is the name of the given metric.
    pub name: String,
    /// Selector is the string-encoded form of a standard kubernetes label selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}


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


/// MetricTarget defines the target value, average value, or average utilization of a specific metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricTarget {
    /// Type represents whether the metric type is Utilization, Value, or AverageValue.
    #[serde(rename = "type")]
    pub type_: MetricTargetType,
    /// Value is the target value of the metric (as a quantity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    /// AverageValue is the target value of the average of the metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    /// AverageUtilization is the target value of the average of the resource metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}


/// MetricValueStatus holds the current value for a metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricValueStatus {
    /// Value is the current value of the metric (as a quantity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    /// AverageValue is the current value of the average of the metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    /// AverageUtilization is the current value of the average of the resource metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}


/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricSource {
    pub described_object: CrossVersionObjectReference,
    pub target: MetricTarget,
    pub metric: MetricIdentifier,
}


/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricStatus {
    pub metric: MetricIdentifier,
    pub current: MetricValueStatus,
    pub described_object: CrossVersionObjectReference,
}


/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSource {
    pub metric: MetricIdentifier,
    pub target: MetricTarget,
}


/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricStatus {
    pub metric: MetricIdentifier,
    pub current: MetricValueStatus,
}


/// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricSource {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    pub target: MetricTarget,
}


/// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricStatus {
    /// Name is the name of the resource in question.
    pub name: crate::core::v1::ResourceName,
    pub current: MetricValueStatus,
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
