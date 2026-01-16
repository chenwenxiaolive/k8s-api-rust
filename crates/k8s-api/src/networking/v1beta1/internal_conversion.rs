use super::*;

impl InternalConversion for IPAddress {
    type Internal = crate::networking::internal::IPAddress;
}

impl InternalConversion for IPAddressList {
    type Internal = crate::networking::internal::IPAddressList;
}

impl InternalConversion for IPAddressSpec {
    type Internal = crate::networking::internal::IPAddressSpec;
}

impl InternalConversion for ParentReference {
    type Internal = crate::networking::internal::ParentReference;
}

impl InternalConversion for ServiceCIDR {
    type Internal = crate::networking::internal::ServiceCIDR;
}

impl InternalConversion for ServiceCIDRList {
    type Internal = crate::networking::internal::ServiceCIDRList;
}

impl InternalConversion for ServiceCIDRSpec {
    type Internal = crate::networking::internal::ServiceCIDRSpec;
}

impl InternalConversion for ServiceCIDRStatus {
    type Internal = crate::networking::internal::ServiceCIDRStatus;
}

impl InternalConversion for Ingress {
    type Internal = crate::networking::internal::Ingress;
}

impl InternalConversion for IngressList {
    type Internal = crate::networking::internal::IngressList;
}

impl InternalConversion for IngressSpec {
    type Internal = crate::networking::internal::IngressSpec;
}

impl InternalConversion for IngressTLS {
    type Internal = crate::networking::internal::IngressTLS;
}

impl InternalConversion for IngressStatus {
    type Internal = crate::networking::internal::IngressStatus;
}

impl InternalConversion for IngressLoadBalancerStatus {
    type Internal = crate::networking::internal::IngressLoadBalancerStatus;
}

impl InternalConversion for IngressLoadBalancerIngress {
    type Internal = crate::networking::internal::IngressLoadBalancerIngress;
}

impl InternalConversion for IngressPortStatus {
    type Internal = crate::networking::internal::IngressPortStatus;
}

impl InternalConversion for IngressRule {
    type Internal = crate::networking::internal::IngressRule;
}

impl InternalConversion for IngressRuleValue {
    type Internal = crate::networking::internal::IngressRuleValue;
}

impl InternalConversion for HTTPIngressRuleValue {
    type Internal = crate::networking::internal::HTTPIngressRuleValue;
}

impl InternalConversion for HTTPIngressPath {
    type Internal = crate::networking::internal::HTTPIngressPath;
}

impl InternalConversion for IngressBackend {
    type Internal = crate::networking::internal::IngressBackend;
}

impl InternalConversion for IngressClass {
    type Internal = crate::networking::internal::IngressClass;
}

impl InternalConversion for IngressClassList {
    type Internal = crate::networking::internal::IngressClassList;
}

impl InternalConversion for IngressClassSpec {
    type Internal = crate::networking::internal::IngressClassSpec;
}

impl InternalConversion for IngressClassParametersReference {
    type Internal = crate::networking::internal::IngressClassParametersReference;
}
