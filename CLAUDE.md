# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **k8s-api-rust** - a Rust rewrite of Kubernetes API types, targeting full compatibility with Kubernetes 1.34 and the kube-rs ecosystem. The project rewrites 26 API groups (439 Go files, ~420,000 lines) from the official Kubernetes Go implementation.

## Build Commands

```bash
cargo build                    # Build all crates
cargo test                     # Run all tests
cargo fmt --check              # Check formatting
cargo clippy --all-targets     # Run linter
cargo test -p k8s-api-core     # Test single crate
```

## Architecture

### Workspace Structure

```
crates/
├── k8s-api-core/     # Core traits (DeepCopy, Resource) and primitives (IntOrString)
└── k8s-api-meta/     # apimachinery types (TypeMeta, ObjectMeta, LabelSelector)
```

### Core Traits (`k8s-api-core`)

**Resource trait** - Provides API metadata for all K8s types:
```rust
pub trait Resource {
    const API_VERSION: &'static str;  // e.g., "apps/v1"
    const GROUP: &'static str;        // e.g., "apps"
    const KIND: &'static str;         // e.g., "Deployment"
    const VERSION: &'static str;      // e.g., "v1"
    const PLURAL: &'static str;       // e.g., "deployments"
}
```

**Marker traits** - `NamespacedResource` and `ClusterResource` distinguish scope.

**DeepCopy trait** - Explicit deep copying semantics, blanket-implemented for Clone types.

### Type Mapping (Go → Rust)

| Go Type | Rust Type |
|---------|-----------|
| `string` | `String` |
| `*T` | `Option<T>` |
| `[]T` | `Vec<T>` |
| `map[K]V` | `BTreeMap<K, V>` |
| `time.Time` | `chrono::DateTime<Utc>` |
| `intstr.IntOrString` | `IntOrString` enum |

### Serialization Pattern

All types must use these serde attributes for K8s API compatibility:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SomeType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_field: Vec<String>,

    #[serde(flatten)]
    pub type_meta: TypeMeta,  // Embedded fields
}
```

## Code Quality Requirements

- `#![deny(missing_docs)]` - All public items must have doc comments
- `#![deny(unsafe_code)]` - No unsafe code
- All types need serde roundtrip tests (serialize → deserialize → compare)
- Clippy must pass with no warnings

## Implementation Roadmap

See `PLAN.md` for detailed progress. Next priorities:
1. Add `Quantity`, `Time`, `MicroTime`, `Duration` types to k8s-api-core
2. Create k8s-api-core-v1 crate with Pod, Container, Service, ConfigMap, Secret
3. Consider code generation for the 308+ types in core/v1

## Reference Source

The `../api` directory contains the original Go Kubernetes API definitions being ported.
