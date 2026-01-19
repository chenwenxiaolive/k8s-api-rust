# Test Migration Report

Scope: Kubernetes Go tests under:
- staging/src/k8s.io/api
- pkg/apis

Total Go test files in scope: 99

Notes:
- Protobuf reserved field checks for NetworkPolicy are covered in `crates/k8s-api-codec` tests.
- Conversion list round-trip and missing-field coverage is in `crates/k8s-api-conversion/tests/list_roundtrip.rs`.
- Go scheme/install/roundtrip tests are not directly portable; coverage is via `crates/k8s-api-codec` tests and `crates/k8s-api/tests/serialization.rs`.

## Category Counts
- types: 4
- conversion: 19
- defaults: 33
- validation: 29
- other: 14

## Group Coverage Summary
| Group | Go tests | types | conversion | defaults | validation | Rust tests |
|-------|---------:|------:|-----------:|---------:|-----------:|------------|
| abac | 2 | 0 | 2 | 0 | 0 | conversion, validation |
| admissionregistration | 4 | 0 | 0 | 3 | 1 | conversion, defaults, validation |
| apiserverinternal | 2 | 0 | 0 | 0 | 1 | validation |
| apps | 6 | 0 | 2 | 3 | 1 | api, conversion, defaults, validation |
| authorization | 1 | 0 | 0 | 0 | 1 | conversion, validation |
| autoscaling | 9 | 0 | 3 | 4 | 2 | conversion, defaults, validation |
| batch | 3 | 0 | 0 | 2 | 1 | api, conversion, defaults, validation |
| certificates | 2 | 0 | 0 | 1 | 1 | conversion, defaults, validation |
| coordination | 1 | 0 | 0 | 0 | 1 | api, conversion, validation |
| core | 17 | 2 | 1 | 1 | 4 | api, conversion, defaults (partial), validation, helpers (taint/toleration + core helper + qos) |
| discovery | 4 | 0 | 1 | 2 | 1 | conversion, defaults, validation |
| extensions | 3 | 1 | 1 | 1 | 0 | conversion, defaults, validation |
| flowcontrol | 9 | 0 | 3 | 5 | 1 | conversion, defaults, validation |
| networking | 5 | 1 | 1 | 2 | 1 | conversion, defaults, validation |
| node | 2 | 0 | 1 | 0 | 1 | conversion, validation |
| policy | 3 | 0 | 2 | 0 | 1 | conversion, validation |
| rbac | 3 | 0 | 1 | 0 | 1 | api, conversion, validation, helpers |
| resource | 13 | 0 | 1 | 4 | 7 | conversion, defaults, validation |
| roundtrip_test.go | 1 | 0 | 0 | 0 | 0 | none |
| scheduling | 5 | 0 | 0 | 3 | 1 | api, conversion, defaults, validation, helpers |
| storage | 3 | 0 | 0 | 2 | 1 | conversion, defaults, validation |
| storagemigration | 1 | 0 | 0 | 0 | 1 | validation |

## Detailed Go Test Files
- pkg/apis/abac/v0/conversion_test.go [abac] (conversion) -> rust_conversion
- pkg/apis/abac/v1beta1/conversion_test.go [abac] (conversion) -> rust_conversion
- pkg/apis/admissionregistration/v1/defaults_test.go [admissionregistration] (defaults) -> rust_defaults
- pkg/apis/admissionregistration/v1alpha1/defaults_test.go [admissionregistration] (defaults) -> rust_defaults
- pkg/apis/admissionregistration/v1beta1/defaults_test.go [admissionregistration] (defaults) -> rust_defaults
- pkg/apis/admissionregistration/validation/validation_test.go [admissionregistration] (validation) -> rust_validation
- pkg/apis/apiserverinternal/install/roundtrip_test.go [apiserverinternal] (other) -> not_applicable (scheme roundtrip; see codec + serialization tests)
- pkg/apis/apiserverinternal/validation/validation_test.go [apiserverinternal] (validation) -> rust_validation
- pkg/apis/apps/v1/conversion_test.go [apps] (conversion) -> rust_conversion
- pkg/apis/apps/v1beta2/conversion_test.go [apps] (conversion) -> rust_conversion
- pkg/apis/apps/v1/defaults_test.go [apps] (defaults) -> rust_defaults
- pkg/apis/apps/v1beta1/defaults_test.go [apps] (defaults) -> rust_defaults
- pkg/apis/apps/v1beta2/defaults_test.go [apps] (defaults) -> rust_defaults
- pkg/apis/apps/validation/validation_test.go [apps] (validation) -> rust_validation
- pkg/apis/authorization/validation/validation_test.go [authorization] (validation) -> rust_validation
- pkg/apis/autoscaling/v1/conversion_test.go [autoscaling] (conversion) -> rust_conversion
- pkg/apis/autoscaling/v2beta1/conversion_test.go [autoscaling] (conversion) -> rust_conversion
- pkg/apis/autoscaling/v2beta2/conversion_test.go [autoscaling] (conversion) -> rust_conversion
- pkg/apis/autoscaling/v1/defaults_test.go [autoscaling] (defaults) -> rust_defaults
- pkg/apis/autoscaling/v2/defaults_test.go [autoscaling] (defaults) -> rust_defaults
- pkg/apis/autoscaling/v2beta1/defaults_test.go [autoscaling] (defaults) -> rust_defaults
- pkg/apis/autoscaling/v2beta2/defaults_test.go [autoscaling] (defaults) -> rust_defaults
- pkg/apis/autoscaling/validation/declarative_validation_test.go [autoscaling] (validation) -> rust_validation
- pkg/apis/autoscaling/validation/validation_test.go [autoscaling] (validation) -> rust_validation
- pkg/apis/batch/v1/defaults_test.go [batch] (defaults) -> rust_defaults
- pkg/apis/batch/v1beta1/defaults_test.go [batch] (defaults) -> rust_defaults
- pkg/apis/batch/validation/validation_test.go [batch] (validation) -> rust_validation
- pkg/apis/certificates/v1beta1/defaults_test.go [certificates] (defaults) -> rust_defaults
- pkg/apis/certificates/validation/validation_test.go [certificates] (validation) -> rust_validation
- pkg/apis/coordination/validation/validation_test.go [coordination] (validation) -> rust_validation
- pkg/apis/core/v1/conversion_test.go [core] (conversion) -> rust_conversion (PodSpec/PodStatus/NodeSpec alias mapping + PodLogOptions query params + RC<->RS conversion)
- pkg/apis/core/v1/defaults_test.go [core] (defaults) -> rust_defaults (partial: pod/container/probe/volume/secret/pv/pvc/endpoints/node/service/limitrange/namespace/replicationcontroller basics + pod requests from limits + pod-level resources (cpu/memory/hugepages, init container aggregation) + volume source iscsi/rbd/azure/scaleio + ephemeral claim template + pod log options (feature off by default) + image volume pull policy (feature off by default) + resource list rounding (decimal + m + micro/nano + exponent + no-op + large values) + service load balancer IP mode)
- pkg/apis/core/helper/helpers_test.go [core] (other) -> rust_helper (resource/access mode/topology helpers)
- pkg/apis/core/install/install_test.go [core] (other) -> not_applicable (scheme install/codec; see codec + serialization tests)
- pkg/apis/core/pods/helpers_test.go [core] (other) -> rust_pods (container visitor + downward API label)
- pkg/apis/core/taint_test.go [core] (other) -> rust_api_tests (taint helpers)
- pkg/apis/core/toleration_test.go [core] (other) -> rust_api_tests (toleration helpers)
- pkg/apis/core/v1/helper/helpers_test.go [core] (other) -> rust_helper (resource/access mode/topology helpers)
- pkg/apis/core/v1/helper/qos/qos_test.go [core] (other) -> rust_qos
- staging/src/k8s.io/api/core/v1/taint_test.go [core] (other) -> rust_api_tests (taint helpers)
- staging/src/k8s.io/api/core/v1/toleration_test.go [core] (other) -> rust_api_tests (toleration helpers)
- pkg/apis/core/types_test.go [core] (types) -> rust_api_tests (ephemeral container alias)
- staging/src/k8s.io/api/core/v1/types_test.go [core] (types) -> rust_api_tests + k8s-api-codec (ServiceSpec protobuf reserved fields)
- pkg/apis/core/v1/validation/validation_test.go [core] (validation) -> rust_validation
- pkg/apis/core/validation/events_test.go [core] (validation) -> rust_validation
- pkg/apis/core/validation/names_test.go [core] (validation) -> rust_validation
- pkg/apis/core/validation/validation_test.go [core] (validation) -> rust_validation
- pkg/apis/discovery/v1beta1/conversion_test.go [discovery] (conversion) -> rust_conversion
- pkg/apis/discovery/v1/defaults_test.go [discovery] (defaults) -> rust_defaults
- pkg/apis/discovery/v1beta1/defaults_test.go [discovery] (defaults) -> rust_defaults
- pkg/apis/discovery/validation/validation_test.go [discovery] (validation) -> rust_validation
- pkg/apis/extensions/v1beta1/conversion_test.go [extensions] (conversion) -> rust_conversion
- pkg/apis/extensions/v1beta1/defaults_test.go [extensions] (defaults) -> rust_defaults
- staging/src/k8s.io/api/extensions/v1beta1/types_test.go [extensions] (types) -> covered by k8s-api-codec tests
- pkg/apis/flowcontrol/v1beta1/conversion_test.go [flowcontrol] (conversion) -> rust_conversion
- pkg/apis/flowcontrol/v1beta2/conversion_test.go [flowcontrol] (conversion) -> rust_conversion
- pkg/apis/flowcontrol/v1beta3/conversion_test.go [flowcontrol] (conversion) -> rust_conversion
- pkg/apis/flowcontrol/internalbootstrap/defaults_test.go [flowcontrol] (defaults) -> rust_defaults (flowcontrol/v1 bootstrap no-op)
- pkg/apis/flowcontrol/v1/defaults_test.go [flowcontrol] (defaults) -> rust_defaults
- pkg/apis/flowcontrol/v1beta1/defaults_test.go [flowcontrol] (defaults) -> rust_defaults
- pkg/apis/flowcontrol/v1beta2/defaults_test.go [flowcontrol] (defaults) -> rust_defaults
- pkg/apis/flowcontrol/v1beta3/defaults_test.go [flowcontrol] (defaults) -> rust_defaults
- pkg/apis/flowcontrol/validation/validation_test.go [flowcontrol] (validation) -> rust_validation
- pkg/apis/networking/v1beta1/conversion_test.go [networking] (conversion) -> rust_conversion
- pkg/apis/networking/v1/defaults_test.go [networking] (defaults) -> rust_defaults
- pkg/apis/networking/v1beta1/defaults_test.go [networking] (defaults) -> rust_defaults
- staging/src/k8s.io/api/networking/v1/types_test.go [networking] (types) -> covered by k8s-api-codec tests
- pkg/apis/networking/validation/validation_test.go [networking] (validation) -> rust_validation
- pkg/apis/node/v1alpha1/conversion_test.go [node] (conversion) -> rust_conversion
- pkg/apis/node/validation/validation_test.go [node] (validation) -> rust_validation
- pkg/apis/policy/v1/conversion_test.go [policy] (conversion) -> rust_conversion
- pkg/apis/policy/v1beta1/conversion_test.go [policy] (conversion) -> rust_conversion
- pkg/apis/policy/validation/validation_test.go [policy] (validation) -> rust_validation
- pkg/apis/rbac/v1alpha1/conversion_test.go [rbac] (conversion) -> rust_conversion
- pkg/apis/rbac/helpers_test.go [rbac] (other) -> rust_helper
- pkg/apis/rbac/validation/validation_test.go [rbac] (validation) -> rust_validation
- pkg/apis/resource/v1beta1/conversion_test.go [resource] (conversion) -> rust_conversion
- pkg/apis/resource/v1/defaults_test.go [resource] (defaults) -> rust_defaults
- pkg/apis/resource/v1alpha3/defaults_test.go [resource] (defaults) -> rust_defaults
- pkg/apis/resource/v1beta1/defaults_test.go [resource] (defaults) -> rust_defaults
- pkg/apis/resource/v1beta2/defaults_test.go [resource] (defaults) -> rust_defaults
- pkg/apis/resource/install/install_test.go [resource] (other) -> not_applicable (scheme install/codec; see codec + serialization tests)
- pkg/apis/resource/validation/validation_common_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_device_capacity_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_deviceclass_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_devicetaintrule_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_resourceclaim_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_resourceclaimtemplate_test.go [resource] (validation) -> rust_validation
- pkg/apis/resource/validation/validation_resourceslice_test.go [resource] (validation) -> rust_validation
- staging/src/k8s.io/api/roundtrip_test.go [roundtrip_test.go] (other) -> not_applicable (scheme fuzzer roundtrip; see codec + serialization tests)
- pkg/apis/scheduling/v1/defaults_test.go [scheduling] (defaults) -> rust_defaults
- pkg/apis/scheduling/v1alpha1/defaults_test.go [scheduling] (defaults) -> rust_defaults
- pkg/apis/scheduling/v1beta1/defaults_test.go [scheduling] (defaults) -> rust_defaults
- pkg/apis/scheduling/v1/helpers_test.go [scheduling] (other) -> rust_helper
- pkg/apis/scheduling/validation/validation_test.go [scheduling] (validation) -> rust_validation
- pkg/apis/storage/v1/defaults_test.go [storage] (defaults) -> rust_defaults
- pkg/apis/storage/v1beta1/defaults_test.go [storage] (defaults) -> rust_defaults
- pkg/apis/storage/validation/validation_test.go [storage] (validation) -> rust_validation
- pkg/apis/storagemigration/validation/validation_test.go [storagemigration] (validation) -> rust_validation
