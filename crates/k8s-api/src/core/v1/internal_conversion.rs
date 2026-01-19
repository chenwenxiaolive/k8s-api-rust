use super::*;
use serde_json::Value;
use std::collections::BTreeMap;

fn ensure_default_fields(value: &mut Value, defaults: &[(&str, Value)]) {
    let map = match value {
        Value::Object(map) => map,
        _ => return,
    };
    for (key, default) in defaults {
        if !map.contains_key(*key) {
            map.insert((*key).to_string(), default.clone());
        }
    }
}

fn ensure_resource_requirements_defaults(value: &mut Value) {
    fn ensure_requirement_fields(value: &mut Value) {
        let map = match value {
            Value::Object(map) => map,
            _ => return,
        };
        if !map.contains_key("limits") {
            map.insert("limits".to_string(), Value::Object(Default::default()));
        }
        if !map.contains_key("requests") {
            map.insert("requests".to_string(), Value::Object(Default::default()));
        }
        if !map.contains_key("claims") {
            map.insert("claims".to_string(), Value::Array(Vec::new()));
        }
    }

    fn ensure_container_resources(value: &mut Value) {
        ensure_default_fields(
            value,
            &[
                ("image", Value::String(String::new())),
                ("command", Value::Array(Vec::new())),
                ("args", Value::Array(Vec::new())),
                ("workingDir", Value::String(String::new())),
                ("ports", Value::Array(Vec::new())),
                ("envFrom", Value::Array(Vec::new())),
                ("env", Value::Array(Vec::new())),
                ("resizePolicy", Value::Array(Vec::new())),
                ("restartPolicyRules", Value::Array(Vec::new())),
                ("volumeMounts", Value::Array(Vec::new())),
                ("volumeDevices", Value::Array(Vec::new())),
                ("terminationMessagePath", Value::String(String::new())),
                ("terminationMessagePolicy", Value::String(String::new())),
                ("imagePullPolicy", Value::String(String::new())),
                ("stdin", Value::Bool(false)),
                ("stdinOnce", Value::Bool(false)),
                ("tty", Value::Bool(false)),
            ],
        );
        let map = match value {
            Value::Object(map) => map,
            _ => return,
        };
        if let Some(resources) = map.get_mut("resources") {
            ensure_requirement_fields(resources);
        }
    }

    let map = match value {
        Value::Object(map) => map,
        _ => return,
    };

    if let Some(resources) = map.get_mut("resources") {
        ensure_requirement_fields(resources);
    }

    for key in ["containers", "initContainers", "ephemeralContainers"] {
        if let Some(Value::Array(items)) = map.get_mut(key) {
            for item in items {
                ensure_container_resources(item);
            }
        }
    }
}

const INIT_CONTAINER_ANNOTATIONS: [&str; 4] = [
    "pod.beta.kubernetes.io/init-containers",
    "pod.alpha.kubernetes.io/init-containers",
    "pod.beta.kubernetes.io/init-container-statuses",
    "pod.alpha.kubernetes.io/init-container-statuses",
];

fn drop_init_container_annotations(annotations: &mut BTreeMap<String, String>) {
    if annotations.is_empty() {
        return;
    }
    if !INIT_CONTAINER_ANNOTATIONS
        .iter()
        .any(|key| annotations.contains_key(*key))
    {
        return;
    }
    annotations.retain(|key, _| {
        !INIT_CONTAINER_ANNOTATIONS
            .iter()
            .any(|candidate| candidate == &key.as_str())
    });
}

fn clamp_negative_termination_grace(value: &mut Option<i64>) {
    if let Some(current) = value.as_mut() {
        if *current < 0 {
            *current = 1;
        }
    }
}

fn merge_string_data(
    data: &mut BTreeMap<String, Vec<u8>>,
    string_data: &mut BTreeMap<String, String>,
) {
    if string_data.is_empty() {
        return;
    }
    for (key, value) in string_data.iter() {
        data.insert(key.clone(), value.as_bytes().to_vec());
    }
    string_data.clear();
}

impl InternalConversion for Pod {
    type Internal = crate::core::internal::Pod;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let spec = match self.spec.as_ref() {
            Some(spec) => Some(spec.into_internal()?),
            None => None,
        };
        let status = match self.status.as_ref() {
            Some(status) => Some(status.into_internal()?),
            None => None,
        };
        let mut internal = Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            spec,
            status,
        };
        drop_init_container_annotations(&mut internal.metadata.annotations);
        if let Some(spec) = internal.spec.as_mut() {
            clamp_negative_termination_grace(&mut spec.termination_grace_period_seconds);
        }
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let spec = match internal.spec.as_ref() {
            Some(spec) => Some(PodSpec::from_internal(spec)?),
            None => None,
        };
        let status = match internal.status.as_ref() {
            Some(status) => Some(PodStatus::from_internal(status)?),
            None => None,
        };
        let mut external = Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            spec,
            status,
        };
        drop_init_container_annotations(&mut external.metadata.annotations);
        if let Some(spec) = external.spec.as_mut() {
            clamp_negative_termination_grace(&mut spec.termination_grace_period_seconds);
        }
        Ok(external)
    }
}

impl InternalConversion for PodList {
    type Internal = crate::core::internal::PodList;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut items = Vec::with_capacity(self.items.len());
        for pod in &self.items {
            items.push(pod.into_internal()?);
        }
        Ok(Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut items = Vec::with_capacity(internal.items.len());
        for pod in &internal.items {
            items.push(Pod::from_internal(pod)?);
        }
        Ok(Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            items,
        })
    }
}

impl InternalConversion for PodSpec {
    type Internal = crate::core::internal::PodSpec;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        ensure_default_fields(
            &mut value,
            &[
                ("initContainers", Value::Array(Vec::new())),
                ("ephemeralContainers", Value::Array(Vec::new())),
                ("volumes", Value::Array(Vec::new())),
                ("restartPolicy", Value::String(String::new())),
                ("dnsPolicy", Value::String(String::new())),
                ("nodeSelector", Value::Object(Default::default())),
                ("serviceAccountName", Value::String(String::new())),
                ("serviceAccount", Value::String(String::new())),
                ("nodeName", Value::String(String::new())),
                ("hostNetwork", Value::Bool(false)),
                ("hostPID", Value::Bool(false)),
                ("hostIPC", Value::Bool(false)),
                ("imagePullSecrets", Value::Array(Vec::new())),
                ("hostname", Value::String(String::new())),
                ("subdomain", Value::String(String::new())),
                ("schedulerName", Value::String(String::new())),
                ("tolerations", Value::Array(Vec::new())),
                ("hostAliases", Value::Array(Vec::new())),
                ("priorityClassName", Value::String(String::new())),
                ("readinessGates", Value::Array(Vec::new())),
                ("overhead", Value::Object(Default::default())),
                ("topologySpreadConstraints", Value::Array(Vec::new())),
                ("schedulingGates", Value::Array(Vec::new())),
                ("resourceClaims", Value::Array(Vec::new())),
            ],
        );
        ensure_resource_requirements_defaults(&mut value);
        let mut internal: Self::Internal = serde_json::from_value(value)?;
        if internal.service_account_name.is_empty() {
            internal.service_account_name = internal.service_account.clone();
        }
        if !internal.service_account_name.is_empty() {
            internal.service_account = internal.service_account_name.clone();
        }
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut external: Self = serde_json::from_value(serde_json::to_value(internal)?)?;
        if external.service_account_name.is_empty() {
            external.service_account_name = external.service_account.clone();
        }
        if !external.service_account_name.is_empty() {
            external.service_account = external.service_account_name.clone();
        }
        Ok(external)
    }
}

impl InternalConversion for PodStatus {
    type Internal = crate::core::internal::PodStatus;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        ensure_default_fields(
            &mut value,
            &[
                ("phase", Value::String(String::new())),
                ("conditions", Value::Array(Vec::new())),
                ("message", Value::String(String::new())),
                ("reason", Value::String(String::new())),
                ("nominatedNodeName", Value::String(String::new())),
                ("hostIP", Value::String(String::new())),
                ("hostIPs", Value::Array(Vec::new())),
                ("podIP", Value::String(String::new())),
                ("podIPs", Value::Array(Vec::new())),
                ("initContainerStatuses", Value::Array(Vec::new())),
                ("containerStatuses", Value::Array(Vec::new())),
                ("ephemeralContainerStatuses", Value::Array(Vec::new())),
                ("qosClass", Value::String(String::new())),
                ("resize", Value::String(String::new())),
                ("resourceClaimStatuses", Value::Array(Vec::new())),
            ],
        );
        let mut internal: Self::Internal = serde_json::from_value(value)?;
        if !internal.pod_i_p.is_empty() {
            if internal.pod_i_ps.is_empty()
                || internal
                    .pod_i_ps
                    .first()
                    .map(|ip| ip.ip.as_str() != internal.pod_i_p.as_str())
                    .unwrap_or(true)
            {
                internal.pod_i_ps = vec![crate::core::internal::PodIP {
                    ip: internal.pod_i_p.clone(),
                }];
            }
        } else if !internal.pod_i_ps.is_empty() {
            internal.pod_i_p = internal.pod_i_ps[0].ip.clone();
        }
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut external: Self = serde_json::from_value(serde_json::to_value(internal)?)?;
        if !external.pod_i_ps.is_empty() {
            external.pod_i_p = external.pod_i_ps[0].ip.clone();
        }
        Ok(external)
    }
}

impl InternalConversion for PodCondition {
    type Internal = crate::core::internal::PodCondition;
}

impl InternalConversion for PodIP {
    type Internal = crate::core::internal::PodIP;
}

impl InternalConversion for HostIP {
    type Internal = crate::core::internal::HostIP;
}

impl InternalConversion for PodTemplate {
    type Internal = crate::core::internal::PodTemplate;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let template = match self.template.as_ref() {
            Some(template) => Some(template.into_internal()?),
            None => None,
        };
        Ok(Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            template,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let template = match internal.template.as_ref() {
            Some(template) => Some(PodTemplateSpec::from_internal(template)?),
            None => None,
        };
        Ok(Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            template,
        })
    }
}

impl InternalConversion for PodTemplateSpec {
    type Internal = crate::core::internal::PodTemplateSpec;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let spec = match self.spec.as_ref() {
            Some(spec) => Some(spec.into_internal()?),
            None => None,
        };
        let mut internal = Self::Internal {
            metadata: self.metadata.clone(),
            spec,
        };
        drop_init_container_annotations(&mut internal.metadata.annotations);
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let spec = match internal.spec.as_ref() {
            Some(spec) => Some(PodSpec::from_internal(spec)?),
            None => None,
        };
        let mut external = Self {
            metadata: internal.metadata.clone(),
            spec,
        };
        drop_init_container_annotations(&mut external.metadata.annotations);
        Ok(external)
    }
}

impl InternalConversion for PodTemplateList {
    type Internal = crate::core::internal::PodTemplateList;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut items = Vec::with_capacity(self.items.len());
        for template in &self.items {
            items.push(template.into_internal()?);
        }
        Ok(Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut items = Vec::with_capacity(internal.items.len());
        for template in &internal.items {
            items.push(PodTemplate::from_internal(template)?);
        }
        Ok(Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            items,
        })
    }
}

impl InternalConversion for Container {
    type Internal = crate::core::internal::Container;
}

impl InternalConversion for EphemeralContainer {
    type Internal = crate::core::internal::EphemeralContainer;
}

impl InternalConversion for ContainerPort {
    type Internal = crate::core::internal::ContainerPort;
}

impl InternalConversion for ContainerStatus {
    type Internal = crate::core::internal::ContainerStatus;
}

impl InternalConversion for ContainerState {
    type Internal = crate::core::internal::ContainerState;
}

impl InternalConversion for ContainerStateWaiting {
    type Internal = crate::core::internal::ContainerStateWaiting;
}

impl InternalConversion for ContainerStateRunning {
    type Internal = crate::core::internal::ContainerStateRunning;
}

impl InternalConversion for ContainerStateTerminated {
    type Internal = crate::core::internal::ContainerStateTerminated;
}

impl InternalConversion for ContainerResizePolicy {
    type Internal = crate::core::internal::ContainerResizePolicy;
}

impl InternalConversion for ContainerRestartRule {
    type Internal = crate::core::internal::ContainerRestartRule;
}

impl InternalConversion for ContainerRestartRuleOnExitCodes {
    type Internal = crate::core::internal::ContainerRestartRuleOnExitCodes;
}

impl InternalConversion for VolumeMountStatus {
    type Internal = crate::core::internal::VolumeMountStatus;
}

impl InternalConversion for ResourceStatus {
    type Internal = crate::core::internal::ResourceStatus;
}

impl InternalConversion for ResourceHealth {
    type Internal = crate::core::internal::ResourceHealth;
}

impl InternalConversion for ContainerUser {
    type Internal = crate::core::internal::ContainerUser;
}

impl InternalConversion for LinuxContainerUser {
    type Internal = crate::core::internal::LinuxContainerUser;
}

impl InternalConversion for EnvVar {
    type Internal = crate::core::internal::EnvVar;
}

impl InternalConversion for EnvVarSource {
    type Internal = crate::core::internal::EnvVarSource;
}

impl InternalConversion for ObjectFieldSelector {
    type Internal = crate::core::internal::ObjectFieldSelector;
}

impl InternalConversion for ResourceFieldSelector {
    type Internal = crate::core::internal::ResourceFieldSelector;
}

impl InternalConversion for ConfigMapKeySelector {
    type Internal = crate::core::internal::ConfigMapKeySelector;
}

impl InternalConversion for SecretKeySelector {
    type Internal = crate::core::internal::SecretKeySelector;
}

impl InternalConversion for FileKeySelector {
    type Internal = crate::core::internal::FileKeySelector;
}

impl InternalConversion for EnvFromSource {
    type Internal = crate::core::internal::EnvFromSource;
}

impl InternalConversion for ConfigMapEnvSource {
    type Internal = crate::core::internal::ConfigMapEnvSource;
}

impl InternalConversion for SecretEnvSource {
    type Internal = crate::core::internal::SecretEnvSource;
}

impl InternalConversion for ResourceRequirements {
    type Internal = crate::core::internal::ResourceRequirements;
}

impl InternalConversion for ResourceClaim {
    type Internal = crate::core::internal::ResourceClaim;
}

impl InternalConversion for Volume {
    type Internal = crate::core::internal::Volume;
}

impl InternalConversion for VolumeSource {
    type Internal = crate::core::internal::VolumeSource;
}

impl InternalConversion for HostPathVolumeSource {
    type Internal = crate::core::internal::HostPathVolumeSource;
}

impl InternalConversion for EmptyDirVolumeSource {
    type Internal = crate::core::internal::EmptyDirVolumeSource;
}

impl InternalConversion for SecretVolumeSource {
    type Internal = crate::core::internal::SecretVolumeSource;
}

impl InternalConversion for ConfigMapVolumeSource {
    type Internal = crate::core::internal::ConfigMapVolumeSource;
}

impl InternalConversion for KeyToPath {
    type Internal = crate::core::internal::KeyToPath;
}

impl InternalConversion for PersistentVolumeClaimVolumeSource {
    type Internal = crate::core::internal::PersistentVolumeClaimVolumeSource;
}

impl InternalConversion for ProjectedVolumeSource {
    type Internal = crate::core::internal::ProjectedVolumeSource;
}

impl InternalConversion for VolumeProjection {
    type Internal = crate::core::internal::VolumeProjection;
}

impl InternalConversion for SecretProjection {
    type Internal = crate::core::internal::SecretProjection;
}

impl InternalConversion for ConfigMapProjection {
    type Internal = crate::core::internal::ConfigMapProjection;
}

impl InternalConversion for DownwardAPIProjection {
    type Internal = crate::core::internal::DownwardAPIProjection;
}

impl InternalConversion for ServiceAccountTokenProjection {
    type Internal = crate::core::internal::ServiceAccountTokenProjection;
}

impl InternalConversion for ClusterTrustBundleProjection {
    type Internal = crate::core::internal::ClusterTrustBundleProjection;
}

impl InternalConversion for PodCertificateProjection {
    type Internal = crate::core::internal::PodCertificateProjection;
}

impl InternalConversion for DownwardAPIVolumeSource {
    type Internal = crate::core::internal::DownwardAPIVolumeSource;
}

impl InternalConversion for DownwardAPIVolumeFile {
    type Internal = crate::core::internal::DownwardAPIVolumeFile;
}

impl InternalConversion for CSIVolumeSource {
    type Internal = crate::core::internal::CSIVolumeSource;
}

impl InternalConversion for VolumeMount {
    type Internal = crate::core::internal::VolumeMount;
}

impl InternalConversion for VolumeDevice {
    type Internal = crate::core::internal::VolumeDevice;
}

impl InternalConversion for Service {
    type Internal = crate::core::internal::Service;
}

impl InternalConversion for ServiceList {
    type Internal = crate::core::internal::ServiceList;
}

impl InternalConversion for ServiceSpec {
    type Internal = crate::core::internal::ServiceSpec;
}

impl InternalConversion for ServicePort {
    type Internal = crate::core::internal::ServicePort;
}

impl InternalConversion for ServiceStatus {
    type Internal = crate::core::internal::ServiceStatus;
}

impl InternalConversion for LoadBalancerStatus {
    type Internal = crate::core::internal::LoadBalancerStatus;
}

impl InternalConversion for LoadBalancerIngress {
    type Internal = crate::core::internal::LoadBalancerIngress;
}

impl InternalConversion for PortStatus {
    type Internal = crate::core::internal::PortStatus;
}

impl InternalConversion for SessionAffinityConfig {
    type Internal = crate::core::internal::SessionAffinityConfig;
}

impl InternalConversion for ClientIPConfig {
    type Internal = crate::core::internal::ClientIPConfig;
}

impl InternalConversion for ConfigMap {
    type Internal = crate::core::internal::ConfigMap;
}

impl InternalConversion for ConfigMapList {
    type Internal = crate::core::internal::ConfigMapList;
}

impl InternalConversion for Secret {
    type Internal = crate::core::internal::Secret;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut internal = Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            immutable: self.immutable,
            data: self.data.clone(),
            string_data: self.string_data.clone(),
            secret_type: self.secret_type.clone(),
        };
        merge_string_data(&mut internal.data, &mut internal.string_data);
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut external = Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            immutable: internal.immutable,
            data: internal.data.clone(),
            string_data: internal.string_data.clone(),
            secret_type: internal.secret_type.clone(),
        };
        merge_string_data(&mut external.data, &mut external.string_data);
        Ok(external)
    }
}

impl InternalConversion for SecretList {
    type Internal = crate::core::internal::SecretList;
}

impl InternalConversion for Namespace {
    type Internal = crate::core::internal::Namespace;
}

impl InternalConversion for NamespaceSpec {
    type Internal = crate::core::internal::NamespaceSpec;
}

impl InternalConversion for NamespaceStatus {
    type Internal = crate::core::internal::NamespaceStatus;
}

impl InternalConversion for NamespaceCondition {
    type Internal = crate::core::internal::NamespaceCondition;
}

impl InternalConversion for NamespaceList {
    type Internal = crate::core::internal::NamespaceList;
}

impl InternalConversion for Node {
    type Internal = crate::core::internal::Node;
}

impl InternalConversion for NodeSpec {
    type Internal = crate::core::internal::NodeSpec;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        ensure_default_fields(
            &mut value,
            &[
                ("podCIDR", Value::String(String::new())),
                ("podCIDRs", Value::Array(Vec::new())),
                ("providerID", Value::String(String::new())),
                ("unschedulable", Value::Bool(false)),
                ("taints", Value::Array(Vec::new())),
                ("externalID", Value::String(String::new())),
            ],
        );
        let mut internal: Self::Internal = serde_json::from_value(value)?;
        if !internal.pod_c_i_d_r.is_empty() {
            if internal.pod_c_i_d_rs.is_empty()
                || internal
                    .pod_c_i_d_rs
                    .first()
                    .map(|cidr| cidr.as_str() != internal.pod_c_i_d_r.as_str())
                    .unwrap_or(true)
            {
                internal.pod_c_i_d_rs = vec![internal.pod_c_i_d_r.clone()];
            }
        } else if !internal.pod_c_i_d_rs.is_empty() {
            internal.pod_c_i_d_r = internal.pod_c_i_d_rs[0].clone();
        }
        Ok(internal)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut external: Self = serde_json::from_value(serde_json::to_value(internal)?)?;
        if !external.pod_c_i_d_rs.is_empty() {
            external.pod_c_i_d_r = external.pod_c_i_d_rs[0].clone();
        }
        Ok(external)
    }
}

impl InternalConversion for NodeStatus {
    type Internal = crate::core::internal::NodeStatus;
}

impl InternalConversion for NodeCondition {
    type Internal = crate::core::internal::NodeCondition;
}

impl InternalConversion for NodeAddress {
    type Internal = crate::core::internal::NodeAddress;
}

impl InternalConversion for NodeDaemonEndpoints {
    type Internal = crate::core::internal::NodeDaemonEndpoints;
}

impl InternalConversion for DaemonEndpoint {
    type Internal = crate::core::internal::DaemonEndpoint;
}

impl InternalConversion for NodeRuntimeHandlerFeatures {
    type Internal = crate::core::internal::NodeRuntimeHandlerFeatures;
}

impl InternalConversion for NodeRuntimeHandler {
    type Internal = crate::core::internal::NodeRuntimeHandler;
}

impl InternalConversion for NodeFeatures {
    type Internal = crate::core::internal::NodeFeatures;
}

impl InternalConversion for NodeSwapStatus {
    type Internal = crate::core::internal::NodeSwapStatus;
}

impl InternalConversion for NodeSystemInfo {
    type Internal = crate::core::internal::NodeSystemInfo;
}

impl InternalConversion for ContainerImage {
    type Internal = crate::core::internal::ContainerImage;
}

impl InternalConversion for AttachedVolume {
    type Internal = crate::core::internal::AttachedVolume;
}

impl InternalConversion for NodeConfigSource {
    type Internal = crate::core::internal::NodeConfigSource;
}

impl InternalConversion for ConfigMapNodeConfigSource {
    type Internal = crate::core::internal::ConfigMapNodeConfigSource;
}

impl InternalConversion for NodeConfigStatus {
    type Internal = crate::core::internal::NodeConfigStatus;
}

impl InternalConversion for AvoidPods {
    type Internal = crate::core::internal::AvoidPods;
}

impl InternalConversion for PreferAvoidPodsEntry {
    type Internal = crate::core::internal::PreferAvoidPodsEntry;
}

impl InternalConversion for PodSignature {
    type Internal = crate::core::internal::PodSignature;
}

impl InternalConversion for NodeList {
    type Internal = crate::core::internal::NodeList;
}

impl InternalConversion for ServiceAccount {
    type Internal = crate::core::internal::ServiceAccount;
}

impl InternalConversion for ServiceAccountList {
    type Internal = crate::core::internal::ServiceAccountList;
}

impl InternalConversion for PersistentVolume {
    type Internal = crate::core::internal::PersistentVolume;
}

impl InternalConversion for PersistentVolumeSource {
    type Internal = crate::core::internal::PersistentVolumeSource;
}

impl InternalConversion for PersistentVolumeSpec {
    type Internal = crate::core::internal::PersistentVolumeSpec;
}

impl InternalConversion for PersistentVolumeStatus {
    type Internal = crate::core::internal::PersistentVolumeStatus;
}

impl InternalConversion for VolumeNodeAffinity {
    type Internal = crate::core::internal::VolumeNodeAffinity;
}

impl InternalConversion for NodeSelector {
    type Internal = crate::core::internal::NodeSelector;
}

impl InternalConversion for NodeSelectorTerm {
    type Internal = crate::core::internal::NodeSelectorTerm;
}

impl InternalConversion for NodeSelectorRequirement {
    type Internal = crate::core::internal::NodeSelectorRequirement;
}

impl InternalConversion for TopologySelectorTerm {
    type Internal = crate::core::internal::TopologySelectorTerm;
}

impl InternalConversion for TopologySelectorLabelRequirement {
    type Internal = crate::core::internal::TopologySelectorLabelRequirement;
}

impl InternalConversion for NFSVolumeSource {
    type Internal = crate::core::internal::NFSVolumeSource;
}

impl InternalConversion for CSIPersistentVolumeSource {
    type Internal = crate::core::internal::CSIPersistentVolumeSource;
}

impl InternalConversion for LocalVolumeSource {
    type Internal = crate::core::internal::LocalVolumeSource;
}

impl InternalConversion for SecretReference {
    type Internal = crate::core::internal::SecretReference;
}

impl InternalConversion for PersistentVolumeClaim {
    type Internal = crate::core::internal::PersistentVolumeClaim;
}

impl InternalConversion for PersistentVolumeClaimSpec {
    type Internal = crate::core::internal::PersistentVolumeClaimSpec;
}

impl InternalConversion for VolumeResourceRequirements {
    type Internal = crate::core::internal::VolumeResourceRequirements;
}

impl InternalConversion for TypedLocalObjectReference {
    type Internal = crate::core::internal::TypedLocalObjectReference;
}

impl InternalConversion for TypedObjectReference {
    type Internal = crate::core::internal::TypedObjectReference;
}

impl InternalConversion for PersistentVolumeClaimStatus {
    type Internal = crate::core::internal::PersistentVolumeClaimStatus;
}

impl InternalConversion for PersistentVolumeClaimCondition {
    type Internal = crate::core::internal::PersistentVolumeClaimCondition;
}

impl InternalConversion for ModifyVolumeStatus {
    type Internal = crate::core::internal::ModifyVolumeStatus;
}

impl InternalConversion for PersistentVolumeList {
    type Internal = crate::core::internal::PersistentVolumeList;
}

impl InternalConversion for PersistentVolumeClaimList {
    type Internal = crate::core::internal::PersistentVolumeClaimList;
}

impl InternalConversion for ObjectReference {
    type Internal = crate::core::internal::ObjectReference;
}

impl InternalConversion for LocalObjectReference {
    type Internal = crate::core::internal::LocalObjectReference;
}

impl InternalConversion for Taint {
    type Internal = crate::core::internal::Taint;
}

impl InternalConversion for Toleration {
    type Internal = crate::core::internal::Toleration;
}

impl InternalConversion for HostAlias {
    type Internal = crate::core::internal::HostAlias;
}

impl InternalConversion for Affinity {
    type Internal = crate::core::internal::Affinity;
}

impl InternalConversion for NodeAffinity {
    type Internal = crate::core::internal::NodeAffinity;
}

impl InternalConversion for PreferredSchedulingTerm {
    type Internal = crate::core::internal::PreferredSchedulingTerm;
}

impl InternalConversion for PodAffinity {
    type Internal = crate::core::internal::PodAffinity;
}

impl InternalConversion for PodAntiAffinity {
    type Internal = crate::core::internal::PodAntiAffinity;
}

impl InternalConversion for PodAffinityTerm {
    type Internal = crate::core::internal::PodAffinityTerm;
}

impl InternalConversion for WeightedPodAffinityTerm {
    type Internal = crate::core::internal::WeightedPodAffinityTerm;
}

impl InternalConversion for TopologySpreadConstraint {
    type Internal = crate::core::internal::TopologySpreadConstraint;
}

impl InternalConversion for PodSecurityContext {
    type Internal = crate::core::internal::PodSecurityContext;
}

impl InternalConversion for SecurityContext {
    type Internal = crate::core::internal::SecurityContext;
}

impl InternalConversion for Capabilities {
    type Internal = crate::core::internal::Capabilities;
}

impl InternalConversion for SELinuxOptions {
    type Internal = crate::core::internal::SELinuxOptions;
}

impl InternalConversion for WindowsSecurityContextOptions {
    type Internal = crate::core::internal::WindowsSecurityContextOptions;
}

impl InternalConversion for Sysctl {
    type Internal = crate::core::internal::Sysctl;
}

impl InternalConversion for SeccompProfile {
    type Internal = crate::core::internal::SeccompProfile;
}

impl InternalConversion for AppArmorProfile {
    type Internal = crate::core::internal::AppArmorProfile;
}

impl InternalConversion for Probe {
    type Internal = crate::core::internal::Probe;
}

impl InternalConversion for ProbeHandler {
    type Internal = crate::core::internal::ProbeHandler;
}

impl InternalConversion for ExecAction {
    type Internal = crate::core::internal::ExecAction;
}

impl InternalConversion for HTTPGetAction {
    type Internal = crate::core::internal::HTTPGetAction;
}

impl InternalConversion for HTTPHeader {
    type Internal = crate::core::internal::HTTPHeader;
}

impl InternalConversion for TCPSocketAction {
    type Internal = crate::core::internal::TCPSocketAction;
}

impl InternalConversion for GRPCAction {
    type Internal = crate::core::internal::GRPCAction;
}

impl InternalConversion for Lifecycle {
    type Internal = crate::core::internal::Lifecycle;
}

impl InternalConversion for LifecycleHandler {
    type Internal = crate::core::internal::LifecycleHandler;
}

impl InternalConversion for SleepAction {
    type Internal = crate::core::internal::SleepAction;
}

impl InternalConversion for PodDNSConfig {
    type Internal = crate::core::internal::PodDNSConfig;
}

impl InternalConversion for PodDNSConfigOption {
    type Internal = crate::core::internal::PodDNSConfigOption;
}

impl InternalConversion for PodReadinessGate {
    type Internal = crate::core::internal::PodReadinessGate;
}

impl InternalConversion for PodOS {
    type Internal = crate::core::internal::PodOS;
}

impl InternalConversion for PodSchedulingGate {
    type Internal = crate::core::internal::PodSchedulingGate;
}

impl InternalConversion for PodResourceClaim {
    type Internal = crate::core::internal::PodResourceClaim;
}

impl InternalConversion for PodResourceClaimStatus {
    type Internal = crate::core::internal::PodResourceClaimStatus;
}

impl InternalConversion for PodExtendedResourceClaimStatus {
    type Internal = crate::core::internal::PodExtendedResourceClaimStatus;
}

impl InternalConversion for ContainerExtendedResourceRequest {
    type Internal = crate::core::internal::ContainerExtendedResourceRequest;
}

impl InternalConversion for GCEPersistentDiskVolumeSource {
    type Internal = crate::core::internal::GCEPersistentDiskVolumeSource;
}

impl InternalConversion for AWSElasticBlockStoreVolumeSource {
    type Internal = crate::core::internal::AWSElasticBlockStoreVolumeSource;
}

impl InternalConversion for GitRepoVolumeSource {
    type Internal = crate::core::internal::GitRepoVolumeSource;
}

impl InternalConversion for ISCSIVolumeSource {
    type Internal = crate::core::internal::ISCSIVolumeSource;
}

impl InternalConversion for ISCSIPersistentVolumeSource {
    type Internal = crate::core::internal::ISCSIPersistentVolumeSource;
}

impl InternalConversion for GlusterfsVolumeSource {
    type Internal = crate::core::internal::GlusterfsVolumeSource;
}

impl InternalConversion for GlusterfsPersistentVolumeSource {
    type Internal = crate::core::internal::GlusterfsPersistentVolumeSource;
}

impl InternalConversion for RBDVolumeSource {
    type Internal = crate::core::internal::RBDVolumeSource;
}

impl InternalConversion for RBDPersistentVolumeSource {
    type Internal = crate::core::internal::RBDPersistentVolumeSource;
}

impl InternalConversion for FlexVolumeSource {
    type Internal = crate::core::internal::FlexVolumeSource;
}

impl InternalConversion for FlexPersistentVolumeSource {
    type Internal = crate::core::internal::FlexPersistentVolumeSource;
}

impl InternalConversion for CinderVolumeSource {
    type Internal = crate::core::internal::CinderVolumeSource;
}

impl InternalConversion for CinderPersistentVolumeSource {
    type Internal = crate::core::internal::CinderPersistentVolumeSource;
}

impl InternalConversion for CephFSVolumeSource {
    type Internal = crate::core::internal::CephFSVolumeSource;
}

impl InternalConversion for CephFSPersistentVolumeSource {
    type Internal = crate::core::internal::CephFSPersistentVolumeSource;
}

impl InternalConversion for FlockerVolumeSource {
    type Internal = crate::core::internal::FlockerVolumeSource;
}

impl InternalConversion for FCVolumeSource {
    type Internal = crate::core::internal::FCVolumeSource;
}

impl InternalConversion for AzureFileVolumeSource {
    type Internal = crate::core::internal::AzureFileVolumeSource;
}

impl InternalConversion for AzureFilePersistentVolumeSource {
    type Internal = crate::core::internal::AzureFilePersistentVolumeSource;
}

impl InternalConversion for VsphereVirtualDiskVolumeSource {
    type Internal = crate::core::internal::VsphereVirtualDiskVolumeSource;
}

impl InternalConversion for QuobyteVolumeSource {
    type Internal = crate::core::internal::QuobyteVolumeSource;
}

impl InternalConversion for AzureDiskVolumeSource {
    type Internal = crate::core::internal::AzureDiskVolumeSource;
}

impl InternalConversion for PhotonPersistentDiskVolumeSource {
    type Internal = crate::core::internal::PhotonPersistentDiskVolumeSource;
}

impl InternalConversion for PortworxVolumeSource {
    type Internal = crate::core::internal::PortworxVolumeSource;
}

impl InternalConversion for ScaleIOVolumeSource {
    type Internal = crate::core::internal::ScaleIOVolumeSource;
}

impl InternalConversion for ScaleIOPersistentVolumeSource {
    type Internal = crate::core::internal::ScaleIOPersistentVolumeSource;
}

impl InternalConversion for StorageOSVolumeSource {
    type Internal = crate::core::internal::StorageOSVolumeSource;
}

impl InternalConversion for StorageOSPersistentVolumeSource {
    type Internal = crate::core::internal::StorageOSPersistentVolumeSource;
}

impl InternalConversion for EphemeralVolumeSource {
    type Internal = crate::core::internal::EphemeralVolumeSource;
}

impl InternalConversion for PersistentVolumeClaimTemplate {
    type Internal = crate::core::internal::PersistentVolumeClaimTemplate;
}

impl InternalConversion for ImageVolumeSource {
    type Internal = crate::core::internal::ImageVolumeSource;
}

impl InternalConversion for Endpoints {
    type Internal = crate::core::internal::Endpoints;
}

impl InternalConversion for EndpointSubset {
    type Internal = crate::core::internal::EndpointSubset;
}

impl InternalConversion for EndpointAddress {
    type Internal = crate::core::internal::EndpointAddress;
}

impl InternalConversion for EndpointPort {
    type Internal = crate::core::internal::EndpointPort;
}

impl InternalConversion for ReplicationController {
    type Internal = crate::core::internal::ReplicationController;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let spec = match self.spec.as_ref() {
            Some(spec) => Some(spec.into_internal()?),
            None => None,
        };
        let status = match self.status.as_ref() {
            Some(status) => Some(status.into_internal()?),
            None => None,
        };
        Ok(Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            spec,
            status,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let spec = match internal.spec.as_ref() {
            Some(spec) => Some(ReplicationControllerSpec::from_internal(spec)?),
            None => None,
        };
        let status = match internal.status.as_ref() {
            Some(status) => Some(ReplicationControllerStatus::from_internal(status)?),
            None => None,
        };
        Ok(Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            spec,
            status,
        })
    }
}

impl InternalConversion for ReplicationControllerList {
    type Internal = crate::core::internal::ReplicationControllerList;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let mut items = Vec::with_capacity(self.items.len());
        for controller in &self.items {
            items.push(controller.into_internal()?);
        }
        Ok(Self::Internal {
            type_meta: self.type_meta.clone(),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let mut items = Vec::with_capacity(internal.items.len());
        for controller in &internal.items {
            items.push(ReplicationController::from_internal(controller)?);
        }
        Ok(Self {
            type_meta: internal.type_meta.clone(),
            metadata: internal.metadata.clone(),
            items,
        })
    }
}

impl InternalConversion for ReplicationControllerSpec {
    type Internal = crate::core::internal::ReplicationControllerSpec;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        let template = match self.template.as_ref() {
            Some(template) => Some(template.into_internal()?),
            None => None,
        };
        Ok(Self::Internal {
            replicas: self.replicas,
            selector: self.selector.clone(),
            template,
            min_ready_seconds: self.min_ready_seconds,
        })
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        let template = match internal.template.as_ref() {
            Some(template) => Some(PodTemplateSpec::from_internal(template)?),
            None => None,
        };
        Ok(Self {
            replicas: internal.replicas,
            selector: internal.selector.clone(),
            template,
            min_ready_seconds: internal.min_ready_seconds,
        })
    }
}

impl InternalConversion for ReplicationControllerStatus {
    type Internal = crate::core::internal::ReplicationControllerStatus;
}

impl InternalConversion for ReplicationControllerCondition {
    type Internal = crate::core::internal::ReplicationControllerCondition;
}

impl InternalConversion for LimitRange {
    type Internal = crate::core::internal::LimitRange;
}

impl InternalConversion for LimitRangeSpec {
    type Internal = crate::core::internal::LimitRangeSpec;
}

impl InternalConversion for LimitRangeItem {
    type Internal = crate::core::internal::LimitRangeItem;
}

impl InternalConversion for ResourceQuota {
    type Internal = crate::core::internal::ResourceQuota;
}

impl InternalConversion for ResourceQuotaSpec {
    type Internal = crate::core::internal::ResourceQuotaSpec;
}

impl InternalConversion for ResourceQuotaStatus {
    type Internal = crate::core::internal::ResourceQuotaStatus;
}

impl InternalConversion for ScopeSelector {
    type Internal = crate::core::internal::ScopeSelector;
}

impl InternalConversion for ScopedResourceSelectorRequirement {
    type Internal = crate::core::internal::ScopedResourceSelectorRequirement;
}

impl InternalConversion for ResourceQuotaList {
    type Internal = crate::core::internal::ResourceQuotaList;
}

impl InternalConversion for LimitRangeList {
    type Internal = crate::core::internal::LimitRangeList;
}

impl InternalConversion for EndpointsList {
    type Internal = crate::core::internal::EndpointsList;
}

impl InternalConversion for Event {
    type Internal = crate::core::internal::Event;
}

impl InternalConversion for EventList {
    type Internal = crate::core::internal::EventList;
}

impl InternalConversion for EventSource {
    type Internal = crate::core::internal::EventSource;
}

impl InternalConversion for EventSeries {
    type Internal = crate::core::internal::EventSeries;
}

impl InternalConversion for Binding {
    type Internal = crate::core::internal::Binding;
}

impl InternalConversion for Preconditions {
    type Internal = crate::core::internal::Preconditions;
}

impl InternalConversion for PodStatusResult {
    type Internal = crate::core::internal::PodStatusResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_podspec_service_account_alias() {
        let internal = crate::core::internal::PodSpec {
            service_account_name: "from-internal".to_string(),
            ..Default::default()
        };
        let external = PodSpec::from_internal(&internal).expect("podspec from internal");
        assert_eq!(external.service_account_name, "from-internal");
        assert_eq!(external.service_account, "from-internal");
    }

    #[test]
    fn test_podspec_service_account_alias_into_internal() {
        let mut spec = PodSpec::default();
        spec.service_account = "legacy".to_string();
        let internal = spec.into_internal().expect("podspec into internal");
        assert_eq!(internal.service_account_name, "legacy");
        assert_eq!(internal.service_account, "legacy");
    }

    #[test]
    fn test_podspec_service_account_name_wins_into_internal() {
        let mut spec = PodSpec::default();
        spec.service_account_name = "new".to_string();
        spec.service_account = "legacy".to_string();
        let internal = spec.into_internal().expect("podspec into internal");
        assert_eq!(internal.service_account_name, "new");
        assert_eq!(internal.service_account, "new");
    }

    #[test]
    fn test_podstatus_podip_alias() {
        let internal = crate::core::internal::PodStatus {
            pod_i_ps: vec![
                crate::core::internal::PodIP {
                    ip: "1.1.1.1".to_string(),
                },
                crate::core::internal::PodIP {
                    ip: "2.2.2.2".to_string(),
                },
            ],
            ..Default::default()
        };
        let external = PodStatus::from_internal(&internal).expect("podstatus from internal");
        assert_eq!(external.pod_i_p, "1.1.1.1");
        assert_eq!(external.pod_i_ps[0].ip, "1.1.1.1");
        assert_eq!(external.pod_i_ps[1].ip, "2.2.2.2");
    }

    #[test]
    fn test_podstatus_podip_alias_into_internal() {
        let status = PodStatus {
            pod_i_p: "10.1.2.3".to_string(),
            ..Default::default()
        };
        let internal = status.into_internal().expect("podstatus into internal");
        assert_eq!(internal.pod_i_p, "10.1.2.3");
        assert_eq!(internal.pod_i_ps.len(), 1);
        assert_eq!(internal.pod_i_ps[0].ip, "10.1.2.3");
    }

    #[test]
    fn test_podstatus_podip_mismatch_into_internal() {
        let status = PodStatus {
            pod_i_p: "10.1.2.3".to_string(),
            pod_i_ps: vec![
                PodIP {
                    ip: "10.9.9.9".to_string(),
                },
                PodIP {
                    ip: "10.1.2.4".to_string(),
                },
            ],
            ..Default::default()
        };
        let internal = status.into_internal().expect("podstatus into internal");
        assert_eq!(internal.pod_i_p, "10.1.2.3");
        assert_eq!(internal.pod_i_ps.len(), 1);
        assert_eq!(internal.pod_i_ps[0].ip, "10.1.2.3");
    }

    #[test]
    fn test_podstatus_podip_match_into_internal() {
        let status = PodStatus {
            pod_i_p: "10.1.2.3".to_string(),
            pod_i_ps: vec![
                PodIP {
                    ip: "10.1.2.3".to_string(),
                },
                PodIP {
                    ip: "10.1.2.4".to_string(),
                },
            ],
            ..Default::default()
        };
        let internal = status.into_internal().expect("podstatus into internal");
        assert_eq!(internal.pod_i_p, "10.1.2.3");
        assert_eq!(internal.pod_i_ps.len(), 2);
        assert_eq!(internal.pod_i_ps[0].ip, "10.1.2.3");
        assert_eq!(internal.pod_i_ps[1].ip, "10.1.2.4");
    }

    #[test]
    fn test_podstatus_podips_alias_into_internal() {
        let status = PodStatus {
            pod_i_ps: vec![
                PodIP {
                    ip: "10.1.2.3".to_string(),
                },
                PodIP {
                    ip: "10.1.2.4".to_string(),
                },
            ],
            ..Default::default()
        };
        let internal = status.into_internal().expect("podstatus into internal");
        assert_eq!(internal.pod_i_p, "10.1.2.3");
        assert_eq!(internal.pod_i_ps.len(), 2);
    }

    #[test]
    fn test_podstatus_empty_into_internal() {
        let status = PodStatus::default();
        let internal = status.into_internal().expect("podstatus into internal");
        assert!(internal.pod_i_p.is_empty());
        assert!(internal.pod_i_ps.is_empty());
    }

    #[test]
    fn test_nodespec_podcidr_alias() {
        let internal = crate::core::internal::NodeSpec {
            pod_c_i_d_rs: vec!["10.0.0.0/24".to_string(), "10.0.1.0/24".to_string()],
            ..Default::default()
        };
        let external = NodeSpec::from_internal(&internal).expect("nodespec from internal");
        assert_eq!(external.pod_c_i_d_r, "10.0.0.0/24");
        assert_eq!(external.pod_c_i_d_rs.len(), 2);
    }

    #[test]
    fn test_nodespec_podcidr_alias_into_internal() {
        let spec = NodeSpec {
            pod_c_i_d_r: "10.0.0.0/24".to_string(),
            ..Default::default()
        };
        let internal = spec.into_internal().expect("nodespec into internal");
        assert_eq!(internal.pod_c_i_d_r, "10.0.0.0/24");
        assert_eq!(internal.pod_c_i_d_rs, vec!["10.0.0.0/24".to_string()]);
    }

    #[test]
    fn test_nodespec_podcidrs_alias_into_internal() {
        let spec = NodeSpec {
            pod_c_i_d_rs: vec![
                "10.0.0.0/24".to_string(),
                "10.0.1.0/24".to_string(),
            ],
            ..Default::default()
        };
        let internal = spec.into_internal().expect("nodespec into internal");
        assert_eq!(internal.pod_c_i_d_r, "10.0.0.0/24");
        assert_eq!(internal.pod_c_i_d_rs.len(), 2);
    }

    #[test]
    fn test_pod_template_spec_drops_init_container_annotations() {
        let mut spec = PodTemplateSpec::default();
        spec.metadata.annotations.insert(
            "pod.beta.kubernetes.io/init-containers".to_string(),
            "legacy".to_string(),
        );
        spec.metadata
            .annotations
            .insert("keep".to_string(), "value".to_string());
        let internal = spec
            .into_internal()
            .expect("pod template spec into internal");
        assert!(!internal
            .metadata
            .annotations
            .contains_key("pod.beta.kubernetes.io/init-containers"));
        assert_eq!(
            internal.metadata.annotations.get("keep").map(String::as_str),
            Some("value")
        );
    }

    #[test]
    fn test_pod_drops_init_container_annotations() {
        let mut pod = Pod::default();
        pod.metadata.annotations.insert(
            "pod.alpha.kubernetes.io/init-container-statuses".to_string(),
            "legacy".to_string(),
        );
        pod.metadata
            .annotations
            .insert("keep".to_string(), "value".to_string());
        let internal = pod.into_internal().expect("pod into internal");
        assert!(!internal
            .metadata
            .annotations
            .contains_key("pod.alpha.kubernetes.io/init-container-statuses"));
        assert_eq!(
            internal.metadata.annotations.get("keep").map(String::as_str),
            Some("value")
        );
    }

    #[test]
    fn test_pod_clamps_negative_termination_grace_period() {
        let pod = Pod {
            spec: Some(PodSpec {
                termination_grace_period_seconds: Some(-5),
                ..Default::default()
            }),
            ..Default::default()
        };
        let internal = pod.into_internal().expect("pod into internal");
        let spec = internal.spec.expect("spec should exist");
        assert_eq!(spec.termination_grace_period_seconds, Some(1));
    }

    #[test]
    fn test_secret_string_data_overwrites_data() {
        let mut secret = Secret::default();
        secret
            .data
            .insert("key".to_string(), b"old".to_vec());
        secret
            .string_data
            .insert("key".to_string(), "new".to_string());
        secret
            .string_data
            .insert("other".to_string(), "value".to_string());
        let internal = secret.into_internal().expect("secret into internal");
        let key_value = internal.data.get("key").expect("key should exist");
        assert_eq!(std::str::from_utf8(key_value).unwrap(), "new");
        let other_value = internal.data.get("other").expect("other should exist");
        assert_eq!(std::str::from_utf8(other_value).unwrap(), "value");
        assert!(internal.string_data.is_empty());
    }
}

impl InternalConversion for PodLogOptions {
    type Internal = crate::core::internal::PodLogOptions;
}

impl InternalConversion for PodAttachOptions {
    type Internal = crate::core::internal::PodAttachOptions;
}

impl InternalConversion for PodExecOptions {
    type Internal = crate::core::internal::PodExecOptions;
}

impl InternalConversion for PodPortForwardOptions {
    type Internal = crate::core::internal::PodPortForwardOptions;
}

impl InternalConversion for PodProxyOptions {
    type Internal = crate::core::internal::PodProxyOptions;
}

impl InternalConversion for NodeProxyOptions {
    type Internal = crate::core::internal::NodeProxyOptions;
}

impl InternalConversion for ServiceProxyOptions {
    type Internal = crate::core::internal::ServiceProxyOptions;
}

impl InternalConversion for SerializedReference {
    type Internal = crate::core::internal::SerializedReference;
}

impl InternalConversion for RangeAllocation {
    type Internal = crate::core::internal::RangeAllocation;
}

impl InternalConversion for List {
    type Internal = crate::core::internal::List;
}

impl InternalConversion for ComponentStatus {
    type Internal = crate::core::internal::ComponentStatus;
}

impl InternalConversion for ComponentStatusList {
    type Internal = crate::core::internal::ComponentStatusList;
}

impl InternalConversion for ComponentCondition {
    type Internal = crate::core::internal::ComponentCondition;
}
