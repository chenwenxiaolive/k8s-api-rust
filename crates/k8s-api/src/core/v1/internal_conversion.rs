use super::*;

impl InternalConversion for Pod {
    type Internal = crate::core::internal::Pod;
}

impl InternalConversion for PodList {
    type Internal = crate::core::internal::PodList;
}

impl InternalConversion for PodSpec {
    type Internal = crate::core::internal::PodSpec;
}

impl InternalConversion for PodStatus {
    type Internal = crate::core::internal::PodStatus;
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
}

impl InternalConversion for PodTemplateSpec {
    type Internal = crate::core::internal::PodTemplateSpec;
}

impl InternalConversion for PodTemplateList {
    type Internal = crate::core::internal::PodTemplateList;
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
}

impl InternalConversion for ReplicationControllerList {
    type Internal = crate::core::internal::ReplicationControllerList;
}

impl InternalConversion for ReplicationControllerSpec {
    type Internal = crate::core::internal::ReplicationControllerSpec;
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
