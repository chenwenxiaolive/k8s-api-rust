use super::*;

impl InternalConversion for HorizontalPodAutoscaler {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscaler;
}

impl InternalConversion for HorizontalPodAutoscalerList {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscalerList;
}

impl InternalConversion for HorizontalPodAutoscalerSpec {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscalerSpec;
}

impl InternalConversion for HorizontalPodAutoscalerStatus {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscalerStatus;
}

impl InternalConversion for HorizontalPodAutoscalerCondition {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscalerCondition;
}

impl InternalConversion for CrossVersionObjectReference {
    type Internal = crate::autoscaling::internal::CrossVersionObjectReference;
}

impl InternalConversion for MetricSpec {
    type Internal = crate::autoscaling::internal::MetricSpec;
}

impl InternalConversion for MetricStatus {
    type Internal = crate::autoscaling::internal::MetricStatus;
}

impl InternalConversion for ObjectMetricSource {
    type Internal = crate::autoscaling::internal::ObjectMetricSource;
}

impl InternalConversion for ObjectMetricStatus {
    type Internal = crate::autoscaling::internal::ObjectMetricStatus;
}

impl InternalConversion for PodsMetricSource {
    type Internal = crate::autoscaling::internal::PodsMetricSource;
}

impl InternalConversion for PodsMetricStatus {
    type Internal = crate::autoscaling::internal::PodsMetricStatus;
}

impl InternalConversion for ResourceMetricSource {
    type Internal = crate::autoscaling::internal::ResourceMetricSource;
}

impl InternalConversion for ResourceMetricStatus {
    type Internal = crate::autoscaling::internal::ResourceMetricStatus;
}

impl InternalConversion for ContainerResourceMetricSource {
    type Internal = crate::autoscaling::internal::ContainerResourceMetricSource;
}

impl InternalConversion for ContainerResourceMetricStatus {
    type Internal = crate::autoscaling::internal::ContainerResourceMetricStatus;
}

impl InternalConversion for ExternalMetricSource {
    type Internal = crate::autoscaling::internal::ExternalMetricSource;
}

impl InternalConversion for ExternalMetricStatus {
    type Internal = crate::autoscaling::internal::ExternalMetricStatus;
}

impl InternalConversion for MetricIdentifier {
    type Internal = crate::autoscaling::internal::MetricIdentifier;
}

impl InternalConversion for MetricTarget {
    type Internal = crate::autoscaling::internal::MetricTarget;
}

impl InternalConversion for MetricValueStatus {
    type Internal = crate::autoscaling::internal::MetricValueStatus;
}

impl InternalConversion for HorizontalPodAutoscalerBehavior {
    type Internal = crate::autoscaling::internal::HorizontalPodAutoscalerBehavior;
}

impl InternalConversion for HPAScalingRules {
    type Internal = crate::autoscaling::internal::HPAScalingRules;
}

impl InternalConversion for HPAScalingPolicy {
    type Internal = crate::autoscaling::internal::HPAScalingPolicy;
}
