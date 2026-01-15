//! Coordination v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{MicroTime, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// Coordinated lease strategy types and constants
pub type CoordinatedLeaseStrategy = String;
pub const STRATEGY_OLDEST_EMULATION_VERSION: &str = "OldestEmulationVersion";

// =============================================================================
// Lease
// =============================================================================

/// Lease defines a lease concept.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LeaseSpec>,
}

/// LeaseList is a list of Lease objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Lease>,
}

/// LeaseSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseSpec {
    /// HolderIdentity contains the identity of the holder of a current lease.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder_identity: Option<String>,
    /// LeaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_duration_seconds: Option<i32>,
    /// AcquireTime is a time when the current lease was acquired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquire_time: Option<MicroTime>,
    /// RenewTime is a time when the current holder of a lease has last updated the lease.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<MicroTime>,
    /// LeaseTransitions is the number of transitions of a lease between holders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_transitions: Option<i32>,
    /// Strategy indicates the strategy for picking the leader for coordinated leader election.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<CoordinatedLeaseStrategy>,
    /// PreferredHolder signals to a lease holder that the lease has a more optimal holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_holder: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lease_serialization_roundtrip() {
        let lease = Lease {
            type_meta: TypeMeta::new("coordination.k8s.io/v1", "Lease"),
            metadata: ObjectMeta::namespaced("default", "leader-lease"),
            spec: Some(LeaseSpec {
                holder_identity: Some("controller-1".to_string()),
                lease_duration_seconds: Some(30),
                lease_transitions: Some(2),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&lease).unwrap();
        let parsed: Lease = serde_json::from_str(&json).unwrap();

        assert_eq!(lease.metadata.name, parsed.metadata.name);
        assert_eq!(
            lease.spec.as_ref().unwrap().holder_identity,
            parsed.spec.as_ref().unwrap().holder_identity
        );
    }
}
