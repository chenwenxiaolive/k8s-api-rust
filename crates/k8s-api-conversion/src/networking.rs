//! Networking API conversions
//!
//! This module provides conversions between networking API versions.

use crate::{ConversionError, Convertible};
use k8s_api_core::IntOrString;

// =============================================================================
// Ingress: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::networking::v1::Ingress> for k8s_api::networking::v1beta1::Ingress {
    fn convert_to(&self) -> Result<k8s_api::networking::v1::Ingress, ConversionError> {
        Ok(k8s_api::networking::v1::Ingress {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "networking.k8s.io/v1",
                "Ingress",
            ),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| convert_ingress_spec_to_v1(s)),
            status: self.status.as_ref().map(|s| convert_ingress_status_to_v1(s)),
        })
    }

    fn convert_from(other: &k8s_api::networking::v1::Ingress) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "networking.k8s.io/v1beta1",
                "Ingress",
            ),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| convert_ingress_spec_from_v1(s)),
            status: other.status.as_ref().map(|s| convert_ingress_status_from_v1(s)),
        })
    }
}

fn convert_ingress_spec_to_v1(
    spec: &k8s_api::networking::v1beta1::IngressSpec,
) -> k8s_api::networking::v1::IngressSpec {
    k8s_api::networking::v1::IngressSpec {
        ingress_class_name: spec.ingress_class_name.clone(),
        default_backend: spec.backend.as_ref().map(|b| convert_ingress_backend_to_v1(b)),
        tls: spec
            .tls
            .iter()
            .map(|t| k8s_api::networking::v1::IngressTLS {
                hosts: t.hosts.clone(),
                secret_name: t.secret_name.clone(),
            })
            .collect(),
        rules: spec.rules.iter().map(|r| convert_ingress_rule_to_v1(r)).collect(),
    }
}

fn convert_ingress_spec_from_v1(
    spec: &k8s_api::networking::v1::IngressSpec,
) -> k8s_api::networking::v1beta1::IngressSpec {
    k8s_api::networking::v1beta1::IngressSpec {
        ingress_class_name: spec.ingress_class_name.clone(),
        backend: spec.default_backend.as_ref().map(|b| convert_ingress_backend_from_v1(b)),
        tls: spec
            .tls
            .iter()
            .map(|t| k8s_api::networking::v1beta1::IngressTLS {
                hosts: t.hosts.clone(),
                secret_name: t.secret_name.clone(),
            })
            .collect(),
        rules: spec.rules.iter().map(|r| convert_ingress_rule_from_v1(r)).collect(),
    }
}

fn convert_ingress_backend_to_v1(
    backend: &k8s_api::networking::v1beta1::IngressBackend,
) -> k8s_api::networking::v1::IngressBackend {
    // v1beta1 uses service_name/service_port directly
    // v1 uses nested IngressServiceBackend structure
    let service = if !backend.service_name.is_empty() {
        Some(k8s_api::networking::v1::IngressServiceBackend {
            name: backend.service_name.clone(),
            port: backend.service_port.as_ref().map(|p| {
                k8s_api::networking::v1::ServiceBackendPort {
                    name: match p {
                        IntOrString::String(s) => s.clone(),
                        IntOrString::Int(_) => String::new(),
                    },
                    number: match p {
                        IntOrString::Int(i) => *i,
                        IntOrString::String(_) => 0,
                    },
                }
            }),
        })
    } else {
        None
    };

    k8s_api::networking::v1::IngressBackend {
        service,
        resource: backend.resource.as_ref().map(|r| {
            k8s_api::networking::v1::TypedLocalObjectReference {
                api_group: r.api_group.clone(),
                kind: r.kind.clone(),
                name: r.name.clone(),
            }
        }),
    }
}

fn convert_ingress_backend_from_v1(
    backend: &k8s_api::networking::v1::IngressBackend,
) -> k8s_api::networking::v1beta1::IngressBackend {
    let (service_name, service_port) = if let Some(ref svc) = backend.service {
        let port = svc.port.as_ref().map(|p| {
            if !p.name.is_empty() {
                IntOrString::String(p.name.clone())
            } else {
                IntOrString::Int(p.number)
            }
        });
        (svc.name.clone(), port)
    } else {
        (String::new(), None)
    };

    k8s_api::networking::v1beta1::IngressBackend {
        service_name,
        service_port,
        resource: backend.resource.as_ref().map(|r| {
            k8s_api::networking::v1beta1::TypedLocalObjectReference {
                api_group: r.api_group.clone(),
                kind: r.kind.clone(),
                name: r.name.clone(),
            }
        }),
    }
}

fn convert_ingress_rule_to_v1(
    rule: &k8s_api::networking::v1beta1::IngressRule,
) -> k8s_api::networking::v1::IngressRule {
    k8s_api::networking::v1::IngressRule {
        host: rule.host.clone(),
        ingress_rule_value: k8s_api::networking::v1::IngressRuleValue {
            http: rule.ingress_rule_value.http.as_ref().map(|h| {
                k8s_api::networking::v1::HTTPIngressRuleValue {
                    paths: h.paths.iter().map(|p| convert_http_ingress_path_to_v1(p)).collect(),
                }
            }),
        },
    }
}

fn convert_ingress_rule_from_v1(
    rule: &k8s_api::networking::v1::IngressRule,
) -> k8s_api::networking::v1beta1::IngressRule {
    k8s_api::networking::v1beta1::IngressRule {
        host: rule.host.clone(),
        ingress_rule_value: k8s_api::networking::v1beta1::IngressRuleValue {
            http: rule.ingress_rule_value.http.as_ref().map(|h| {
                k8s_api::networking::v1beta1::HTTPIngressRuleValue {
                    paths: h
                        .paths
                        .iter()
                        .map(|p| convert_http_ingress_path_from_v1(p))
                        .collect(),
                }
            }),
        },
    }
}

fn convert_http_ingress_path_to_v1(
    path: &k8s_api::networking::v1beta1::HTTPIngressPath,
) -> k8s_api::networking::v1::HTTPIngressPath {
    k8s_api::networking::v1::HTTPIngressPath {
        path: path.path.clone(),
        path_type: path.path_type.clone(),
        backend: convert_ingress_backend_to_v1(&path.backend),
    }
}

fn convert_http_ingress_path_from_v1(
    path: &k8s_api::networking::v1::HTTPIngressPath,
) -> k8s_api::networking::v1beta1::HTTPIngressPath {
    k8s_api::networking::v1beta1::HTTPIngressPath {
        path: path.path.clone(),
        path_type: path.path_type.clone(),
        backend: convert_ingress_backend_from_v1(&path.backend),
    }
}

fn convert_ingress_status_to_v1(
    status: &k8s_api::networking::v1beta1::IngressStatus,
) -> k8s_api::networking::v1::IngressStatus {
    k8s_api::networking::v1::IngressStatus {
        load_balancer: status.load_balancer.as_ref().map(|lb| {
            k8s_api::networking::v1::IngressLoadBalancerStatus {
                ingress: lb
                    .ingress
                    .iter()
                    .map(|i| k8s_api::networking::v1::IngressLoadBalancerIngress {
                        ip: i.ip.clone(),
                        hostname: i.hostname.clone(),
                        ports: i
                            .ports
                            .iter()
                            .map(|p| k8s_api::networking::v1::IngressPortStatus {
                                port: p.port,
                                protocol: p.protocol.clone(),
                                error: p.error.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            }
        }),
    }
}

fn convert_ingress_status_from_v1(
    status: &k8s_api::networking::v1::IngressStatus,
) -> k8s_api::networking::v1beta1::IngressStatus {
    k8s_api::networking::v1beta1::IngressStatus {
        load_balancer: status.load_balancer.as_ref().map(|lb| {
            k8s_api::networking::v1beta1::IngressLoadBalancerStatus {
                ingress: lb
                    .ingress
                    .iter()
                    .map(|i| k8s_api::networking::v1beta1::IngressLoadBalancerIngress {
                        ip: i.ip.clone(),
                        hostname: i.hostname.clone(),
                        ports: i
                            .ports
                            .iter()
                            .map(|p| k8s_api::networking::v1beta1::IngressPortStatus {
                                port: p.port,
                                protocol: p.protocol.clone(),
                                error: p.error.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            }
        }),
    }
}

// =============================================================================
// IngressClass: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::networking::v1::IngressClass>
    for k8s_api::networking::v1beta1::IngressClass
{
    fn convert_to(&self) -> Result<k8s_api::networking::v1::IngressClass, ConversionError> {
        Ok(k8s_api::networking::v1::IngressClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "networking.k8s.io/v1",
                "IngressClass",
            ),
            metadata: self.metadata.clone(),
            spec: self.spec.as_ref().map(|s| k8s_api::networking::v1::IngressClassSpec {
                controller: s.controller.clone(),
                parameters: s.parameters.as_ref().map(|p| {
                    k8s_api::networking::v1::IngressClassParametersReference {
                        api_group: p.api_group.clone(),
                        kind: p.kind.clone(),
                        name: p.name.clone(),
                        scope: p.scope.clone(),
                        namespace: p.namespace.clone(),
                    }
                }),
            }),
        })
    }

    fn convert_from(other: &k8s_api::networking::v1::IngressClass) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "networking.k8s.io/v1beta1",
                "IngressClass",
            ),
            metadata: other.metadata.clone(),
            spec: other.spec.as_ref().map(|s| {
                k8s_api::networking::v1beta1::IngressClassSpec {
                    controller: s.controller.clone(),
                    parameters: s.parameters.as_ref().map(|p| {
                        k8s_api::networking::v1beta1::IngressClassParametersReference {
                            api_group: p.api_group.clone(),
                            kind: p.kind.clone(),
                            name: p.name.clone(),
                            scope: p.scope.clone(),
                            namespace: p.namespace.clone(),
                        }
                    }),
                }
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_ingress_conversion_roundtrip() {
        let v1beta1_ingress = k8s_api::networking::v1beta1::Ingress {
            metadata: ObjectMeta::named("test-ingress"),
            spec: Some(k8s_api::networking::v1beta1::IngressSpec {
                ingress_class_name: Some("nginx".to_string()),
                backend: Some(k8s_api::networking::v1beta1::IngressBackend {
                    service_name: "default-backend".to_string(),
                    service_port: Some(IntOrString::Int(80)),
                    ..Default::default()
                }),
                rules: vec![k8s_api::networking::v1beta1::IngressRule {
                    host: "example.com".to_string(),
                    ingress_rule_value: k8s_api::networking::v1beta1::IngressRuleValue {
                        http: Some(k8s_api::networking::v1beta1::HTTPIngressRuleValue {
                            paths: vec![k8s_api::networking::v1beta1::HTTPIngressPath {
                                path: "/api".to_string(),
                                path_type: Some("Prefix".to_string()),
                                backend: k8s_api::networking::v1beta1::IngressBackend {
                                    service_name: "api-service".to_string(),
                                    service_port: Some(IntOrString::Int(8080)),
                                    ..Default::default()
                                },
                            }],
                        }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_ingress: k8s_api::networking::v1::Ingress = v1beta1_ingress.convert_to().unwrap();
        assert_eq!(v1_ingress.metadata.name, "test-ingress");
        assert_eq!(
            v1_ingress.spec.as_ref().unwrap().ingress_class_name,
            Some("nginx".to_string())
        );

        // Check backend conversion
        let default_backend = v1_ingress.spec.as_ref().unwrap().default_backend.as_ref().unwrap();
        assert_eq!(
            default_backend.service.as_ref().unwrap().name,
            "default-backend"
        );

        // Convert back to v1beta1
        let converted_back: k8s_api::networking::v1beta1::Ingress =
            k8s_api::networking::v1beta1::Ingress::convert_from(&v1_ingress).unwrap();
        assert_eq!(converted_back.metadata.name, "test-ingress");
        assert_eq!(
            converted_back.spec.as_ref().unwrap().rules[0].host,
            "example.com"
        );
    }

    #[test]
    fn test_ingress_class_conversion_roundtrip() {
        let v1beta1_class = k8s_api::networking::v1beta1::IngressClass {
            metadata: ObjectMeta::named("nginx"),
            spec: Some(k8s_api::networking::v1beta1::IngressClassSpec {
                controller: "k8s.io/ingress-nginx".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Convert to v1
        let v1_class: k8s_api::networking::v1::IngressClass = v1beta1_class.convert_to().unwrap();
        assert_eq!(v1_class.metadata.name, "nginx");
        assert_eq!(
            v1_class.spec.as_ref().unwrap().controller,
            "k8s.io/ingress-nginx"
        );

        // Convert back
        let converted_back: k8s_api::networking::v1beta1::IngressClass =
            k8s_api::networking::v1beta1::IngressClass::convert_from(&v1_class).unwrap();
        assert_eq!(converted_back.metadata.name, "nginx");
    }
}
