use super::*;

impl InternalConversion for EndpointSlice {
    type Internal = crate::discovery::internal::EndpointSlice;
}

impl InternalConversion for EndpointSliceList {
    type Internal = crate::discovery::internal::EndpointSliceList;
}

impl InternalConversion for Endpoint {
    type Internal = crate::discovery::internal::Endpoint;
}

impl InternalConversion for EndpointConditions {
    type Internal = crate::discovery::internal::EndpointConditions;
}

impl InternalConversion for EndpointHints {
    type Internal = crate::discovery::internal::EndpointHints;
}

impl InternalConversion for ForZone {
    type Internal = crate::discovery::internal::ForZone;
}

impl InternalConversion for ForNode {
    type Internal = crate::discovery::internal::ForNode;
}

impl InternalConversion for EndpointPort {
    type Internal = crate::discovery::internal::EndpointPort;
}
