# Kubernetes API Rust Refactoring Plan

This document outlines the detailed plan for refactoring Kubernetes API types from Go to Rust.

## Table of Contents

- [Source Code Analysis](#source-code-analysis)
- [Current Progress](#current-progress)
- [Refactoring Strategy](#refactoring-strategy)
- [Detailed Roadmap](#detailed-roadmap)
- [Work Estimates](#work-estimates)
- [Technical Guidelines](#technical-guidelines)
- [Next Actions](#next-actions)

---

## Source Code Analysis

### Source Location

The Kubernetes Go API types are located in:
- **Primary**: `kubernetes/staging/src/k8s.io/api/`
- **Internal**: `kubernetes/pkg/apis/`

### API Groups Overview

Total: **27 API groups** identified in `staging/src/k8s.io/api/`:

| API Group | Versions | Priority | Status |
|-----------|----------|----------|--------|
| **core** | v1 | P1 | 🟡 60% |
| **apps** | v1, v1beta1, v1beta2 | P1 | 🟡 70% |
| **batch** | v1, v1beta1 | P1 | 🟡 70% |
| **networking** | v1, v1beta1 | P2 | ⚪ 0% |
| **rbac** | v1, v1alpha1, v1beta1 | P2 | ⚪ 0% |
| **storage** | v1, v1alpha1, v1beta1 | P2 | ⚪ 0% |
| **autoscaling** | v1, v2, v2beta1, v2beta2 | P2 | ⚪ 0% |
| **policy** | v1, v1beta1 | P2 | ⚪ 0% |
| **scheduling** | v1, v1alpha1, v1beta1 | P3 | ⚪ 0% |
| **certificates** | v1, v1alpha1, v1beta1 | P3 | ⚪ 0% |
| **coordination** | v1, v1alpha2, v1beta1 | P3 | ⚪ 0% |
| **discovery** | v1, v1beta1 | P3 | ⚪ 0% |
| **events** | v1, v1beta1 | P3 | ⚪ 0% |
| **node** | v1, v1alpha1, v1beta1 | P3 | ⚪ 0% |
| **authentication** | v1, v1alpha1, v1beta1 | P3 | ⚪ 0% |
| **authorization** | v1, v1beta1 | P3 | ⚪ 0% |
| **admissionregistration** | v1, v1alpha1, v1beta1 | P4 | ⚪ 0% |
| **flowcontrol** | v1, v1beta1, v1beta2, v1beta3 | P4 | ⚪ 0% |
| **apidiscovery** | v2, v2beta1 | P4 | ⚪ 0% |
| **resource** | v1, v1alpha3, v1beta1, v1beta2 | P4 | ⚪ 0% |
| **apiserverinternal** | v1alpha1 | P4 | ⚪ 0% |
| **storagemigration** | v1alpha1 | P4 | ⚪ 0% |
| **imagepolicy** | v1alpha1 | P4 | ⚪ 0% |
| **admission** | v1, v1beta1 | P4 | ⚪ 0% |
| **extensions** | v1beta1 (deprecated) | P4 | ⚪ 0% |

### Code Volume

- **Go source**: ~18,000+ lines (v1 types.go files only)
- **Estimated Rust target**: ~15,000-20,000 lines
- **Go types in core/v1**: ~396 types
- **Current Rust types in core/v1**: 136 types (~34%)

---

## Current Progress

### Completed Crates

#### 1. k8s-api-core (416 lines)

**Purpose**: Fundamental types used across all API definitions

**Implemented**:
- ✅ Schema types
  - `GroupKind`, `GroupVersion`, `GroupResource`
  - `GroupVersionKind`, `GroupVersionResource`
- ✅ Resource types
  - `Quantity` - Kubernetes resource quantities (CPU, memory, etc.)
  - `IntOrString` - Union type for integer or string values
- ✅ Runtime traits
  - `Object` trait for Kubernetes objects
  - `DeepCopy` trait

**Location**: `crates/k8s-api-core/`

#### 2. k8s-apimachinery (450 lines)

**Purpose**: API machinery and metadata types

**Implemented**:
- ✅ `TypeMeta` - API version and kind
- ✅ `ObjectMeta` - Standard object metadata (name, namespace, labels, annotations, etc.)
- ✅ `ListMeta` - Metadata for list responses
- ✅ `Time`, `MicroTime` - Timestamp types with proper serialization
- ✅ `Status` - API response status
- ✅ `Condition` - Generic condition type
- ✅ `LabelSelector` - Label selector with match expressions
- ✅ `OwnerReference` - Cross-object ownership
- ✅ `ManagedFieldsEntry` - Field management metadata

**Location**: `crates/k8s-apimachinery/`

#### 3. k8s-api (3,338 lines)

**Purpose**: Kubernetes API types for all API groups

**Implemented**:

##### core/v1 (2,421 lines, 136 types, ~34% complete)
- ✅ Pod and PodSpec ecosystem
  - Pod, PodSpec, PodStatus, PodCondition
  - Container, ContainerSpec, ContainerStatus
  - InitContainer, EphemeralContainer
- ✅ Workload support types
  - EnvVar, EnvVarSource, EnvFromSource
  - ResourceRequirements, ResourceList
  - Probe (Liveness, Readiness, Startup)
  - Lifecycle hooks
- ✅ Volume types (partial)
  - Volume, VolumeSource (basic structure)
  - EmptyDir, HostPath, ConfigMap, Secret volumes
  - PersistentVolumeClaim references
- ✅ Storage
  - PersistentVolume, PersistentVolumeClaim
  - PersistentVolumeSpec, PersistentVolumeStatus
- ✅ Configuration
  - ConfigMap, ConfigMapVolumeSource
  - Secret, SecretVolumeSource
- ✅ Service
  - Service, ServiceSpec, ServiceStatus
  - ServicePort, ServiceAccount
- ✅ Basic resources
  - Namespace, Node
  - Event (basic)

**Missing in core/v1** (~260 types):
- ⚪ Complete volume source types (25+ types)
  - CSI, NFS, iSCSI, FC, RBD, CephFS, etc.
  - Projected volumes, DownwardAPI
- ⚪ Advanced pod features
  - SecurityContext, PodSecurityContext
  - Affinity, Tolerations, TopologySpreadConstraints
- ⚪ Networking details
  - NetworkPolicy
  - Endpoints, EndpointSlice
- ⚪ Resource quota and limits
  - ResourceQuota, LimitRange
- ⚪ Additional core resources
  - ReplicationController
  - ComponentStatus
  - Binding

##### apps/v1 (597 lines, ~70% complete)
- ✅ Deployment
  - Deployment, DeploymentSpec, DeploymentStatus
  - DeploymentStrategy, RollingUpdateDeployment
- ✅ StatefulSet
  - StatefulSet, StatefulSetSpec, StatefulSetStatus
  - StatefulSetUpdateStrategy
- ✅ DaemonSet
  - DaemonSet, DaemonSetSpec, DaemonSetStatus
  - DaemonSetUpdateStrategy
- ✅ ReplicaSet
  - ReplicaSet, ReplicaSetSpec, ReplicaSetStatus

**Missing in apps/v1**:
- ⚪ ControllerRevision
- ⚪ Some advanced strategy options

##### batch/v1 (320 lines, ~70% complete)
- ✅ Job
  - Job, JobSpec, JobStatus
  - JobCondition
- ✅ CronJob
  - CronJob, CronJobSpec, CronJobStatus
  - CronJobSchedule

**Missing in batch/v1**:
- ⚪ Advanced job templates
- ⚪ Some policy fields

**Location**: `crates/k8s-api/`

#### 4. k8s-api-validation (212 lines)

**Purpose**: Validation logic for API types

**Implemented**:
- ✅ Validation framework
- ✅ Core v1 validators (basic)
  - DNS label validation
  - Resource name validation
  - Namespace validation

**Missing**:
- ⚪ Complete validation rules for all types
- ⚪ Field-level validators
- ⚪ Business logic validators

**Location**: `crates/k8s-api-validation/`

#### 5. k8s-api-conversion (39 lines)

**Purpose**: Version conversion between API versions

**Implemented**:
- ✅ Conversion framework (skeleton)
- ✅ Conversion scheme structure

**Missing**:
- ⚪ Actual conversion implementations
- ⚪ v1beta1 ↔ v1 conversions
- ⚪ v2beta1 ↔ v2 conversions

**Location**: `crates/k8s-api-conversion/`

### Summary Statistics

- **Total crates**: 5
- **Total Rust files**: 24
- **Total lines of code**: ~4,400 lines
- **Build status**: ✅ Compiles successfully
- **Test coverage**: Minimal

---

## Refactoring Strategy

### Principles

1. **Type safety first**: Leverage Rust's type system to prevent invalid states
2. **Idiomatic Rust**: Use Rust conventions (Option, Result, traits, etc.)
3. **Serde integration**: Full serialization/deserialization support
4. **Modular design**: Separate crates for clear dependency management
5. **Feature flags**: Allow granular selection of API groups
6. **Progressive enhancement**: Start with stable APIs (v1), then add alpha/beta

### Mapping Guidelines

#### Go → Rust Type Mappings

| Go Type | Rust Type | Notes |
|---------|-----------|-------|
| `string` | `String` | |
| `*string` | `Option<String>` | Optional fields |
| `[]string` | `Vec<String>` | |
| `map[string]string` | `BTreeMap<String, String>` | Ordered for consistency |
| `int32` | `i32` | |
| `int64` | `i64` | |
| `bool` | `bool` | |
| `*bool` | `Option<bool>` | |
| `intstr.IntOrString` | `IntOrString` | Custom enum |
| `resource.Quantity` | `Quantity` | Custom type |
| `metav1.Time` | `Time` | Custom wrapper around chrono |

#### Struct Tags → Serde Attributes

```go
// Go
type Pod struct {
    Name string `json:"name,omitempty"`
}
```

```rust
// Rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
```

#### Embedded Types

```go
// Go
type Pod struct {
    metav1.TypeMeta   `json:",inline"`
    metav1.ObjectMeta `json:"metadata,omitempty"`
}
```

```rust
// Rust
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,
}
```

---

## Detailed Roadmap

### Phase 1: Complete Core API Groups (Priority 1)

#### Task 1.1: Complete core/v1 📍 CURRENT

**Estimated**: +2,000 lines, ~260 additional types

**Sub-tasks**:
1. Complete volume source types
   - [ ] CSIVolumeSource and related types
   - [ ] NFSVolumeSource
   - [ ] iSCSIVolumeSource
   - [ ] FCVolumeSource
   - [ ] RBDVolumeSource
   - [ ] CephFSVolumeSource
   - [ ] GlusterfsVolumeSource
   - [ ] AWSElasticBlockStoreVolumeSource
   - [ ] GCEPersistentDiskVolumeSource
   - [ ] AzureDiskVolumeSource
   - [ ] AzureFileVolumeSource
   - [ ] ProjectedVolumeSource
   - [ ] DownwardAPIVolumeSource
   - [ ] And ~15 more volume types

2. Security context
   - [ ] SecurityContext
   - [ ] PodSecurityContext
   - [ ] SELinuxOptions
   - [ ] Capabilities
   - [ ] WindowsSecurityContextOptions

3. Pod scheduling
   - [ ] Affinity (NodeAffinity, PodAffinity, PodAntiAffinity)
   - [ ] Toleration
   - [ ] TopologySpreadConstraint
   - [ ] PodSchedulingGate

4. Networking
   - [ ] NetworkPolicy
   - [ ] Endpoints
   - [ ] EndpointSubset
   - [ ] EndpointAddress, EndpointPort

5. Resource management
   - [ ] ResourceQuota, ResourceQuotaSpec, ResourceQuotaStatus
   - [ ] LimitRange, LimitRangeSpec, LimitRangeItem

6. Additional core resources
   - [ ] ReplicationController
   - [ ] ComponentStatus
   - [ ] Binding
   - [ ] Event (complete)

**Priority**: HIGH - Many dependent API groups need complete core/v1

#### Task 1.2: Complete apps/v1

**Estimated**: +500 lines

**Sub-tasks**:
- [ ] ControllerRevision
- [ ] Complete all strategy and policy options
- [ ] Add missing status conditions

**Priority**: HIGH

#### Task 1.3: Complete batch/v1

**Estimated**: +200 lines

**Sub-tasks**:
- [ ] Add pod failure policy
- [ ] Complete job templates
- [ ] Add success policy

**Priority**: HIGH

**Phase 1 Total**: ~2,700 lines

### Phase 2: Common API Groups (Priority 2)

#### Task 2.1: networking/v1

**Estimated**: ~800 lines

**Types**:
- [ ] Ingress, IngressSpec, IngressStatus
- [ ] IngressClass, IngressClassSpec
- [ ] IngressRule, HTTPIngressRuleValue
- [ ] IngressBackend, IngressServiceBackend
- [ ] NetworkPolicy (move from core/v1)
- [ ] NetworkPolicySpec, NetworkPolicyIngressRule, NetworkPolicyEgressRule

**Priority**: HIGH - Very commonly used

#### Task 2.2: rbac/v1

**Estimated**: ~600 lines

**Types**:
- [ ] Role, RoleSpec
- [ ] RoleBinding, RoleBindingSpec
- [ ] ClusterRole, ClusterRoleSpec
- [ ] ClusterRoleBinding, ClusterRoleBindingSpec
- [ ] PolicyRule
- [ ] Subject, RoleRef

**Priority**: HIGH - Essential for authorization

#### Task 2.3: storage/v1

**Estimated**: ~700 lines

**Types**:
- [ ] StorageClass, StorageClassSpec
- [ ] VolumeAttachment, VolumeAttachmentSpec, VolumeAttachmentStatus
- [ ] VolumeAttachmentSource
- [ ] CSIDriver, CSIDriverSpec
- [ ] CSINode, CSINodeSpec
- [ ] VolumeError

**Priority**: HIGH - Essential for storage

#### Task 2.4: autoscaling/v1, v2

**Estimated**: ~600 lines

**Types**:
- [ ] HorizontalPodAutoscaler (v1)
- [ ] HorizontalPodAutoscaler (v2)
- [ ] HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus
- [ ] MetricSpec, MetricStatus
- [ ] ResourceMetricSource, PodsMetricSource, ObjectMetricSource
- [ ] CrossVersionObjectReference

**Priority**: MEDIUM - Common for production workloads

#### Task 2.5: policy/v1

**Estimated**: ~400 lines

**Types**:
- [ ] PodDisruptionBudget, PodDisruptionBudgetSpec, PodDisruptionBudgetStatus
- [ ] PodSecurityPolicy (deprecated but still in use)
- [ ] Eviction

**Priority**: MEDIUM

**Phase 2 Total**: ~3,100 lines

### Phase 3: Extended API Groups (Priority 3)

#### Task 3.1: scheduling/v1

**Estimated**: ~300 lines

**Types**:
- [ ] PriorityClass, PriorityClassSpec

#### Task 3.2: certificates/v1

**Estimated**: ~400 lines

**Types**:
- [ ] CertificateSigningRequest
- [ ] CertificateSigningRequestSpec, CertificateSigningRequestStatus

#### Task 3.3: coordination/v1

**Estimated**: ~300 lines

**Types**:
- [ ] Lease, LeaseSpec

#### Task 3.4: discovery/v1

**Estimated**: ~400 lines

**Types**:
- [ ] EndpointSlice, EndpointSliceSpec
- [ ] Endpoint, EndpointConditions, EndpointHints

#### Task 3.5: events/v1

**Estimated**: ~300 lines

**Types**:
- [ ] Event, EventSpec

#### Task 3.6: node/v1

**Estimated**: ~500 lines

**Types**:
- [ ] RuntimeClass, RuntimeClassSpec

#### Task 3.7: authentication/v1

**Estimated**: ~400 lines

**Types**:
- [ ] TokenRequest, TokenRequestSpec, TokenRequestStatus
- [ ] TokenReview, TokenReviewSpec, TokenReviewStatus

#### Task 3.8: authorization/v1

**Estimated**: ~500 lines

**Types**:
- [ ] SubjectAccessReview, SubjectAccessReviewSpec, SubjectAccessReviewStatus
- [ ] SelfSubjectAccessReview
- [ ] LocalSubjectAccessReview
- [ ] SelfSubjectRulesReview

**Phase 3 Total**: ~3,100 lines

### Phase 4: Advanced API Groups (Priority 4)

#### Task 4.1-4.10: Remaining API groups

**Estimated**: ~3,000 lines total

- [ ] admissionregistration/v1 (~500 lines)
- [ ] flowcontrol/v1 (~600 lines)
- [ ] apidiscovery/v2 (~400 lines)
- [ ] resource/v1 (~500 lines)
- [ ] apiserverinternal/v1alpha1 (~300 lines)
- [ ] storagemigration/v1alpha1 (~200 lines)
- [ ] imagepolicy/v1alpha1 (~200 lines)
- [ ] admission/v1 (~200 lines)
- [ ] extensions/v1beta1 (~100 lines, deprecated)

**Phase 4 Total**: ~3,000 lines

### Phase 5: Cross-cutting Concerns

#### Task 5.1: Validation

**Estimated**: +2,000 lines

**Sub-tasks**:
- [ ] Complete validation for core/v1
- [ ] Add validation for all API groups
- [ ] Field validators (regex, range, enum, etc.)
- [ ] Business logic validators
- [ ] Custom validators
- [ ] Validation error messages

#### Task 5.2: Conversion

**Estimated**: +1,500 lines

**Sub-tasks**:
- [ ] apps: v1beta1 ↔ v1beta2 ↔ v1
- [ ] autoscaling: v1 ↔ v2beta1 ↔ v2beta2 ↔ v2
- [ ] batch: v1beta1 ↔ v1
- [ ] networking: v1beta1 ↔ v1
- [ ] rbac: v1alpha1 ↔ v1beta1 ↔ v1
- [ ] And conversions for other versioned APIs

#### Task 5.3: Testing

**Estimated**: +3,000 lines

**Sub-tasks**:
- [ ] Unit tests for all types
- [ ] Serialization/deserialization tests
- [ ] Validation tests
- [ ] Conversion tests
- [ ] Round-trip tests (JSON ↔ Rust ↔ JSON)
- [ ] Integration tests

#### Task 5.4: Documentation

**Sub-tasks**:
- [ ] API documentation for all types
- [ ] Usage examples
- [ ] Migration guide from other Rust k8s libraries
- [ ] Comparison with Go types

**Phase 5 Total**: ~6,500 lines

---

## Work Estimates

### Summary by Phase

| Phase | Description | Lines | Types | Effort |
|-------|-------------|-------|-------|--------|
| **Current** | Completed work | 4,400 | ~200 | ✅ Done |
| **Phase 1** | Complete P1 API groups | 2,700 | ~300 | 2-3 weeks |
| **Phase 2** | Common API groups | 3,100 | ~100 | 2-3 weeks |
| **Phase 3** | Extended API groups | 3,100 | ~80 | 2-3 weeks |
| **Phase 4** | Advanced API groups | 3,000 | ~60 | 2 weeks |
| **Phase 5** | Validation, conversion, tests | 6,500 | N/A | 3-4 weeks |
| **Total** | Complete project | 22,800 | ~740 | 11-16 weeks |

### Effort Breakdown

- **Type implementation**: 40% (mapping Go types to Rust)
- **Validation logic**: 20% (implementing validation rules)
- **Conversion logic**: 15% (version conversion)
- **Testing**: 20% (unit, integration, round-trip tests)
- **Documentation**: 5% (API docs, examples)

### Parallelization Opportunities

Multiple API groups can be implemented in parallel:
- Phase 2 tasks (2.1-2.5) are independent
- Phase 3 tasks (3.1-3.8) are independent
- Validation and conversion can be done alongside type implementation

---

## Technical Guidelines

### Code Style

1. **Naming conventions**
   - Use Rust naming conventions (snake_case for fields, PascalCase for types)
   - Match Kubernetes naming in JSON via serde rename

2. **Default values**
   - Use `#[serde(default)]` for optional fields
   - Implement `Default` trait where appropriate

3. **Error handling**
   - Use `Result<T, E>` for operations that can fail
   - Define custom error types with `thiserror`

4. **Documentation**
   - Add doc comments for all public types and fields
   - Include examples in doc comments
   - Reference Kubernetes documentation URLs

### Testing Strategy

1. **Unit tests**
   - Test each type's serialization/deserialization
   - Test default values
   - Test validation logic

2. **Integration tests**
   - Test interaction between types
   - Test schema validation
   - Test conversion between versions

3. **Compatibility tests**
   - Round-trip: JSON → Rust → JSON
   - Compare with Go implementation output
   - Test against real Kubernetes API

### Performance Considerations

1. **Minimize allocations**
   - Use `Cow<str>` where appropriate
   - Use `&str` in function parameters
   - Consider `Box` for large nested structures

2. **Optimize serialization**
   - Use efficient serde configurations
   - Consider binary formats (protobuf) as feature flag

3. **Lazy evaluation**
   - Use feature flags to avoid compiling unused API groups
   - Consider lazy_static for expensive computations

---

## Next Actions

### Immediate (This Week)

1. ✅ Create README.md
2. ✅ Create REFACTORING_PLAN.md
3. 📍 Start Task 1.1: Complete core/v1 volume sources
4. 📍 Add validation tests for existing types

### Short-term (Next 2 Weeks)

1. Complete Phase 1 (Priority 1 API groups)
2. Set up CI/CD pipeline
3. Add more comprehensive tests
4. Start Phase 2 (networking/v1)

### Medium-term (Next Month)

1. Complete Phase 2 (Common API groups)
2. Begin Phase 3 (Extended API groups)
3. Implement basic validation logic
4. Write usage examples

### Long-term (Next 2-3 Months)

1. Complete all stable v1 APIs
2. Add comprehensive validation
3. Implement version conversion
4. Achieve 80%+ test coverage
5. Publish first stable release (1.0)

---

## Tracking Progress

### Metrics

- **Type coverage**: Types implemented / Total types
- **Line coverage**: Lines of code / Estimated total
- **API group coverage**: Groups with stable v1 / Total groups
- **Test coverage**: Test lines / Implementation lines
- **Build health**: CI passing rate

### Milestones

- [ ] **M1**: Complete core/v1 (100%)
- [ ] **M2**: Complete Phase 1 (core, apps, batch)
- [ ] **M3**: Complete Phase 2 (networking, rbac, storage, autoscaling, policy)
- [ ] **M4**: Complete Phase 3 (all extended APIs)
- [ ] **M5**: Complete Phase 4 (advanced APIs)
- [ ] **M6**: Comprehensive validation
- [ ] **M7**: Version conversion support
- [ ] **M8**: 80% test coverage
- [ ] **M9**: First release (v0.1.0)
- [ ] **M10**: Stable release (v1.0.0)

---

## Contributing

See contribution guidelines in the main README.md.

For specific tasks, check this plan for:
- Priority levels (P1-P4)
- Estimated effort
- Dependencies

Feel free to pick any task and submit a PR!

---

**Last Updated**: 2026-01-13

**Version**: 0.1.0

**Status**: Phase 1 in progress
