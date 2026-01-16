//! Events v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub use crate::core::v1::{EventSource, ObjectReference};

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
    /// EventTime is the time when this Event was first observed.
    pub event_time: String,
    /// Series is data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,
    /// ReportingController is the name of the controller that emitted this Event.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_controller: String,
    /// ReportingInstance is the ID of the controller instance.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_instance: String,
    /// Action is what action was taken/failed regarding the Regarding object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: String,
    /// Reason is why the action was taken.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Regarding contains the object this Event is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regarding: Option<ObjectReference>,
    /// Related is the optional secondary object for more complex actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ObjectReference>,
    /// Note is a human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// Type is the type of this event (Normal, Warning).
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub type_: String,
    /// DeprecatedSource is the deprecated field for the source component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_source: Option<EventSource>,
    /// DeprecatedFirstTimestamp is the deprecated field for the first timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: Option<String>,
    /// DeprecatedLastTimestamp is the deprecated field for the last timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: Option<String>,
    /// DeprecatedCount is the deprecated field for the count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_count: Option<i32>,
}

/// EventList is a list of Event objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Event>,
}

/// EventSeries contain information on series of events.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// Count is the number of occurrences in this series up to the last heartbeat time.
    pub count: i32,
    /// LastObservedTime is the time when last Event from the series was seen.
    pub last_observed_time: String,
}
