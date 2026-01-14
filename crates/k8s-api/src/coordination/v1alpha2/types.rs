//! Coordination v1alpha2 type definitions
//!
//! This module provides alpha-level coordination types including LeaseCandidate (K8s 1.32+).

use k8s_apimachinery::apis::meta::v1::{MicroTime, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// LeaseCandidate (K8s 1.32+)
// =============================================================================

/// LeaseCandidate defines a candidate for a Lease object.
/// Candidates are created such that coordinated leader election will pick the best leader
/// from the list of candidates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec contains the specification of the Lease.
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
    /// Items is a list of schema objects.
    pub items: Vec<LeaseCandidate>,
}

/// LeaseCandidateSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateSpec {
    /// LeaseName is the name of the lease for which this candidate is contending.
    /// This field is immutable.
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

// Coordinated lease strategy types
pub const STRATEGY_OLDEST_EMULATION_VERSION: &str = "OldestEmulationVersion";
