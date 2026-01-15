use k8s_api_core::resource::IntOrString;
use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::de::DeserializeOwned;
use serde::Serialize;

fn assert_roundtrip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).unwrap();
    let decoded: T = serde_json::from_str(&json).unwrap();
    assert_eq!(&decoded, value);
}

#[test]
fn test_admission_review_roundtrip() {
    let review = k8s_api::admission::v1::AdmissionReview {
        type_meta: TypeMeta::new("admission.k8s.io/v1", "AdmissionReview"),
        request: Some(k8s_api::admission::v1::AdmissionRequest {
            uid: "123".to_string(),
            kind: k8s_api::admission::v1::GroupVersionKind {
                group: "".to_string(),
                version: "v1".to_string(),
                kind: "Pod".to_string(),
            },
            resource: k8s_api::admission::v1::GroupVersionResource {
                group: "".to_string(),
                version: "v1".to_string(),
                resource: "pods".to_string(),
            },
            operation: "CREATE".to_string(),
            user_info: k8s_api::admission::v1::UserInfo::default(),
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_roundtrip(&review);
}

#[test]
fn test_event_roundtrip() {
    let event = k8s_api::events::v1::Event {
        type_meta: TypeMeta::new("events.k8s.io/v1", "Event"),
        metadata: ObjectMeta::named("evt"),
        event_time: "now".to_string(),
        type_: "Normal".to_string(),
        reason: "Started".to_string(),
        ..Default::default()
    };

    assert_roundtrip(&event);
}

#[test]
fn test_api_service_roundtrip() {
    let service = k8s_api::apiregistration::v1::APIService {
        type_meta: TypeMeta::new("apiregistration.k8s.io/v1", "APIService"),
        metadata: ObjectMeta::named("v1.example.com"),
        spec: Some(k8s_api::apiregistration::v1::APIServiceSpec {
            group: "example.com".to_string(),
            version: "v1".to_string(),
            group_priority_minimum: 10,
            version_priority: 20,
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_roundtrip(&service);
}

#[test]
fn test_custom_resource_definition_roundtrip() {
    let crd = k8s_api::apiextensions::v1::CustomResourceDefinition {
        type_meta: TypeMeta::new("apiextensions.k8s.io/v1", "CustomResourceDefinition"),
        metadata: ObjectMeta::named("foos.example.com"),
        spec: k8s_api::apiextensions::v1::CustomResourceDefinitionSpec {
            group: "example.com".to_string(),
            names: k8s_api::apiextensions::v1::CustomResourceDefinitionNames {
                plural: "foos".to_string(),
                kind: "Foo".to_string(),
                ..Default::default()
            },
            scope: "Namespaced".to_string(),
            versions: vec![k8s_api::apiextensions::v1::CustomResourceDefinitionVersion {
                name: "v1".to_string(),
                served: true,
                storage: true,
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    assert_roundtrip(&crd);
}

#[test]
fn test_image_review_roundtrip() {
    let review = k8s_api::imagepolicy::v1alpha1::ImageReview {
        type_meta: TypeMeta::new("imagepolicy.k8s.io/v1alpha1", "ImageReview"),
        metadata: ObjectMeta::named("review"),
        spec: k8s_api::imagepolicy::v1alpha1::ImageReviewSpec {
            containers: vec![k8s_api::imagepolicy::v1alpha1::ImageReviewContainerSpec {
                image: "nginx:latest".to_string(),
            }],
            ..Default::default()
        },
        status: Some(k8s_api::imagepolicy::v1alpha1::ImageReviewStatus {
            allowed: true,
            ..Default::default()
        }),
    };

    assert_roundtrip(&review);
}

#[test]
fn test_storage_version_roundtrip() {
    let version = k8s_api::apiserverinternal::v1alpha1::StorageVersion {
        type_meta: TypeMeta::new("internal.apiserver.k8s.io/v1alpha1", "StorageVersion"),
        metadata: ObjectMeta::named("apps.v1.deployments"),
        status: k8s_api::apiserverinternal::v1alpha1::StorageVersionStatus {
            storage_versions: vec![k8s_api::apiserverinternal::v1alpha1::ServerStorageVersion {
                api_server_id: "server-1".to_string(),
                encoding_version: "v1".to_string(),
                decodable_versions: vec!["v1".to_string()],
                served_versions: vec!["v1".to_string()],
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    assert_roundtrip(&version);
}

#[test]
fn test_api_group_discovery_roundtrip() {
    let group = k8s_api::apidiscovery::v2::APIGroupDiscovery {
        type_meta: TypeMeta::new("apidiscovery.k8s.io/v2", "APIGroupDiscovery"),
        metadata: ObjectMeta::named("apps"),
        versions: vec![k8s_api::apidiscovery::v2::APIVersionDiscovery {
            version: "v1".to_string(),
            resources: vec![k8s_api::apidiscovery::v2::APIResourceDiscovery {
                resource: "deployments".to_string(),
                response_kind: Some(k8s_api::apidiscovery::v2::GroupVersionKind {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    kind: "Deployment".to_string(),
                }),
                scope: "Namespaced".to_string(),
                singular_resource: "deployment".to_string(),
                verbs: vec!["get".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        }],
    };

    assert_roundtrip(&group);
}

#[test]
fn test_storage_version_migration_roundtrip() {
    let migration = k8s_api::storagemigration::v1alpha1::StorageVersionMigration {
        type_meta: TypeMeta::new("storagemigration.k8s.io/v1alpha1", "StorageVersionMigration"),
        metadata: ObjectMeta::named("migrate"),
        spec: Some(k8s_api::storagemigration::v1alpha1::StorageVersionMigrationSpec {
            resource: k8s_api::storagemigration::v1alpha1::GroupVersionResource {
                group: "apps".to_string(),
                version: "v1".to_string(),
                resource: "deployments".to_string(),
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_roundtrip(&migration);
}

#[test]
fn test_extensions_ingress_roundtrip() {
    let ingress = k8s_api::extensions::v1beta1::Ingress {
        type_meta: TypeMeta::new("extensions/v1beta1", "Ingress"),
        metadata: ObjectMeta::named("ing"),
        spec: Some(k8s_api::extensions::v1beta1::IngressSpec {
            backend: Some(k8s_api::extensions::v1beta1::IngressBackend {
                service_name: "default".to_string(),
                service_port: Some(IntOrString::Int(80)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_roundtrip(&ingress);
}
