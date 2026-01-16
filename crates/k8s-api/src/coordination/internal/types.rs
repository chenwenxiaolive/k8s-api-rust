//! Internal type definitions for coordination.

use k8s_apimachinery::apis::meta::v1::{MicroTime, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type CoordinatedLeaseStrategy = String;

pub const STRATEGY_OLDEST_EMULATION_VERSION: &str = "OldestEmulationVersion";


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


/// LeaseCandidate defines a candidate for a Lease object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec contains the specification of the LeaseCandidate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LeaseCandidateSpec>,
}


/// LeaseCandidateList is a list of LeaseCandidate objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<LeaseCandidate>,
}


/// LeaseCandidateSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateSpec {
    /// LeaseName is the name of the lease for which this candidate is contending.
    pub lease_name: String,
    /// PingTime is the last time that the server has requested the LeaseCandidate to renew.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<MicroTime>,
    /// RenewTime is the time that the LeaseCandidate was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<MicroTime>,
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`.
    pub binary_version: String,
    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub emulation_version: String,
    /// Strategy is the strategy that coordinated leader election will use for picking the leader.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub strategy: String,
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
