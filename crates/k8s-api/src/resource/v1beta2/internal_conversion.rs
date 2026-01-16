use super::*;

impl InternalConversion for ResourceClaim {
    type Internal = crate::resource::internal::ResourceClaim;
}

impl InternalConversion for ResourceClaimList {
    type Internal = crate::resource::internal::ResourceClaimList;
}

impl InternalConversion for ResourceClaimSpec {
    type Internal = crate::resource::internal::ResourceClaimSpec;
}

impl InternalConversion for ResourceClaimStatus {
    type Internal = crate::resource::internal::ResourceClaimStatus;
}

impl InternalConversion for DeviceClaim {
    type Internal = crate::resource::internal::DeviceClaim;
}

impl InternalConversion for DeviceRequest {
    type Internal = crate::resource::internal::DeviceRequest;
}

impl InternalConversion for ExactDeviceRequest {
    type Internal = crate::resource::internal::ExactDeviceRequest;
}

impl InternalConversion for DeviceSubRequest {
    type Internal = crate::resource::internal::DeviceSubRequest;
}

impl InternalConversion for CapacityRequirements {
    type Internal = crate::resource::internal::CapacityRequirements;
}

impl InternalConversion for DeviceSelector {
    type Internal = crate::resource::internal::DeviceSelector;
}

impl InternalConversion for CELDeviceSelector {
    type Internal = crate::resource::internal::CELDeviceSelector;
}

impl InternalConversion for DeviceConstraint {
    type Internal = crate::resource::internal::DeviceConstraint;
}

impl InternalConversion for DeviceClaimConfiguration {
    type Internal = crate::resource::internal::DeviceClaimConfiguration;
}

impl InternalConversion for DeviceConfiguration {
    type Internal = crate::resource::internal::DeviceConfiguration;
}

impl InternalConversion for OpaqueDeviceConfiguration {
    type Internal = crate::resource::internal::OpaqueDeviceConfiguration;
}

impl InternalConversion for AllocationResult {
    type Internal = crate::resource::internal::AllocationResult;
}

impl InternalConversion for DeviceAllocationResult {
    type Internal = crate::resource::internal::DeviceAllocationResult;
}

impl InternalConversion for DeviceRequestAllocationResult {
    type Internal = crate::resource::internal::DeviceRequestAllocationResult;
}

impl InternalConversion for DeviceAllocationConfiguration {
    type Internal = crate::resource::internal::DeviceAllocationConfiguration;
}

impl InternalConversion for ResourceClaimConsumerReference {
    type Internal = crate::resource::internal::ResourceClaimConsumerReference;
}

impl InternalConversion for AllocatedDeviceStatus {
    type Internal = crate::resource::internal::AllocatedDeviceStatus;
}

impl InternalConversion for NetworkDeviceData {
    type Internal = crate::resource::internal::NetworkDeviceData;
}

impl InternalConversion for DeviceClass {
    type Internal = crate::resource::internal::DeviceClass;
}

impl InternalConversion for DeviceClassList {
    type Internal = crate::resource::internal::DeviceClassList;
}

impl InternalConversion for DeviceClassSpec {
    type Internal = crate::resource::internal::DeviceClassSpec;
}

impl InternalConversion for DeviceClassConfiguration {
    type Internal = crate::resource::internal::DeviceClassConfiguration;
}

impl InternalConversion for ResourceClaimTemplate {
    type Internal = crate::resource::internal::ResourceClaimTemplate;
}

impl InternalConversion for ResourceClaimTemplateList {
    type Internal = crate::resource::internal::ResourceClaimTemplateList;
}

impl InternalConversion for ResourceClaimTemplateSpec {
    type Internal = crate::resource::internal::ResourceClaimTemplateSpec;
}

impl InternalConversion for ResourceSlice {
    type Internal = crate::resource::internal::ResourceSlice;
}

impl InternalConversion for ResourceSliceList {
    type Internal = crate::resource::internal::ResourceSliceList;
}

impl InternalConversion for ResourceSliceSpec {
    type Internal = crate::resource::internal::ResourceSliceSpec;
}

impl InternalConversion for CounterSet {
    type Internal = crate::resource::internal::CounterSet;
}

impl InternalConversion for Counter {
    type Internal = crate::resource::internal::Counter;
}

impl InternalConversion for ResourcePool {
    type Internal = crate::resource::internal::ResourcePool;
}

impl InternalConversion for Device {
    type Internal = crate::resource::internal::Device;
}

impl InternalConversion for DeviceCounterConsumption {
    type Internal = crate::resource::internal::DeviceCounterConsumption;
}

impl InternalConversion for DeviceAttribute {
    type Internal = crate::resource::internal::DeviceAttribute;
}

impl InternalConversion for DeviceCapacity {
    type Internal = crate::resource::internal::DeviceCapacity;
}

impl InternalConversion for CapacityRequestPolicy {
    type Internal = crate::resource::internal::CapacityRequestPolicy;
}

impl InternalConversion for CapacityRequestPolicyRange {
    type Internal = crate::resource::internal::CapacityRequestPolicyRange;
}

impl InternalConversion for DeviceTaint {
    type Internal = crate::resource::internal::DeviceTaint;
}

impl InternalConversion for DeviceToleration {
    type Internal = crate::resource::internal::DeviceToleration;
}
