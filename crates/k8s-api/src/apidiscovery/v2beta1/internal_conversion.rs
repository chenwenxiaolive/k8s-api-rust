use super::*;

impl InternalConversion for APIGroupDiscoveryList {
    type Internal = crate::apidiscovery::internal::APIGroupDiscoveryList;
}

impl InternalConversion for APIGroupDiscovery {
    type Internal = crate::apidiscovery::internal::APIGroupDiscovery;
}

impl InternalConversion for APIVersionDiscovery {
    type Internal = crate::apidiscovery::internal::APIVersionDiscovery;
}

impl InternalConversion for APIResourceDiscovery {
    type Internal = crate::apidiscovery::internal::APIResourceDiscovery;
}

impl InternalConversion for APISubresourceDiscovery {
    type Internal = crate::apidiscovery::internal::APISubresourceDiscovery;
}
