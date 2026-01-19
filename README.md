# k8s-api-rust

Rust implementation of Kubernetes API types, refactored from the official Go codebase.

## Overview

This project provides a complete Rust implementation of Kubernetes API types, offering type-safe, idiomatic Rust interfaces for working with Kubernetes resources. The types are carefully refactored from the official Kubernetes Go codebase (`staging/src/k8s.io/api` and `pkg/apis`).

## Features

- **Type-safe**: Full Rust type system guarantees for Kubernetes API objects
- **Serde integration**: Built-in JSON/YAML serialization and deserialization
- **Modular design**: Separated into multiple crates for granular dependencies
- **Feature flags**: Optional compilation of API groups to minimize binary size
- **Validation**: Built-in validation logic for API types
- **Version conversion**: Support for converting between API versions
- **External codecs**: JSON/Protobuf codecs and strategic-merge-patch content types

## Architecture

The project is organized as a Cargo workspace with the following crates:

### Core Crates

- **k8s-api-core** - Fundamental types and utilities
  - Schema types: `GroupVersionKind`, `GroupVersionResource`
  - Resource types: `Quantity`, `IntOrString`
  - Runtime interfaces and traits

- **k8s-apimachinery** - API machinery and metadata types
  - `TypeMeta`, `ObjectMeta`, `ListMeta`
  - `Time`, `Duration`, `MicroTime`
  - `Status`, `Condition`, `LabelSelector`
  - `OwnerReference`, `ManagedFieldsEntry`

### API Crates

- **k8s-api** - Kubernetes API types for all API groups
  - `core/v1`: Pod, Service, ConfigMap, Secret, Volume, etc.
  - `apps/v1`: Deployment, StatefulSet, DaemonSet, ReplicaSet
  - `batch/v1`: Job, CronJob
  - More API groups coming soon...

### Extension Crates

- **k8s-api-validation** - Validation logic for API types
  - Field validation rules
  - Business logic validation
  - Custom validators

- **k8s-api-conversion** - Version conversion between API versions
  - Version upgrade/downgrade logic
  - Conversion schemes
- **k8s-api-codec** - External codecs for JSON/Protobuf and patch content types

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
k8s-api = "0.1"
k8s-apimachinery = "0.1"
```

### Example: Creating a Pod

```rust
use k8s_api::core::v1::{Pod, PodSpec, Container};
use k8s_apimachinery::ObjectMeta;

let pod = Pod {
    metadata: ObjectMeta {
        name: Some("my-pod".to_string()),
        namespace: Some("default".to_string()),
        ..Default::default()
    },
    spec: Some(PodSpec {
        containers: vec![
            Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                ..Default::default()
            }
        ],
        ..Default::default()
    }),
    ..Default::default()
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&pod)?;
println!("{}", json);
```

### Example: Creating a PriorityClass

```rust
use k8s_api::scheduling::v1::PriorityClass;
use k8s_apimachinery::{ObjectMeta, TypeMeta};

let class = PriorityClass {
    type_meta: TypeMeta::new("scheduling.k8s.io/v1", "PriorityClass"),
    metadata: ObjectMeta::named("high-priority"),
    value: 1000,
    global_default: Some(false),
    description: "critical workloads".to_string(),
    preemption_policy: Some("PreemptLowerPriority".to_string()),
};

let json = serde_json::to_string_pretty(&class)?;
println!("{}", json);
```

### Feature Flags

Enable only the API groups you need:

```toml
[dependencies]
k8s-api = { version = "0.1", default-features = false, features = ["core", "apps"] }
```

Available features:
- `core` - Core API (Pod, Service, ConfigMap, etc.)
- `apps` - Apps API (Deployment, StatefulSet, etc.)
- `batch` - Batch API (Job, CronJob)
- `all` - Enable all API groups

### External Codecs (JSON/Protobuf + Strategic Merge Patch)

Use `k8s-api-codec` for protobuf round-trips and patch content types:

```rust
use k8s_api::core::v1::Namespace;
use k8s_api_codec::ExternalVersionCodec;
use k8s_apimachinery::apis::meta::v1::ObjectMeta;
use serde_json::json;

let namespace = Namespace {
    metadata: ObjectMeta::named("codec-demo"),
    ..Default::default()
};

let codec = ExternalVersionCodec::from_api_version("v1", "Namespace")?;
let bytes = codec.encode_protobuf(&namespace)?;
let decoded: Namespace = codec.decode_protobuf(&bytes)?;

let patch = ExternalVersionCodec::patch_strategic(json!({ "metadata": { "labels": { "app": "demo" }}}));
assert_eq!(patch.content_type(), "application/strategic-merge-patch+json");
```

## Project Structure

```
k8s-api-rust/
├── Cargo.toml                 # Workspace definition
├── crates/
│   ├── k8s-api-core/          # Core types and utilities
│   ├── k8s-apimachinery/      # API machinery types
│   ├── k8s-api/               # API group types
│   │   ├── core/v1/           # Core API v1
│   │   ├── apps/v1/           # Apps API v1
│   │   └── batch/v1/          # Batch API v1
│   ├── k8s-api-validation/    # Validation logic
│   ├── k8s-api-conversion/    # Version conversion
│   └── k8s-api-codec/         # JSON/Protobuf codecs and patch types
└── README.md
```

## Current Status

**Total Tests: 628 (all passing)**

## Refactor Goals (v1.34.1)

### Model Definitions

- External versions support JSON + Protobuf codecs: **Done** (`crates/k8s-api-codec/src/lib.rs`)
- External versions support strategic-merge-patch: **Done** (`crates/k8s-api-codec/src/lib.rs`)
- External logic considers JSON/Protobuf/strategic-merge-patch together: **Done**
  - Status: `ExternalVersionCodec` provides a version-scoped entry point for JSON/Protobuf encode/decode and patch types.

### Defaults

- Only external versions have defaults: **Done**
  - Status: internal types no longer include `#[serde(default)]` attributes.

### Version Conversion

- Bidirectional conversion between internal and external versions: **Done**
- Conversion implemented in external version modules and every external version supports internal <-> external: **Done**

### Validation

- Only internal versions require validation: **Done**
  - Status: `k8s-api-validation` exposes internal wrappers that validate internal types via external validators.

### Next Plan

1. Expand tests to cover external JSON/Protobuf/patch helpers across representative groups. **Done**
2. Decide on defaulting infrastructure and port defaults tests (apps, autoscaling, batch, core, etc.).
   - Started with scheduling PriorityClass defaulting (v1/v1alpha1/v1beta1).
   - Added discovery EndpointSlice port defaults (v1/v1beta1).
   - Added networking NetworkPolicy and Ingress path defaults (v1/v1beta1).
   - Added admissionregistration defaults (v1/v1alpha1/v1beta1).
   - Added apps defaults (v1/v1beta1/v1beta2).
   - Added autoscaling defaults (v1/v2/v2beta1/v2beta2).
   - Added storage defaults (v1/v1beta1).
   - Added batch defaults (v1/v1beta1).
   - Added flowcontrol defaults (v1/v1beta1/v1beta2/v1beta3).
   - Added extensions defaults (v1beta1).
   - Added resource defaults (v1/v1alpha3/v1beta1/v1beta2).
   - Added certificates defaults (v1beta1).
   - Extended core/v1 defaults (pod/container/probe/volume/secret/pv/pvc/endpoints/node/service/limitrange/namespace/replicationcontroller basics + pod requests from limits + pod-level resources (cpu/memory/hugepages, init container aggregation + feature gate off) + volume source iscsi/rbd/azure/scaleio + ephemeral claim template + pod log options + image volume pull policy feature gates + lifecycle/GRPC defaults + ephemeral container defaults + pod hostNetwork pod-vs-spec behavior + service session affinity config defaults + node allocatable cases + resource list rounding (decimal + binary + exponent) + service load balancer IP mode).
   - Added flowcontrol v1 bootstrap defaulting no-op coverage (mandatory + suggested configs).
3. Add remaining core/extensions conversion coverage (core/v1 conversion tests).
   - Added extensions Ingress backend conversion coverage (v1beta1).
   - Added core Pod/PodSpec/PodStatus/NodeSpec conversion coverage (service account/PodIP/PodCIDR aliasing + init-container annotation pruning + pod terminationGracePeriod clamp + Secret stringData merge).

### API Types Coverage (k8s-api)

| Module | Types | Status |
|--------|-------|--------|
| core | 309 | ✅ Complete |
| resource | 186 | ✅ Complete |
| flowcontrol | 120 | ✅ Complete |
| admissionregistration | 118 | ✅ Complete |
| apps | 110 | ✅ Complete |
| autoscaling | 101 | ✅ Complete |
| networking | 65 | ✅ Complete |
| storage | 62 | ✅ Complete |
| extensions | 52 | ✅ Complete |
| rbac | 36 | ✅ Complete |
| batch | 34 | ✅ Complete |
| authorization | 30 | ✅ Complete |
| policy | 28 | ✅ Complete |
| certificates | 26 | ✅ Complete |
| authentication | 21 | ✅ Complete |
| discovery | 19 | ✅ Complete |
| apiextensions | 18 | ✅ Complete |
| admission | 16 | ✅ Complete |
| apidiscovery | 16 | ✅ Complete |
| node | 15 | ✅ Complete |
| coordination | 13 | ✅ Complete |
| events | 9 | ✅ Complete |
| apiserverinternal | 8 | ✅ Complete |
| storagemigration | 7 | ✅ Complete |
| scheduling | 6 | ✅ Complete |
| apiregistration | 5 | ✅ Complete |
| imagepolicy | 4 | ✅ Complete |
| abac | 3 | ✅ Complete |

**Total: 1437 API types across 28 modules**

### Version Conversion (k8s-api-conversion)

| Module | Conversions | Types |
|--------|-------------|-------|
| abac | 1 | Policy (v0, v1beta1) |
| admission | 1 | AdmissionReview |
| admissionregistration | 12 | MutatingWebhookConfiguration, MutatingWebhookConfigurationList, ValidatingWebhookConfiguration, ValidatingWebhookConfigurationList, ValidatingAdmissionPolicy (v1alpha1, v1beta1), ValidatingAdmissionPolicyList, ValidatingAdmissionPolicyBinding (v1alpha1, v1beta1), ValidatingAdmissionPolicyBindingList, MutatingAdmissionPolicy (v1alpha1, v1beta1), MutatingAdmissionPolicyList, MutatingAdmissionPolicyBinding (v1alpha1, v1beta1), MutatingAdmissionPolicyBindingList |
| apidiscovery | 2 | APIGroupDiscovery, APIGroupDiscoveryList |
| authentication | 2 | TokenReview, SelfSubjectReview |
| authorization | 4 | SubjectAccessReview, SelfSubjectAccessReview, LocalSubjectAccessReview, SelfSubjectRulesReview |
| storage | 12 | StorageClass, StorageClassList, VolumeAttachment, VolumeAttachmentList, CSIDriver, CSIDriverList, CSINode, CSINodeList, CSIStorageCapacity, CSIStorageCapacityList, VolumeAttributesClass, VolumeAttributesClassList |
| resource | 8 | ResourceClaim, ResourceClaimList, ResourceClaimTemplate, ResourceClaimTemplateList, DeviceClass, DeviceClassList, ResourceSlice, ResourceSliceList (v1beta1, v1beta2) |
| rbac | 8 | Role, RoleList, RoleBinding, RoleBindingList, ClusterRole, ClusterRoleList, ClusterRoleBinding, ClusterRoleBindingList (v1alpha1, v1beta1) |
| apps | 11 | Deployment, DeploymentList, StatefulSet, StatefulSetList, DaemonSet, DaemonSetList, ReplicaSet, ReplicaSetList, ControllerRevision, ControllerRevisionList, Scale (v1beta1, v1beta2) |
| batch | 4 | Job, JobList, CronJob, CronJobList |
| networking | 8 | Ingress, IngressList, IngressClass, IngressClassList, IPAddress, IPAddressList, ServiceCIDR, ServiceCIDRList |
| policy | 3 | PodDisruptionBudget, PodDisruptionBudgetList, Eviction |
| autoscaling | 2 | HorizontalPodAutoscaler, HorizontalPodAutoscalerList (v1, v2beta1, v2beta2) |
| discovery | 2 | EndpointSlice, EndpointSliceList |
| events | 2 | Event, EventList |
| certificates | 4 | CertificateSigningRequest, CertificateSigningRequestList, ClusterTrustBundle, ClusterTrustBundleList (v1alpha1, v1beta1) |
| coordination | 4 | Lease, LeaseList, LeaseCandidate, LeaseCandidateList |
| flowcontrol | 4 | FlowSchema, FlowSchemaList, PriorityLevelConfiguration, PriorityLevelConfigurationList |
| node | 2 | RuntimeClass, RuntimeClassList (v1alpha1, v1beta1) |
| scheduling | 2 | PriorityClass, PriorityClassList (v1alpha1, v1beta1) |

**Total: 98 version conversions implemented**

### Validation (k8s-api-validation)

| Module | Validators | Coverage |
|--------|------------|----------|
| common | 19 | DNS names, labels, selectors, quantities, resources |
| admissionregistration | 8 | Webhook configurations, policies |
| authentication | 5 | TokenReview, TokenRequest, SelfSubjectReview |
| authorization | 8 | SAR/SSAR/LSAR, SelfSubjectRulesReview |
| certificates | 11 | CertificateSigningRequest |
| storage | 9 | StorageClass, VolumeAttachment, CSI types |
| coordination | 10 | Lease |
| node | 12 | RuntimeClass, Toleration |
| apps | 8 | Deployment, StatefulSet, DaemonSet, ReplicaSet |
| core | 8 | Pod, Service, ConfigMap, Namespace |
| networking | 8 | NetworkPolicy, Ingress, IngressClass |
| rbac | 7 | Role, ClusterRole, RoleBinding, Subject |
| batch | 4 | Job, CronJob |
| autoscaling | 3 | HPA metrics, behaviors |
| policy | 3 | PodDisruptionBudget |
| discovery | 1 | EndpointSlice |
| flowcontrol | 4 | FlowSchema, PriorityLevelConfiguration |
| resource | 12 | ResourceClaim, ResourceClass, ResourceSlice |
| abac | 2 | Policy |
| admission | 2 | AdmissionReview |
| apidiscovery | 4 | APIGroupDiscovery, APIGroupDiscoveryList |
| apiextensions | 1 | CustomResourceDefinition |
| apiregistration | 1 | APIService |
| apiserverinternal | 1 | StorageVersion |
| events | 2 | Event |
| extensions | 1 | Ingress |
| imagepolicy | 1 | ImageReview |
| storagemigration | 1 | StorageVersionMigration |

**Total: 156 validation functions across 28 modules**

### Serialization Tests

- ✅ core/v1: Pod, Service, ConfigMap
- ✅ apps/v1: Deployment, StatefulSet, DaemonSet, ReplicaSet
- ✅ batch/v1: Job, CronJob
- ✅ rbac/v1: Role, ClusterRole, RoleBinding, ClusterRoleBinding
- ✅ scheduling/v1: PriorityClass
- ✅ coordination/v1: Lease
- ✅ admission/v1: AdmissionReview
- ✅ events/v1: Event
- ✅ apiregistration/v1: APIService
- ✅ apiextensions/v1: CustomResourceDefinition
- ✅ apidiscovery/v2: APIGroupDiscovery
- ✅ apiserverinternal/v1alpha1: StorageVersion
- ✅ imagepolicy/v1alpha1: ImageReview
- ✅ storagemigration/v1alpha1: StorageVersionMigration
- ✅ extensions/v1beta1: Ingress

### Roadmap

**High Priority:**
- [x] `resource` module validation (102 types)
- [x] `admissionregistration` module validation (53 types)
- [x] `discovery` version conversion ✅
- [x] `events` version conversion ✅

**Medium Priority:**
- [x] `flowcontrol` module validation (44 types)
- [x] `certificates` module validation ✅
- [x] `authentication` module validation
- [x] `authorization` module validation

**Low Priority:**
- [x] `coordination` module validation ✅
- [x] `node` module validation ✅
- [x] `scheduling` module validation ✅
- [x] Additional serialization tests ✅
- [x] Documentation examples ✅

### Next Steps (Planned Order)

1. [x] Expand version conversion coverage for remaining multi-version API groups/types
   - Added list conversions and remaining multi-version resources (apps lists/Scale, networking IPAddress/ServiceCIDR, storage lists, etc.)
2. [x] Add validation coverage for remaining API groups
   - Added `admission`, `events`, `extensions`, `imagepolicy`, `apiserverinternal`, `apidiscovery`,
     `abac`, `apiregistration`, `apiextensions`, `storagemigration`
3. [x] Add serialization round-trip tests for untested modules
   - Added tests for admission, events, apiregistration, apiextensions, apidiscovery, imagepolicy,
     apiserverinternal, storagemigration, and extensions

### Follow-up Work

- [x] Add missing `apiextensions/v1` types (ConversionReview/Request/Response, ConditionStatus, JSONSchema* aliases, ResourceScope, ValidationRules)
- [x] Add `apiregistration/v1` ConditionStatus and APIServiceConditionType enums
- [x] Implement `apiextensions/v1beta1` module to match Kubernetes v1.34.1
- [x] Implement `apiregistration/v1beta1` module to match Kubernetes v1.34.1
- [x] Reconcile Rust-only types vs Go (policy/v1beta1 PodSecurityPolicy*, batch/v1beta1 Job*, resource/v1alpha3 claim types) and decide keep/remove
- [x] Audit duplicated meta types (GroupVersionKind, GroupVersionResource, ObjectReference, UserInfo, EventSource, TypedLocalObjectReference, Toleration) vs apimachinery/core sources

## Source Reference

This project refactors types from the official Kubernetes codebase:
- Source: `kubernetes/staging/src/k8s.io/api/`
- Version: Kubernetes v1.34.1 only (no other versions targeted)
- Total API groups: 28
- Total code volume: ~18,000+ lines of Go code

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running with specific features

```bash
cargo build --no-default-features --features "core,apps"
```

## Contributing

Contributions are welcome! Areas where help is needed:

1. **Type implementations**: Help complete remaining API types
2. **Validation logic**: Implement validation rules for API types
3. **Tests**: Add unit tests and integration tests
4. **Documentation**: Improve code documentation and examples
5. **Conversion logic**: Implement version conversion between API versions

Please see the Roadmap section for specific tasks.

## License

Apache-2.0

This project follows the same license as Kubernetes.

## Acknowledgments

This project is based on the official Kubernetes API types:
- [kubernetes/api](https://github.com/kubernetes/api)
- [kubernetes/apimachinery](https://github.com/kubernetes/apimachinery)

Special thanks to the Kubernetes community for their excellent work on the API design.
