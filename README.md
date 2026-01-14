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
│   └── k8s-api-conversion/    # Version conversion
└── README.md
```

## Current Status

**Total Tests: 187 (all passing)**

### API Types Coverage (k8s-api)

| Module | Types | Status |
|--------|-------|--------|
| core | 185 | ✅ Complete |
| resource | 102 | ✅ Types defined |
| storage | 54 | ✅ Complete |
| admissionregistration | 53 | ✅ Types defined |
| networking | 50 | ✅ Complete |
| apps | 50 | ✅ Complete |
| flowcontrol | 44 | ✅ Types defined |
| autoscaling | 31 | ✅ Complete |
| policy | 26 | ✅ Complete |
| authorization | 26 | ✅ Types defined |
| batch | 25 | ✅ Complete |
| rbac | 24 | ✅ Complete |
| authentication | 17 | ✅ Types defined |
| apiextensions | 17 | ✅ Types defined |
| discovery | 13 | ✅ Complete |
| certificates | 12 | ✅ Types defined |
| node | 9 | ✅ Types defined |
| events | 9 | ✅ Types defined |
| coordination | 6 | ✅ Types defined |
| scheduling | 2 | ✅ Types defined |

**Total: 783 API types across 26 modules**

### Version Conversion (k8s-api-conversion)

| Module | Conversions | Types |
|--------|-------------|-------|
| storage | 5 | StorageClass, VolumeAttachment, CSIDriver, CSINode, CSIStorageCapacity |
| rbac | 4 | Role, ClusterRole, RoleBinding, ClusterRoleBinding |
| apps | 2 | Deployment, StatefulSet |
| batch | 2 | Job, CronJob |
| networking | 2 | NetworkPolicy, Ingress |
| policy | 2 | PodDisruptionBudget, Eviction |
| autoscaling | 1 | HorizontalPodAutoscaler |

**Total: 18 version conversions implemented**

### Validation (k8s-api-validation)

| Module | Validators | Coverage |
|--------|------------|----------|
| common | 19 | DNS names, labels, selectors, quantities, resources |
| storage | 9 | StorageClass, VolumeAttachment, CSI types |
| apps | 8 | Deployment, StatefulSet, DaemonSet, ReplicaSet |
| core | 8 | Pod, Service, ConfigMap, Namespace |
| networking | 8 | NetworkPolicy, Ingress, IngressClass |
| rbac | 7 | Role, ClusterRole, RoleBinding, Subject |
| batch | 4 | Job, CronJob |
| autoscaling | 3 | HPA metrics, behaviors |
| policy | 3 | PodDisruptionBudget |
| discovery | 1 | EndpointSlice |

**Total: 70 validation functions across 10 modules**

### Serialization Tests

- ✅ core/v1: Pod, Service, ConfigMap
- ✅ apps/v1: Deployment, StatefulSet, DaemonSet, ReplicaSet
- ✅ batch/v1: Job, CronJob
- ✅ rbac/v1: Role, ClusterRole, RoleBinding, ClusterRoleBinding

### Roadmap

**High Priority:**
- [ ] `resource` module validation (102 types)
- [ ] `admissionregistration` module validation (53 types)
- [ ] `discovery` version conversion

**Medium Priority:**
- [ ] `flowcontrol` module validation (44 types)
- [ ] `certificates` module validation
- [ ] `authentication/authorization` validation

**Low Priority:**
- [ ] `coordination`, `scheduling`, `events` modules
- [ ] Additional serialization tests
- [ ] Documentation examples

## Source Reference

This project refactors types from the official Kubernetes codebase:
- Source: `kubernetes/staging/src/k8s.io/api/`
- Version: Based on latest Kubernetes release
- Total API groups: 27
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

Please see [REFACTORING_PLAN.md](REFACTORING_PLAN.md) for specific tasks.

## License

Apache-2.0

This project follows the same license as Kubernetes.

## Acknowledgments

This project is based on the official Kubernetes API types:
- [kubernetes/api](https://github.com/kubernetes/api)
- [kubernetes/apimachinery](https://github.com/kubernetes/apimachinery)

Special thanks to the Kubernetes community for their excellent work on the API design.
