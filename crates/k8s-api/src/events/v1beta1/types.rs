//! Events v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{ListMeta, MicroTime, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

pub use crate::core::v1::EventSource;

// =============================================================================
// Event
// =============================================================================

/// Event is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// eventTime is the time when this Event was first observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<MicroTime>,
    /// series is data about the Event series this event represents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,
    /// reportingController is the name of the controller that emitted this Event.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_controller: String,
    /// reportingInstance is the ID of the controller instance.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_instance: String,
    /// action is what action was taken/failed regarding the regarding object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: String,
    /// reason is why the action was taken.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// regarding contains the object this Event is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regarding: Option<crate::core::v1::ObjectReference>,
    /// related is the optional secondary object for more complex actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<crate::core::v1::ObjectReference>,
    /// note is a human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// type is the type of this event (Normal, Warning).
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub event_type: String,
    /// deprecatedSource is the deprecated field assigning the source of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_source: Option<EventSource>,
    /// deprecatedFirstTimestamp is the deprecated field for when this Event was first observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: Option<Time>,
    /// deprecatedLastTimestamp is the deprecated field for when this Event was last observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: Option<Time>,
    /// deprecatedCount is the deprecated field assuring backward compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_count: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// count is the number of occurrences in this series up to the last heartbeat time.
    pub count: i32,
    /// lastObservedTime is the time when last Event from the series was seen.
    pub last_observed_time: MicroTime,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<Event>,
}
