//! Events API conversions
//!
//! This module provides conversions between events API versions.
//!
//! Note: v1beta1 is deprecated. v1 is the stable version.
//! Key differences:
//! - v1 uses String for time fields, v1beta1 uses MicroTime/Time types (DateTime<Utc>)
//! - v1 uses local ObjectReference, v1beta1 uses core::v1::ObjectReference
//! - v1 uses `type_` field, v1beta1 uses `event_type`

use crate::{ConversionError, Convertible};
use chrono::{DateTime, Utc};

// =============================================================================
// Event: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::events::v1::Event> for k8s_api::events::v1beta1::Event {
    fn convert_to(&self) -> Result<k8s_api::events::v1::Event, ConversionError> {
        Ok(k8s_api::events::v1::Event {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "events.k8s.io/v1",
                "Event",
            ),
            metadata: self.metadata.clone(),
            event_time: self
                .event_time
                .as_ref()
                .and_then(|t| t.0.as_ref())
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
            series: self.series.as_ref().map(convert_series_to_v1),
            reporting_controller: self.reporting_controller.clone(),
            reporting_instance: self.reporting_instance.clone(),
            action: self.action.clone(),
            reason: self.reason.clone(),
            regarding: self.regarding.as_ref().map(convert_object_ref_to_v1),
            related: self.related.as_ref().map(convert_object_ref_to_v1),
            note: self.note.clone(),
            type_: self.event_type.clone(),
            deprecated_source: self
                .deprecated_source
                .as_ref()
                .map(convert_event_source_to_v1),
            deprecated_first_timestamp: self
                .deprecated_first_timestamp
                .as_ref()
                .and_then(|t| t.0.as_ref())
                .map(|dt| dt.to_rfc3339()),
            deprecated_last_timestamp: self
                .deprecated_last_timestamp
                .as_ref()
                .and_then(|t| t.0.as_ref())
                .map(|dt| dt.to_rfc3339()),
            deprecated_count: self.deprecated_count,
        })
    }

    fn convert_from(other: &k8s_api::events::v1::Event) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "events.k8s.io/v1beta1",
                "Event",
            ),
            metadata: other.metadata.clone(),
            event_time: if other.event_time.is_empty() {
                None
            } else {
                Some(k8s_apimachinery::apis::meta::v1::MicroTime(
                    parse_datetime(&other.event_time),
                ))
            },
            series: other.series.as_ref().map(convert_series_from_v1),
            reporting_controller: other.reporting_controller.clone(),
            reporting_instance: other.reporting_instance.clone(),
            action: other.action.clone(),
            reason: other.reason.clone(),
            regarding: other.regarding.as_ref().map(convert_object_ref_from_v1),
            related: other.related.as_ref().map(convert_object_ref_from_v1),
            note: other.note.clone(),
            event_type: other.type_.clone(),
            deprecated_source: other
                .deprecated_source
                .as_ref()
                .map(convert_event_source_from_v1),
            deprecated_first_timestamp: other
                .deprecated_first_timestamp
                .as_ref()
                .filter(|s| !s.is_empty())
                .map(|s| k8s_apimachinery::apis::meta::v1::Time(parse_datetime(s))),
            deprecated_last_timestamp: other
                .deprecated_last_timestamp
                .as_ref()
                .filter(|s| !s.is_empty())
                .map(|s| k8s_apimachinery::apis::meta::v1::Time(parse_datetime(s))),
            deprecated_count: other.deprecated_count,
        })
    }
}

// =============================================================================
// EventList: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::events::v1::EventList> for k8s_api::events::v1beta1::EventList {
    fn convert_to(&self) -> Result<k8s_api::events::v1::EventList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::events::v1::EventList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "events.k8s.io/v1",
                "EventList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::events::v1::EventList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::events::v1beta1::Event::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "events.k8s.io/v1beta1",
                "EventList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// Helper conversion functions
// =============================================================================

/// Parse datetime string to Option<DateTime<Utc>>
fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .ok()
}

fn convert_series_to_v1(
    series: &k8s_api::events::v1beta1::EventSeries,
) -> k8s_api::events::v1::EventSeries {
    k8s_api::events::v1::EventSeries {
        count: series.count,
        last_observed_time: series
            .last_observed_time
            .0
            .as_ref()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
    }
}

fn convert_series_from_v1(
    series: &k8s_api::events::v1::EventSeries,
) -> k8s_api::events::v1beta1::EventSeries {
    k8s_api::events::v1beta1::EventSeries {
        count: series.count,
        last_observed_time: k8s_apimachinery::apis::meta::v1::MicroTime(parse_datetime(
            &series.last_observed_time,
        )),
    }
}

fn convert_object_ref_to_v1(
    obj_ref: &k8s_api::core::v1::ObjectReference,
) -> k8s_api::events::v1::ObjectReference {
    k8s_api::events::v1::ObjectReference {
        kind: obj_ref.kind.clone(),
        namespace: obj_ref.namespace.clone(),
        name: obj_ref.name.clone(),
        uid: obj_ref.uid.clone(),
        api_version: obj_ref.api_version.clone(),
        resource_version: obj_ref.resource_version.clone(),
        field_path: obj_ref.field_path.clone(),
    }
}

fn convert_object_ref_from_v1(
    obj_ref: &k8s_api::events::v1::ObjectReference,
) -> k8s_api::core::v1::ObjectReference {
    k8s_api::core::v1::ObjectReference {
        kind: obj_ref.kind.clone(),
        namespace: obj_ref.namespace.clone(),
        name: obj_ref.name.clone(),
        uid: obj_ref.uid.clone(),
        api_version: obj_ref.api_version.clone(),
        resource_version: obj_ref.resource_version.clone(),
        field_path: obj_ref.field_path.clone(),
    }
}

fn convert_event_source_to_v1(
    source: &k8s_api::events::v1beta1::EventSource,
) -> k8s_api::events::v1::EventSource {
    k8s_api::events::v1::EventSource {
        component: source.component.clone(),
        host: source.host.clone(),
    }
}

fn convert_event_source_from_v1(
    source: &k8s_api::events::v1::EventSource,
) -> k8s_api::events::v1beta1::EventSource {
    k8s_api::events::v1beta1::EventSource {
        component: source.component.clone(),
        host: source.host.clone(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    fn test_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap()
    }

    #[test]
    fn test_event_v1beta1_to_v1() {
        let v1beta1_event = k8s_api::events::v1beta1::Event {
            metadata: ObjectMeta {
                name: "test-event".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            event_time: Some(k8s_apimachinery::apis::meta::v1::MicroTime(Some(
                test_datetime(),
            ))),
            reporting_controller: "my-controller".to_string(),
            reporting_instance: "my-controller-xyz".to_string(),
            action: "Created".to_string(),
            reason: "SuccessfulCreate".to_string(),
            note: "Pod was created successfully".to_string(),
            event_type: "Normal".to_string(),
            ..Default::default()
        };

        let v1_event: k8s_api::events::v1::Event = v1beta1_event.convert_to().unwrap();

        assert_eq!(v1_event.metadata.name, "test-event");
        assert!(v1_event.event_time.contains("2024-01-15"));
        assert_eq!(v1_event.reporting_controller, "my-controller");
        assert_eq!(v1_event.action, "Created");
        assert_eq!(v1_event.type_, "Normal");
    }

    #[test]
    fn test_event_v1_to_v1beta1() {
        let v1_event = k8s_api::events::v1::Event {
            metadata: ObjectMeta {
                name: "test-event-v1".to_string(),
                namespace: "kube-system".to_string(),
                ..Default::default()
            },
            event_time: "2024-01-15T12:00:00+00:00".to_string(),
            reporting_controller: "kube-scheduler".to_string(),
            reporting_instance: "kube-scheduler-0".to_string(),
            action: "Scheduled".to_string(),
            reason: "Scheduled".to_string(),
            note: "Successfully assigned pod to node".to_string(),
            type_: "Normal".to_string(),
            series: Some(k8s_api::events::v1::EventSeries {
                count: 5,
                last_observed_time: "2024-01-15T12:05:00+00:00".to_string(),
            }),
            ..Default::default()
        };

        let v1beta1_event = k8s_api::events::v1beta1::Event::convert_from(&v1_event).unwrap();

        assert_eq!(v1beta1_event.metadata.name, "test-event-v1");
        assert!(v1beta1_event.event_time.is_some());
        assert_eq!(v1beta1_event.event_type, "Normal");
        assert!(v1beta1_event.series.is_some());
        assert_eq!(v1beta1_event.series.as_ref().unwrap().count, 5);
    }

    #[test]
    fn test_event_roundtrip() {
        let original = k8s_api::events::v1beta1::Event {
            metadata: ObjectMeta {
                name: "roundtrip-event".to_string(),
                namespace: "test-ns".to_string(),
                ..Default::default()
            },
            event_time: Some(k8s_apimachinery::apis::meta::v1::MicroTime(Some(
                test_datetime(),
            ))),
            series: Some(k8s_api::events::v1beta1::EventSeries {
                count: 10,
                last_observed_time: k8s_apimachinery::apis::meta::v1::MicroTime(Some(
                    Utc.with_ymd_and_hms(2024, 1, 15, 14, 10, 0).unwrap(),
                )),
            }),
            reporting_controller: "test-controller".to_string(),
            reporting_instance: "test-instance".to_string(),
            action: "TestAction".to_string(),
            reason: "TestReason".to_string(),
            note: "Test note".to_string(),
            event_type: "Warning".to_string(),
            regarding: Some(k8s_api::core::v1::ObjectReference {
                kind: "Pod".to_string(),
                namespace: "test-ns".to_string(),
                name: "test-pod".to_string(),
                uid: "pod-uid-123".to_string(),
                api_version: "v1".to_string(),
                ..Default::default()
            }),
            deprecated_source: Some(k8s_api::events::v1beta1::EventSource {
                component: "kubelet".to_string(),
                host: "node-1".to_string(),
            }),
            deprecated_first_timestamp: Some(k8s_apimachinery::apis::meta::v1::Time(Some(
                Utc.with_ymd_and_hms(2024, 1, 15, 13, 0, 0).unwrap(),
            ))),
            deprecated_last_timestamp: Some(k8s_apimachinery::apis::meta::v1::Time(Some(
                Utc.with_ymd_and_hms(2024, 1, 15, 14, 0, 0).unwrap(),
            ))),
            deprecated_count: Some(10),
            ..Default::default()
        };

        let v1: k8s_api::events::v1::Event = original.convert_to().unwrap();
        let roundtrip = k8s_api::events::v1beta1::Event::convert_from(&v1).unwrap();

        assert_eq!(original.metadata.name, roundtrip.metadata.name);
        assert_eq!(original.metadata.namespace, roundtrip.metadata.namespace);
        // DateTime values should be equivalent (may differ in timezone format)
        assert!(roundtrip.event_time.is_some());
        assert_eq!(original.event_type, roundtrip.event_type);
        assert_eq!(
            original.series.as_ref().unwrap().count,
            roundtrip.series.as_ref().unwrap().count
        );
        assert_eq!(
            original.regarding.as_ref().unwrap().name,
            roundtrip.regarding.as_ref().unwrap().name
        );
        assert_eq!(
            original.deprecated_source.as_ref().unwrap().component,
            roundtrip.deprecated_source.as_ref().unwrap().component
        );
        assert_eq!(original.deprecated_count, roundtrip.deprecated_count);
    }

    #[test]
    fn test_event_with_related_object() {
        let v1beta1_event = k8s_api::events::v1beta1::Event {
            metadata: ObjectMeta {
                name: "related-event".to_string(),
                ..Default::default()
            },
            event_time: Some(k8s_apimachinery::apis::meta::v1::MicroTime(Some(
                test_datetime(),
            ))),
            regarding: Some(k8s_api::core::v1::ObjectReference {
                kind: "Deployment".to_string(),
                name: "my-deployment".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            }),
            related: Some(k8s_api::core::v1::ObjectReference {
                kind: "ReplicaSet".to_string(),
                name: "my-deployment-abc123".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            }),
            event_type: "Normal".to_string(),
            ..Default::default()
        };

        let v1_event: k8s_api::events::v1::Event = v1beta1_event.convert_to().unwrap();

        assert!(v1_event.regarding.is_some());
        assert_eq!(v1_event.regarding.as_ref().unwrap().kind, "Deployment");
        assert!(v1_event.related.is_some());
        assert_eq!(v1_event.related.as_ref().unwrap().kind, "ReplicaSet");
    }

    #[test]
    fn test_event_list_roundtrip() {
        let list = k8s_api::events::v1beta1::EventList {
            metadata: ListMeta {
                resource_version: "8".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::events::v1beta1::Event {
                metadata: ObjectMeta::named("event"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::events::v1::EventList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "8");
        assert_eq!(v1_list.items[0].metadata.name, "event");

        let roundtrip: k8s_api::events::v1beta1::EventList =
            k8s_api::events::v1beta1::EventList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
