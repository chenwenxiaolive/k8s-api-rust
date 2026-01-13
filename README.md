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

### Completed

- ✅ Core infrastructure (k8s-api-core, k8s-apimachinery)
- ✅ Core API v1 - Partial (136 types, ~60% complete)
  - Pod, Container, Service, ConfigMap, Secret
  - Volume, PersistentVolume, PersistentVolumeClaim
  - Namespace, Node, ServiceAccount
- ✅ Apps API v1 - Partial (~70% complete)
  - Deployment, StatefulSet, DaemonSet, ReplicaSet
- ✅ Batch API v1 - Partial (~70% complete)
  - Job, CronJob
- ✅ Validation framework
- ✅ Conversion framework (skeleton)

### In Progress

- 🚧 Completing core/v1 types
- 🚧 Completing apps/v1 types
- 🚧 Completing batch/v1 types

### Planned

See [REFACTORING_PLAN.md](REFACTORING_PLAN.md) for detailed roadmap.

Priority order:
1. Complete core/v1, apps/v1, batch/v1
2. Add networking/v1, rbac/v1, storage/v1
3. Add autoscaling, policy, scheduling
4. Add remaining API groups
5. Complete validation and conversion logic
6. Add comprehensive tests

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
