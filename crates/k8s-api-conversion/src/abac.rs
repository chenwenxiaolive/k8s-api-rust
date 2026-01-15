//! ABAC API conversions
//!
//! This module provides conversions between ABAC API versions.

use crate::{ConversionError, Convertible};

// =============================================================================
// Policy: v0 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::abac::v1beta1::Policy> for k8s_api::abac::v0::Policy {
    fn convert_to(&self) -> Result<k8s_api::abac::v1beta1::Policy, ConversionError> {
        Ok(k8s_api::abac::v1beta1::Policy {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "abac.authorization.k8s.io/v1beta1",
                "Policy",
            ),
            spec: k8s_api::abac::v1beta1::PolicySpec {
                user: self.user.clone(),
                group: self.group.clone(),
                read_only: self.read_only,
                api_group: String::new(),
                resource: self.resource.clone(),
                namespace: self.namespace.clone(),
                non_resource_path: String::new(),
            },
        })
    }

    fn convert_from(
        other: &k8s_api::abac::v1beta1::Policy,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "abac.authorization.k8s.io/v0",
                "Policy",
            ),
            user: other.spec.user.clone(),
            group: other.spec.group.clone(),
            read_only: other.spec.read_only,
            resource: other.spec.resource.clone(),
            namespace: other.spec.namespace.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::TypeMeta;

    #[test]
    fn test_policy_roundtrip() {
        let v0_policy = k8s_api::abac::v0::Policy {
            type_meta: TypeMeta::new("abac.authorization.k8s.io/v0", "Policy"),
            user: "alice".to_string(),
            group: "devs".to_string(),
            read_only: true,
            resource: "pods".to_string(),
            namespace: "default".to_string(),
        };

        let v1beta1_policy: k8s_api::abac::v1beta1::Policy = v0_policy.convert_to().unwrap();
        assert_eq!(v1beta1_policy.spec.user, "alice");
        assert_eq!(v1beta1_policy.spec.group, "devs");
        assert!(v1beta1_policy.spec.read_only);
        assert_eq!(v1beta1_policy.spec.resource, "pods");
        assert_eq!(v1beta1_policy.spec.namespace, "default");
        assert!(v1beta1_policy.spec.api_group.is_empty());
        assert!(v1beta1_policy.spec.non_resource_path.is_empty());

        let converted_back: k8s_api::abac::v0::Policy =
            k8s_api::abac::v0::Policy::convert_from(&v1beta1_policy).unwrap();
        assert_eq!(converted_back.user, "alice");
        assert_eq!(converted_back.group, "devs");
        assert!(converted_back.read_only);
        assert_eq!(converted_back.resource, "pods");
        assert_eq!(converted_back.namespace, "default");
    }
}
