use super::*;

impl InternalConversion for DeviceSelector {
    type Internal = crate::resource::internal::DeviceSelector;
}

impl InternalConversion for CELDeviceSelector {
    type Internal = crate::resource::internal::CELDeviceSelector;
}

impl InternalConversion for DeviceTaint {
    type Internal = crate::resource::internal::DeviceTaint;
}

impl InternalConversion for DeviceTaintRule {
    type Internal = crate::resource::internal::DeviceTaintRule;
}

impl InternalConversion for DeviceTaintRuleSpec {
    type Internal = crate::resource::internal::DeviceTaintRuleSpec;
}

impl InternalConversion for DeviceTaintSelector {
    type Internal = crate::resource::internal::DeviceTaintSelector;
}

impl InternalConversion for DeviceTaintRuleList {
    type Internal = crate::resource::internal::DeviceTaintRuleList;
}
