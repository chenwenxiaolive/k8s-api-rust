//! Resource-related types for Kubernetes core/v1 API.
//!
//! This module contains types related to resource management, including
//! resource requirements, resource lists, and resource quotas.

use k8s_api_core::Quantity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ResourceName is the name identifying various resources in a ResourceList.
pub type ResourceName = String;

/// Well-known resource names.
pub mod resource_name {
    /// CPU, in cores. (500m = .5 cores)
    pub const CPU: &str = "cpu";
    /// Memory, in bytes. (500Gi = 500GiB = 500 * 1024 * 1024 * 1024)
    pub const MEMORY: &str = "memory";
    /// Volume size, in bytes (e.g., 5Gi = 5GiB = 5 * 1024 * 1024 * 1024)
    pub const STORAGE: &str = "storage";
    /// Local ephemeral storage, in bytes. (500Gi = 500GiB = 500 * 1024 * 1024 * 1024)
    pub const EPHEMERAL_STORAGE: &str = "ephemeral-storage";
    /// Number of pods
    pub const PODS: &str = "pods";
    /// Number of services
    pub const SERVICES: &str = "services";
    /// Number of replication controllers
    pub const REPLICATION_CONTROLLERS: &str = "replicationcontrollers";
    /// Number of resource quotas
    pub const RESOURCE_QUOTAS: &str = "resourcequotas";
    /// Number of secrets
    pub const SECRETS: &str = "secrets";
    /// Number of configmaps
    pub const CONFIG_MAPS: &str = "configmaps";
    /// Number of PersistentVolumeClaims
    pub const PERSISTENT_VOLUME_CLAIMS: &str = "persistentvolumeclaims";
    /// Number of services of type NodePort
    pub const SERVICES_NODE_PORTS: &str = "services.nodeports";
    /// Number of services of type LoadBalancer
    pub const SERVICES_LOAD_BALANCERS: &str = "services.loadbalancers";
}

/// ResourceList is a map of resource names to quantities.
///
/// Example:
/// ```
/// use k8s_api_core_v1::{ResourceList, Quantity};
///
/// let mut resources = ResourceList::new();
/// resources.insert("cpu".to_string(), Quantity::from("500m"));
/// resources.insert("memory".to_string(), Quantity::from("128Mi"));
/// ```
pub type ResourceList = BTreeMap<ResourceName, Quantity>;

/// ResourceRequirements describes the compute resource requirements.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: ResourceList,

    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: ResourceList,

    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ResourceClaim>,
}

impl ResourceRequirements {
    /// Creates a new ResourceRequirements with the given limits and requests.
    pub fn new(limits: ResourceList, requests: ResourceList) -> Self {
        Self {
            limits,
            requests,
            claims: Vec::new(),
        }
    }

    /// Creates ResourceRequirements with only limits specified.
    pub fn with_limits(limits: ResourceList) -> Self {
        Self {
            limits,
            ..Default::default()
        }
    }

    /// Creates ResourceRequirements with only requests specified.
    pub fn with_requests(requests: ResourceList) -> Self {
        Self {
            requests,
            ..Default::default()
        }
    }
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,

    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available, otherwise
    /// only the result of this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

/// VolumeResourceRequirements describes the storage resource requirements for a volume.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: ResourceList,

    /// Requests describes the minimum amount of compute resources required.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: ResourceList,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_requirements_serialize() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("1"));
        limits.insert("memory".to_string(), Quantity::from("512Mi"));

        let mut requests = ResourceList::new();
        requests.insert("cpu".to_string(), Quantity::from("500m"));
        requests.insert("memory".to_string(), Quantity::from("256Mi"));

        let requirements = ResourceRequirements::new(limits, requests);
        let json = serde_json::to_string(&requirements).unwrap();

        assert!(json.contains("\"limits\""));
        assert!(json.contains("\"requests\""));
        assert!(json.contains("\"cpu\""));
        assert!(json.contains("\"memory\""));
    }

    #[test]
    fn test_resource_requirements_roundtrip() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("2"));

        let original = ResourceRequirements::with_limits(limits);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: ResourceRequirements = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_resource_claim() {
        let claim = ResourceClaim {
            name: "gpu".to_string(),
            request: Some("nvidia-tesla-v100".to_string()),
        };
        let json = serde_json::to_string(&claim).unwrap();
        let parsed: ResourceClaim = serde_json::from_str(&json).unwrap();
        assert_eq!(claim, parsed);
    }

    #[test]
    fn test_empty_resource_requirements_serializes_to_empty_object() {
        let requirements = ResourceRequirements::default();
        let json = serde_json::to_string(&requirements).unwrap();
        assert_eq!(json, "{}");
    }
}
