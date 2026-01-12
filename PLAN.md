# Rust 重写 Kubernetes 1.34 API 模块实现计划

## 概述

将 Kubernetes 1.34 API 模块从 Go 重写为 Rust，完全兼容现有 K8s API 协议。

**原代码**: `../api/` (439个Go文件，~42万行代码，26个API组，61个版本)

**目标**:
- 全部26个API组（61个版本）
- 完全兼容（JSON + Protobuf序列化）
- 使用Rust derive宏
- 兼容kube-rs生态

**仓库**: https://github.com/chenwenxiaolive/k8s-api-rust

---

## 当前进度

### ✅ 已完成
- [x] 创建GitHub仓库
- [x] 初始化Cargo workspace结构
- [x] k8s-api-core crate
  - [x] DeepCopy trait
  - [x] Resource trait
  - [x] IntOrString 类型
- [x] k8s-api-meta crate
  - [x] TypeMeta
  - [x] ObjectMeta
  - [x] OwnerReference
  - [x] LabelSelector

### 🔲 下一步任务
- [ ] k8s-api-core 补充
  - [ ] Quantity 类型（资源数量，如 "100m", "1Gi"）
  - [ ] Time, MicroTime, Duration 类型
- [ ] k8s-api-core-v1 crate（核心API，最大最重要）
  - [ ] Pod, PodSpec, PodStatus
  - [ ] Container, ContainerPort
  - [ ] Service, ServiceSpec
  - [ ] ConfigMap, Secret
  - [ ] PersistentVolume, PersistentVolumeClaim
  - [ ] Node, Namespace
  - [ ] ... (共308+类型)

---

## 1. 项目结构

```
k8s-api-rust/
├── Cargo.toml                          # Workspace root
├── crates/
│   ├── k8s-api-core/                   # ✅ 核心traits和原语类型
│   ├── k8s-api-meta/                   # ✅ apimachinery类型
│   ├── k8s-api-core-v1/                # 🔲 core/v1 API组
│   ├── k8s-api-apps-v1/                # 🔲 apps/v1
│   ├── k8s-api-batch-v1/               # 🔲 batch/v1
│   ├── ... (61个API版本crate)
│   ├── k8s-api/                        # 🔲 总览crate
│   └── k8s-api-codegen/                # 🔲 代码生成工具
├── proto/                              # 🔲 Protobuf定义文件
└── tests/                              # 🔲 兼容性测试
```

---

## 2. 核心依赖

```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
prost = "0.13"
prost-build = "0.13"
kube = { version = "0.88", default-features = false }
k8s-openapi = { version = "0.21", features = ["latest"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
```

---

## 3. 类型映射策略

| Go类型 | Rust类型 |
|--------|----------|
| `string` | `String` |
| `*T` (pointer) | `Option<T>` |
| `[]T` (slice) | `Vec<T>` |
| `map[K]V` | `BTreeMap<K, V>` |
| `metav1.Time` | `chrono::DateTime<Utc>` |
| `resource.Quantity` | 自定义 `Quantity` |
| `intstr.IntOrString` | `enum IntOrString { Int(i32), String(String) }` |

**示例转换**:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}
```

---

## 4. 实现顺序（按依赖关系）

### 阶段1: 基础设施 ✅
1. **k8s-api-core** - DeepCopy trait, Resource trait, Quantity, IntOrString
2. **k8s-api-meta** - TypeMeta, ObjectMeta, LabelSelector, Time

### 阶段2: 核心API 🔲 ← 下一步
3. **k8s-api-core-v1** - Pod, Service, ConfigMap, Secret等 (308+类型，最大)

### 阶段3: 工作负载API 🔲
4. **k8s-api-apps-v1** - Deployment, StatefulSet, DaemonSet, ReplicaSet
5. **k8s-api-batch-v1** - Job, CronJob
6. apps/v1beta1, apps/v1beta2, batch/v1beta1

### 阶段4: 网络与安全 🔲
7. **k8s-api-networking-v1** - NetworkPolicy, Ingress
8. **k8s-api-rbac-v1** - Role, ClusterRole, RoleBinding
9. networking/v1beta1, rbac/v1alpha1, rbac/v1beta1

### 阶段5: 自动扩展与存储 🔲
10. **k8s-api-autoscaling-v1/v2** - HPA
11. **k8s-api-storage-v1** - StorageClass, CSI

### 阶段6: 剩余API组 🔲
12. admission, admissionregistration, authentication, authorization
13. certificates, coordination, discovery, events
14. flowcontrol, node, policy, resource, scheduling
15. apidiscovery, apiserverinternal, extensions, imagepolicy, storagemigration

### 阶段7: 集成 🔲
16. **k8s-api** umbrella crate
17. kube-rs兼容层实现
18. 完整测试套件

---

## 5. 代码生成策略

采用**混合方式**: 生成器 + 手动审查

1. 构建 `k8s-api-codegen` 工具解析Go AST
2. 自动生成基础Rust代码
3. 手动审查和修复边缘情况

---

## 6. 测试策略

1. **Roundtrip测试** - JSON序列化/反序列化无损
2. **兼容性测试** - 使用 `testdata/HEAD/` 中的182个测试fixture
3. **kube-rs集成测试** - 验证与kube-rs客户端的兼容性
4. **Fuzzing** - 确保反序列化不会panic

---

## 7. 关键文件参考

| 文件 | 用途 |
|------|------|
| `api/core/v1/types.go` | 核心API类型定义 (8375行) |
| `api/core/v1/generated.proto` | Protobuf schema |
| `api/apps/v1/types.go` | 工作负载类型 |
| `api/testdata/HEAD/` | 182个测试fixture |
| `api/roundtrip_test.go` | 所有61个版本的导入结构 |

---

## 8. 明天继续的任务

1. **补充 k8s-api-core**:
   - 实现 `Quantity` 类型（解析 "100m", "1Gi" 等资源数量格式）
   - 实现 `Time`, `MicroTime`, `Duration` 类型

2. **开始 k8s-api-core-v1**:
   - 创建 crate 结构
   - 从 `api/core/v1/types.go` 开始转换核心类型
   - 优先实现: Pod, Container, Service, ConfigMap, Secret

3. **考虑代码生成**:
   - 评估是否需要构建代码生成工具
   - 或者手动转换（308个类型工作量大）
