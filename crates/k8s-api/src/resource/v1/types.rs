//! Resource v1 API type definitions for Dynamic Resource Allocation (DRA)
//!
//! This module provides types for dynamic resource allocation (K8s 1.34+).

// Reserved constants for built-in controllers.
pub const FINALIZER: &str = "resource.kubernetes.io/delete-protection";
pub const EXTENDED_RESOURCE_CLAIM_ANNOTATION: &str =
    "resource.kubernetes.io/extended-resource-claim";
pub const RESOURCE_DEVICE_CLASS_PREFIX: &str = "deviceclass.resource.kubernetes.io/";

pub const RESOURCE_SLICE_SELECTOR_NODE_NAME: &str = "spec.nodeName";
pub const RESOURCE_SLICE_SELECTOR_DRIVER: &str = "spec.driver";

use k8s_apimachinery::apis::meta::v1::{Condition, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// =============================================================================
// ResourceClaim
// =============================================================================

/// ResourceClaim describes a request for access to resources in the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: ResourceClaimSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceClaimStatus>,
}

/// ResourceClaimList is a collection of claims.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ResourceClaim>,
}

/// ResourceClaimSpec defines what is being requested and how to configure it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpec {
    /// Devices defines how to request devices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<DeviceClaim>,
}

/// ResourceClaimStatus tracks whether the resource has been allocated and what the result was.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatus {
    /// Allocation is set once the claim has been allocated successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<AllocationResult>,
    /// ReservedFor indicates which entities are currently allowed to use the claim.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_for: Vec<ResourceClaimConsumerReference>,
    /// Devices contains the status of each device allocated for this claim.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<AllocatedDeviceStatus>,
}

/// DeviceClaim defines how to request devices with a ResourceClaim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaim {
    /// Requests represent individual requests for distinct devices.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<DeviceRequest>,
    /// Constraints must all be satisfied by the set of devices that get allocated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<DeviceConstraint>,
    /// Config contains configuration parameters for each request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceClaimConfiguration>,
}

/// DeviceRequest is a request for devices required for a claim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    /// Name can be used to reference this request in a pod.spec.containers[].resources.claims entry.
    pub name: String,
    /// DeviceClassName references a specific DeviceClass.
    pub device_class_name: String,
    /// Selectors define criteria which must be satisfied by a specific device.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// AllocationMode and its related fields define how devices are allocated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub allocation_mode: String,
    /// Count is used only when the count allocation mode is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// AdminAccess indicates that this is a claim for administrative access to the device(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    /// FirstAvailable is used with the ExactCount allocation mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_available: Option<Vec<DeviceSubRequest>>,
}

/// DeviceSubRequest is a sub-request in FirstAvailable allocation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSubRequest {
    /// Name is the name of the sub-request.
    pub name: String,
    /// DeviceClassName references a specific DeviceClass.
    pub device_class_name: String,
    /// Selectors define criteria which must be satisfied by a specific device.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// AllocationMode and its related fields define how devices are allocated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub allocation_mode: String,
    /// Count is used only when the count allocation mode is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

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

/// DeviceConstraint must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConstraint {
    /// Requests is a list of the one or more requests in this claim which must co-satisfy this constraint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    /// MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_attribute: Option<String>,
}

/// DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaimConfiguration {
    /// Requests lists the names of requests where the configuration applies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    /// Opaque provides driver-specific configuration parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

/// OpaqueDeviceConfiguration contains configuration parameters for a driver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpaqueDeviceConfiguration {
    /// Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.
    pub driver: String,
    /// Parameters can contain arbitrary data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// AllocationResult contains attributes of an allocated resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResult {
    /// Devices is the result of allocating devices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<DeviceAllocationResult>,
    /// NodeSelector defines where the allocated resources are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<serde_json::Value>,
}

/// DeviceAllocationResult is the result of allocating devices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationResult {
    /// Results lists all allocated devices.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<DeviceRequestAllocationResult>,
    /// Config contains configuration parameters for the allocated devices.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceAllocationConfiguration>,
}

/// DeviceRequestAllocationResult contains the allocation result for one request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestAllocationResult {
    /// Request is the name of the request in the claim which caused this device to be allocated.
    pub request: String,
    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked.
    pub driver: String,
    /// Pool is the name of the DRA driver pool.
    pub pool: String,
    /// Device is the name of the allocated device.
    pub device: String,
    /// AdminAccess indicates that this device was allocated for administrative access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
}

/// DeviceAllocationConfiguration gets embedded in an AllocationResult.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationConfiguration {
    /// Source records where the configuration originates from.
    pub source: String,
    /// Requests lists the names of requests where the configuration applies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    /// Opaque provides driver-specific configuration parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

/// ResourceClaimConsumerReference contains enough information to identify the consumer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimConsumerReference {
    /// APIGroup is the group for the resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    /// Resource is the type of resource being referenced.
    pub resource: String,
    /// Name is the name of resource being referenced.
    pub name: String,
    /// UID identifies exactly one incarnation of the resource.
    pub uid: String,
}

/// AllocatedDeviceStatus contains the status of an allocated device.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedDeviceStatus {
    /// Driver specifies the name of the DRA driver.
    pub driver: String,
    /// Pool is the name of the DRA driver pool.
    pub pool: String,
    /// Device is the name of the allocated device.
    pub device: String,
    /// Conditions contains the latest observation of the device's state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    /// Data contains arbitrary driver-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// NetworkData contains network-related information specific to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<NetworkDeviceData>,
}

/// NetworkDeviceData provides network-related details for the allocated device.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDeviceData {
    /// InterfaceName specifies the name of the network interface.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub interface_name: String,
    /// IPs lists the network addresses assigned to the device's network interface.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ips: Vec<String>,
    /// HardwareAddress represents the hardware address (e.g. MAC address).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hardware_address: String,
}

// =============================================================================
// DeviceClass
// =============================================================================

/// DeviceClass is a vendor- or admin-provided resource that contains device configuration and selectors.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: DeviceClassSpec,
}

/// DeviceClassList is a collection of classes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<DeviceClass>,
}

/// DeviceClassSpec is used in a DeviceClass to define what can be allocated and how to configure it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpec {
    /// Selectors define criteria which must be satisfied by a specific device.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// Config defines configuration parameters that apply to each device that is claimed via this class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceClassConfiguration>,
}

/// DeviceClassConfiguration is used in DeviceClass.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassConfiguration {
    /// Opaque provides driver-specific configuration parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

// =============================================================================
// ResourceClaimTemplate
// =============================================================================

/// ResourceClaimTemplate is used to produce ResourceClaim objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: ResourceClaimTemplateSpec,
}

/// ResourceClaimTemplateList is a collection of claim templates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ResourceClaimTemplate>,
}

/// ResourceClaimTemplateSpec contains the metadata and fields for a ResourceClaim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateSpec {
    /// ObjectMeta may contain labels and annotations that will be copied into the ResourceClaim.
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec for the ResourceClaim.
    pub spec: ResourceClaimSpec,
}

// =============================================================================
// ResourceSlice
// =============================================================================

/// ResourceSlice represents one or more resources in a pool of similar resources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: ResourceSliceSpec,
}

/// ResourceSliceList is a collection of ResourceSlices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ResourceSlice>,
}

/// ResourceSliceSpec contains the information published by the driver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceSpec {
    /// Driver identifies the DRA driver providing the capacity information.
    pub driver: String,
    /// Pool describes the pool that this ResourceSlice belongs to.
    pub pool: ResourcePool,
    /// NodeName identifies the node which provides the resources in this pool.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// NodeSelector defines which nodes have access to the resources in the pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<serde_json::Value>,
    /// AllNodes indicates that all nodes have access to the resources in the pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    /// Devices lists all available devices in this pool slice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<Device>,
    /// SharedCapacity defines capacities that are shared by multiple devices.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_capacity: Vec<SharedCapacity>,
}

/// SharedCapacity defines shared capacities for devices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedCapacity {
    /// Name is the name of this shared capacity.
    pub name: String,
    /// Capacity is the total capacity.
    pub capacity: String,
}

/// ResourcePool describes the pool that ResourceSlices belong to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePool {
    /// Name is used to identify the pool.
    pub name: String,
    /// Generation tracks the change in a pool over time.
    pub generation: i64,
    /// ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number.
    pub resource_slice_count: i64,
}

/// Device represents one individual hardware instance that can be selected based on its attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Name is unique identifier among all devices managed by the driver on the node.
    pub name: String,
    /// Basic defines the set of attributes and capacities the device has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic: Option<BasicDevice>,
}

/// BasicDevice defines the set of attributes and capacities for a basic device.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicDevice {
    /// Attributes defines the set of attributes for this device.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, DeviceAttribute>,
    /// Capacity defines the set of capacities for this device.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, DeviceCapacity>,
    /// ConsumesCounters defines which shared capacities this device consumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes_counters: Vec<DeviceCounterConsumption>,
    /// NodeName limits this device to be usable from this specific node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// NodeSelector limits this device to be usable from nodes matching this selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<serde_json::Value>,
    /// AllNodes indicates this device is usable from all nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
}

/// DeviceCounterConsumption defines counter consumption.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCounterConsumption {
    /// CounterSet is the name of the counter set.
    pub counter_set: String,
    /// Counters maps counter names to amounts consumed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, String>,
}

/// DeviceAttribute must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
    /// IntValue is a number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    /// BoolValue is a true/false value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    /// StringValue is a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    /// VersionValue is a semantic version according to semver.org spec 2.0.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_value: Option<String>,
}

/// DeviceCapacity describes a quantity associated with a device.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapacity {
    /// Value defines how much of a certain device capacity is available.
    pub value: String,
}

// Allocation mode constants
pub const ALLOCATION_MODE_EXACT_COUNT: &str = "ExactCount";
pub const ALLOCATION_MODE_ALL: &str = "All";

// Device allocation configuration source constants
pub const ALLOCATION_CONFIG_SOURCE_CLAIM: &str = "FromClaim";
pub const ALLOCATION_CONFIG_SOURCE_CLASS: &str = "FromClass";
