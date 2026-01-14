//! Discovery API conversions
//!
//! This module provides conversions between discovery API versions.
//!
//! Note: v1beta1 is deprecated. v1 is the stable version.
//! Key differences:
//! - v1 uses `deprecated_topology` instead of v1beta1's `topology`
//! - v1 adds `zone` and `hints` fields to Endpoint

use crate::{ConversionError, Convertible};

// =============================================================================
// EndpointSlice: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::discovery::v1::EndpointSlice>
    for k8s_api::discovery::v1beta1::EndpointSlice
{
    fn convert_to(&self) -> Result<k8s_api::discovery::v1::EndpointSlice, ConversionError> {
        Ok(k8s_api::discovery::v1::EndpointSlice {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "discovery.k8s.io/v1",
                "EndpointSlice",
            ),
            metadata: self.metadata.clone(),
            address_type: self.address_type.clone(),
            endpoints: self.endpoints.iter().map(convert_endpoint_to_v1).collect(),
            ports: self.ports.iter().map(convert_port_to_v1).collect(),
        })
    }

    fn convert_from(
        other: &k8s_api::discovery::v1::EndpointSlice,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "discovery.k8s.io/v1beta1",
                "EndpointSlice",
            ),
            metadata: other.metadata.clone(),
            address_type: other.address_type.clone(),
            endpoints: other
                .endpoints
                .iter()
                .map(convert_endpoint_from_v1)
                .collect(),
            ports: other.ports.iter().map(convert_port_from_v1).collect(),
        })
    }
}

// =============================================================================
// Helper conversion functions
// =============================================================================

fn convert_endpoint_to_v1(
    endpoint: &k8s_api::discovery::v1beta1::Endpoint,
) -> k8s_api::discovery::v1::Endpoint {
    k8s_api::discovery::v1::Endpoint {
        addresses: endpoint.addresses.clone(),
        conditions: endpoint.conditions.as_ref().map(convert_conditions_to_v1),
        hostname: endpoint.hostname.clone(),
        target_ref: endpoint.target_ref.as_ref().map(convert_object_ref_to_v1),
        // v1beta1's topology becomes v1's deprecated_topology
        deprecated_topology: endpoint.topology.clone(),
        node_name: endpoint.node_name.clone(),
        // v1 has zone field, try to extract from topology if available
        zone: endpoint.topology.get("topology.kubernetes.io/zone").cloned(),
        // v1 has hints, not available in v1beta1
        hints: None,
    }
}

fn convert_endpoint_from_v1(
    endpoint: &k8s_api::discovery::v1::Endpoint,
) -> k8s_api::discovery::v1beta1::Endpoint {
    let mut topology = endpoint.deprecated_topology.clone();
    // If zone is set in v1, add it to topology for v1beta1
    if let Some(ref zone) = endpoint.zone {
        topology.insert("topology.kubernetes.io/zone".to_string(), zone.clone());
    }

    k8s_api::discovery::v1beta1::Endpoint {
        addresses: endpoint.addresses.clone(),
        conditions: endpoint.conditions.as_ref().map(convert_conditions_from_v1),
        hostname: endpoint.hostname.clone(),
        target_ref: endpoint.target_ref.as_ref().map(convert_object_ref_from_v1),
        topology,
        node_name: endpoint.node_name.clone(),
    }
}

fn convert_conditions_to_v1(
    conditions: &k8s_api::discovery::v1beta1::EndpointConditions,
) -> k8s_api::discovery::v1::EndpointConditions {
    k8s_api::discovery::v1::EndpointConditions {
        ready: conditions.ready,
        serving: conditions.serving,
        terminating: conditions.terminating,
    }
}

fn convert_conditions_from_v1(
    conditions: &k8s_api::discovery::v1::EndpointConditions,
) -> k8s_api::discovery::v1beta1::EndpointConditions {
    k8s_api::discovery::v1beta1::EndpointConditions {
        ready: conditions.ready,
        serving: conditions.serving,
        terminating: conditions.terminating,
    }
}

fn convert_object_ref_to_v1(
    obj_ref: &k8s_api::core::v1::ObjectReference,
) -> k8s_api::discovery::v1::ObjectReference {
    k8s_api::discovery::v1::ObjectReference {
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
    obj_ref: &k8s_api::discovery::v1::ObjectReference,
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

fn convert_port_to_v1(
    port: &k8s_api::discovery::v1beta1::EndpointPort,
) -> k8s_api::discovery::v1::EndpointPort {
    k8s_api::discovery::v1::EndpointPort {
        name: port.name.clone(),
        protocol: port.protocol.clone(),
        port: port.port,
        app_protocol: port.app_protocol.clone(),
    }
}

fn convert_port_from_v1(
    port: &k8s_api::discovery::v1::EndpointPort,
) -> k8s_api::discovery::v1beta1::EndpointPort {
    k8s_api::discovery::v1beta1::EndpointPort {
        name: port.name.clone(),
        protocol: port.protocol.clone(),
        port: port.port,
        app_protocol: port.app_protocol.clone(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_endpoint_slice_v1beta1_to_v1() {
        let v1beta1_es = k8s_api::discovery::v1beta1::EndpointSlice {
            metadata: ObjectMeta {
                name: "test-slice".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            address_type: "IPv4".to_string(),
            endpoints: vec![k8s_api::discovery::v1beta1::Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                conditions: Some(k8s_api::discovery::v1beta1::EndpointConditions {
                    ready: Some(true),
                    serving: Some(true),
                    terminating: Some(false),
                }),
                node_name: Some("node-1".to_string()),
                ..Default::default()
            }],
            ports: vec![k8s_api::discovery::v1beta1::EndpointPort {
                name: Some("http".to_string()),
                protocol: Some("TCP".to_string()),
                port: Some(80),
                app_protocol: Some("HTTP".to_string()),
            }],
            ..Default::default()
        };

        let v1_es: k8s_api::discovery::v1::EndpointSlice = v1beta1_es.convert_to().unwrap();

        assert_eq!(v1_es.metadata.name, "test-slice");
        assert_eq!(v1_es.address_type, "IPv4");
        assert_eq!(v1_es.endpoints.len(), 1);
        assert_eq!(v1_es.endpoints[0].addresses, vec!["10.0.0.1"]);
        assert_eq!(v1_es.endpoints[0].node_name, Some("node-1".to_string()));
        assert_eq!(v1_es.ports.len(), 1);
        assert_eq!(v1_es.ports[0].port, Some(80));
    }

    #[test]
    fn test_endpoint_slice_v1_to_v1beta1() {
        let v1_es = k8s_api::discovery::v1::EndpointSlice {
            metadata: ObjectMeta {
                name: "test-slice".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            address_type: "IPv6".to_string(),
            endpoints: vec![k8s_api::discovery::v1::Endpoint {
                addresses: vec!["2001:db8::1".to_string()],
                zone: Some("us-east-1a".to_string()),
                hints: Some(k8s_api::discovery::v1::EndpointHints {
                    for_zones: vec![k8s_api::discovery::v1::ForZone {
                        name: "us-east-1a".to_string(),
                    }],
                }),
                ..Default::default()
            }],
            ports: vec![],
            ..Default::default()
        };

        let v1beta1_es =
            k8s_api::discovery::v1beta1::EndpointSlice::convert_from(&v1_es).unwrap();

        assert_eq!(v1beta1_es.metadata.name, "test-slice");
        assert_eq!(v1beta1_es.address_type, "IPv6");
        assert_eq!(v1beta1_es.endpoints.len(), 1);
        // Zone should be in topology
        assert_eq!(
            v1beta1_es.endpoints[0]
                .topology
                .get("topology.kubernetes.io/zone"),
            Some(&"us-east-1a".to_string())
        );
    }

    #[test]
    fn test_endpoint_slice_roundtrip() {
        let mut topology = std::collections::BTreeMap::new();
        topology.insert(
            "topology.kubernetes.io/zone".to_string(),
            "us-west-2b".to_string(),
        );

        let original = k8s_api::discovery::v1beta1::EndpointSlice {
            metadata: ObjectMeta {
                name: "roundtrip-slice".to_string(),
                namespace: "test-ns".to_string(),
                ..Default::default()
            },
            address_type: "IPv4".to_string(),
            endpoints: vec![k8s_api::discovery::v1beta1::Endpoint {
                addresses: vec!["192.168.1.1".to_string(), "192.168.1.2".to_string()],
                conditions: Some(k8s_api::discovery::v1beta1::EndpointConditions {
                    ready: Some(true),
                    serving: Some(true),
                    terminating: Some(false),
                }),
                hostname: Some("pod-1".to_string()),
                node_name: Some("worker-1".to_string()),
                topology: topology.clone(),
                target_ref: None,
            }],
            ports: vec![
                k8s_api::discovery::v1beta1::EndpointPort {
                    name: Some("http".to_string()),
                    protocol: Some("TCP".to_string()),
                    port: Some(8080),
                    app_protocol: None,
                },
                k8s_api::discovery::v1beta1::EndpointPort {
                    name: Some("https".to_string()),
                    protocol: Some("TCP".to_string()),
                    port: Some(8443),
                    app_protocol: Some("HTTPS".to_string()),
                },
            ],
            ..Default::default()
        };

        let v1: k8s_api::discovery::v1::EndpointSlice = original.convert_to().unwrap();
        let roundtrip =
            k8s_api::discovery::v1beta1::EndpointSlice::convert_from(&v1).unwrap();

        assert_eq!(original.metadata.name, roundtrip.metadata.name);
        assert_eq!(original.metadata.namespace, roundtrip.metadata.namespace);
        assert_eq!(original.address_type, roundtrip.address_type);
        assert_eq!(original.endpoints.len(), roundtrip.endpoints.len());
        assert_eq!(
            original.endpoints[0].addresses,
            roundtrip.endpoints[0].addresses
        );
        assert_eq!(
            original.endpoints[0].hostname,
            roundtrip.endpoints[0].hostname
        );
        assert_eq!(original.ports.len(), roundtrip.ports.len());
        assert_eq!(original.ports[0].port, roundtrip.ports[0].port);
    }

    #[test]
    fn test_endpoint_with_target_ref() {
        let v1beta1_es = k8s_api::discovery::v1beta1::EndpointSlice {
            metadata: ObjectMeta {
                name: "with-ref".to_string(),
                ..Default::default()
            },
            address_type: "IPv4".to_string(),
            endpoints: vec![k8s_api::discovery::v1beta1::Endpoint {
                addresses: vec!["10.0.0.5".to_string()],
                target_ref: Some(k8s_api::core::v1::ObjectReference {
                    kind: "Pod".to_string(),
                    namespace: "default".to_string(),
                    name: "my-pod".to_string(),
                    uid: "abc-123".to_string(),
                    api_version: "v1".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ports: vec![],
            ..Default::default()
        };

        let v1_es: k8s_api::discovery::v1::EndpointSlice = v1beta1_es.convert_to().unwrap();

        assert!(v1_es.endpoints[0].target_ref.is_some());
        let target_ref = v1_es.endpoints[0].target_ref.as_ref().unwrap();
        assert_eq!(target_ref.kind, "Pod");
        assert_eq!(target_ref.name, "my-pod");
        assert_eq!(target_ref.uid, "abc-123");
    }
}
