# k8s-api-rust

Kubernetes API types rewritten in Rust, compatible with Kubernetes 1.34.

## Overview

This project provides Rust implementations of Kubernetes API types, fully compatible with:
- JSON serialization (serde)
- Protocol Buffers serialization (prost)
- kube-rs ecosystem

## Structure

```
crates/
├── k8s-api-core/     # Core traits and primitive types
├── k8s-api-meta/     # apimachinery types (TypeMeta, ObjectMeta)
└── ...               # API group crates (coming soon)
```

## License

Apache-2.0
