//! Resource v1alpha3 API type definitions for Dynamic Resource Allocation (DRA)

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

pub type DeviceTaintEffect = String;

/// DeviceSelector must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelector {
    /// CEL contains a CEL expression for selecting a device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cel: Option<CELDeviceSelector>,
}

/// CELDeviceSelector contains a CEL expression for selecting a device.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CELDeviceSelector {
    /// Expression is a CEL expression which evaluates a single device.
    pub expression: String,
}

/// CELSelectorExpressionMaxCost specifies the cost limit for a single CEL selector evaluation.
pub const CEL_SELECTOR_EXPRESSION_MAX_COST: i64 = 1_000_000;

/// CELSelectorExpressionMaxLength is the maximum length of a CEL selector expression string.
pub const CEL_SELECTOR_EXPRESSION_MAX_LENGTH: i64 = 10 * 1024;

/// The device this taint is attached to has the "effect" on claims and pods using the claim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaint {
    /// The taint key to be applied to a device.
    pub key: String,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// The effect of the taint on claims that do not tolerate the taint.
    pub effect: DeviceTaintEffect,
    /// TimeAdded represents the time at which the taint was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<Time>,
}

// DeviceTaintEffect constants
pub const DEVICE_TAINT_EFFECT_NO_SCHEDULE: &str = "NoSchedule";
pub const DEVICE_TAINT_EFFECT_NO_EXECUTE: &str = "NoExecute";

/// DeviceTaintRule adds one taint to all devices which match the selector.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaintRule {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: DeviceTaintRuleSpec,
}

/// DeviceTaintRuleSpec specifies the selector and one taint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaintRuleSpec {
    /// DeviceSelector defines which device(s) the taint is applied to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_selector: Option<DeviceTaintSelector>,
    /// Taint is the taint that gets applied to matching devices.
    pub taint: DeviceTaint,
}

/// DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaintSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
}

/// DeviceTaintRuleList is a collection of DeviceTaintRules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaintRuleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<DeviceTaintRule>,
}
