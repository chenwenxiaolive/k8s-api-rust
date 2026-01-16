use k8s_api_conversion::{ConversionError, Convertible};
use k8s_apimachinery::apis::meta::v1::ObjectMeta;

macro_rules! roundtrip_list {
    ($test_name:ident, $from_list:path, $to_list:path, $item:expr) => {
        #[test]
        fn $test_name() {
            let item = $item;
            let mut from: $from_list = Default::default();
            from.items = vec![item];
            let converted: $to_list = from.convert_to().expect("convert to target");
            assert_eq!(converted.items.len(), 1);
            assert_eq!(converted.items[0].metadata.name, "demo");
            let roundtrip: $from_list =
                <$from_list as Convertible<$to_list>>::convert_from(&converted)
                    .expect("convert from target");
            assert_eq!(roundtrip.items.len(), 1);
            assert_eq!(roundtrip.items[0].metadata.name, "demo");
        }
    };
}

// admissionregistration
roundtrip_list!(
    admissionregistration_mutating_webhook_list_v1beta1,
    k8s_api::admissionregistration::v1beta1::MutatingWebhookConfigurationList,
    k8s_api::admissionregistration::v1::MutatingWebhookConfigurationList,
    k8s_api::admissionregistration::v1beta1::MutatingWebhookConfiguration {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_validating_webhook_list_v1beta1,
    k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfigurationList,
    k8s_api::admissionregistration::v1::ValidatingWebhookConfigurationList,
    k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfiguration {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_validating_admission_policy_list_v1beta1,
    k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicy {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_validating_admission_policy_list_v1alpha1,
    k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_validating_admission_policy_binding_list_v1beta1,
    k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_validating_admission_policy_binding_list_v1alpha1,
    k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_mutating_admission_policy_list_v1alpha1,
    k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyList,
    k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    admissionregistration_mutating_admission_policy_binding_list_v1alpha1,
    k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingList,
    k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// apidiscovery
roundtrip_list!(
    apidiscovery_group_list_v2beta1,
    k8s_api::apidiscovery::v2beta1::APIGroupDiscoveryList,
    k8s_api::apidiscovery::v2::APIGroupDiscoveryList,
    k8s_api::apidiscovery::v2beta1::APIGroupDiscovery {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// apps
roundtrip_list!(
    apps_controller_revision_list_v1beta1,
    k8s_api::apps::v1beta1::ControllerRevisionList,
    k8s_api::apps::v1::ControllerRevisionList,
    k8s_api::apps::v1beta1::ControllerRevision {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_controller_revision_list_v1beta2,
    k8s_api::apps::v1beta2::ControllerRevisionList,
    k8s_api::apps::v1::ControllerRevisionList,
    k8s_api::apps::v1beta2::ControllerRevision {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_deployment_list_v1beta1,
    k8s_api::apps::v1beta1::DeploymentList,
    k8s_api::apps::v1::DeploymentList,
    k8s_api::apps::v1beta1::Deployment {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_deployment_list_v1beta2,
    k8s_api::apps::v1beta2::DeploymentList,
    k8s_api::apps::v1::DeploymentList,
    k8s_api::apps::v1beta2::Deployment {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_statefulset_list_v1beta1,
    k8s_api::apps::v1beta1::StatefulSetList,
    k8s_api::apps::v1::StatefulSetList,
    k8s_api::apps::v1beta1::StatefulSet {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_statefulset_list_v1beta2,
    k8s_api::apps::v1beta2::StatefulSetList,
    k8s_api::apps::v1::StatefulSetList,
    k8s_api::apps::v1beta2::StatefulSet {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_daemonset_list_v1beta2,
    k8s_api::apps::v1beta2::DaemonSetList,
    k8s_api::apps::v1::DaemonSetList,
    k8s_api::apps::v1beta2::DaemonSet {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    apps_replicaset_list_v1beta2,
    k8s_api::apps::v1beta2::ReplicaSetList,
    k8s_api::apps::v1::ReplicaSetList,
    k8s_api::apps::v1beta2::ReplicaSet {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// autoscaling
roundtrip_list!(
    autoscaling_hpa_list_v1,
    k8s_api::autoscaling::v1::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v1::HorizontalPodAutoscaler {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    autoscaling_hpa_list_v2beta1,
    k8s_api::autoscaling::v2beta1::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v2beta1::HorizontalPodAutoscaler {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    autoscaling_hpa_list_v2beta2,
    k8s_api::autoscaling::v2beta2::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v2::HorizontalPodAutoscalerList,
    k8s_api::autoscaling::v2beta2::HorizontalPodAutoscaler {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// batch
roundtrip_list!(
    batch_cronjob_list_v1beta1,
    k8s_api::batch::v1beta1::CronJobList,
    k8s_api::batch::v1::CronJobList,
    k8s_api::batch::v1beta1::CronJob {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// certificates
roundtrip_list!(
    certificates_csr_list_v1beta1,
    k8s_api::certificates::v1beta1::CertificateSigningRequestList,
    k8s_api::certificates::v1::CertificateSigningRequestList,
    k8s_api::certificates::v1beta1::CertificateSigningRequest {
        metadata: ObjectMeta::named("demo"),
        spec: k8s_api::certificates::v1beta1::CertificateSigningRequestSpec {
            signer_name: Some("example.com/signer".to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
);
roundtrip_list!(
    certificates_cluster_trust_bundle_list_v1alpha1,
    k8s_api::certificates::v1alpha1::ClusterTrustBundleList,
    k8s_api::certificates::v1beta1::ClusterTrustBundleList,
    k8s_api::certificates::v1alpha1::ClusterTrustBundle {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// coordination
roundtrip_list!(
    coordination_lease_list_v1beta1,
    k8s_api::coordination::v1beta1::LeaseList,
    k8s_api::coordination::v1::LeaseList,
    k8s_api::coordination::v1beta1::Lease {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    coordination_lease_candidate_list_v1alpha2,
    k8s_api::coordination::v1alpha2::LeaseCandidateList,
    k8s_api::coordination::v1beta1::LeaseCandidateList,
    k8s_api::coordination::v1alpha2::LeaseCandidate {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// discovery
roundtrip_list!(
    discovery_endpoint_slice_list_v1beta1,
    k8s_api::discovery::v1beta1::EndpointSliceList,
    k8s_api::discovery::v1::EndpointSliceList,
    k8s_api::discovery::v1beta1::EndpointSlice {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// events
roundtrip_list!(
    events_event_list_v1beta1,
    k8s_api::events::v1beta1::EventList,
    k8s_api::events::v1::EventList,
    k8s_api::events::v1beta1::Event {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// flowcontrol
roundtrip_list!(
    flowcontrol_flow_schema_list_v1beta1,
    k8s_api::flowcontrol::v1beta1::FlowSchemaList,
    k8s_api::flowcontrol::v1::FlowSchemaList,
    k8s_api::flowcontrol::v1beta1::FlowSchema {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    flowcontrol_flow_schema_list_v1beta2,
    k8s_api::flowcontrol::v1beta2::FlowSchemaList,
    k8s_api::flowcontrol::v1::FlowSchemaList,
    k8s_api::flowcontrol::v1beta2::FlowSchema {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    flowcontrol_flow_schema_list_v1beta3,
    k8s_api::flowcontrol::v1beta3::FlowSchemaList,
    k8s_api::flowcontrol::v1::FlowSchemaList,
    k8s_api::flowcontrol::v1beta3::FlowSchema {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    flowcontrol_priority_level_list_v1beta1,
    k8s_api::flowcontrol::v1beta1::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1beta1::PriorityLevelConfiguration {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    flowcontrol_priority_level_list_v1beta2,
    k8s_api::flowcontrol::v1beta2::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1beta2::PriorityLevelConfiguration {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    flowcontrol_priority_level_list_v1beta3,
    k8s_api::flowcontrol::v1beta3::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1::PriorityLevelConfigurationList,
    k8s_api::flowcontrol::v1beta3::PriorityLevelConfiguration {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// networking
roundtrip_list!(
    networking_ingress_list_v1beta1,
    k8s_api::networking::v1beta1::IngressList,
    k8s_api::networking::v1::IngressList,
    k8s_api::networking::v1beta1::Ingress {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    networking_ingress_class_list_v1beta1,
    k8s_api::networking::v1beta1::IngressClassList,
    k8s_api::networking::v1::IngressClassList,
    k8s_api::networking::v1beta1::IngressClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    networking_ip_address_list_v1beta1,
    k8s_api::networking::v1beta1::IPAddressList,
    k8s_api::networking::v1::IPAddressList,
    k8s_api::networking::v1beta1::IPAddress {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    networking_service_cidr_list_v1beta1,
    k8s_api::networking::v1beta1::ServiceCIDRList,
    k8s_api::networking::v1::ServiceCIDRList,
    k8s_api::networking::v1beta1::ServiceCIDR {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// node
roundtrip_list!(
    node_runtime_class_list_v1beta1,
    k8s_api::node::v1beta1::RuntimeClassList,
    k8s_api::node::v1::RuntimeClassList,
    k8s_api::node::v1beta1::RuntimeClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    node_runtime_class_list_v1alpha1,
    k8s_api::node::v1alpha1::RuntimeClassList,
    k8s_api::node::v1::RuntimeClassList,
    k8s_api::node::v1alpha1::RuntimeClass {
        metadata: ObjectMeta::named("demo"),
        spec: k8s_api::node::v1alpha1::RuntimeClassSpec {
            runtime_handler: "demo-handler".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
);

// policy
roundtrip_list!(
    policy_pdb_list_v1beta1,
    k8s_api::policy::v1beta1::PodDisruptionBudgetList,
    k8s_api::policy::v1::PodDisruptionBudgetList,
    k8s_api::policy::v1beta1::PodDisruptionBudget {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// rbac
roundtrip_list!(
    rbac_role_list_v1beta1,
    k8s_api::rbac::v1beta1::RoleList,
    k8s_api::rbac::v1::RoleList,
    k8s_api::rbac::v1beta1::Role {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_role_list_v1alpha1,
    k8s_api::rbac::v1alpha1::RoleList,
    k8s_api::rbac::v1::RoleList,
    k8s_api::rbac::v1alpha1::Role {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_role_binding_list_v1beta1,
    k8s_api::rbac::v1beta1::RoleBindingList,
    k8s_api::rbac::v1::RoleBindingList,
    k8s_api::rbac::v1beta1::RoleBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_role_binding_list_v1alpha1,
    k8s_api::rbac::v1alpha1::RoleBindingList,
    k8s_api::rbac::v1::RoleBindingList,
    k8s_api::rbac::v1alpha1::RoleBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_cluster_role_list_v1beta1,
    k8s_api::rbac::v1beta1::ClusterRoleList,
    k8s_api::rbac::v1::ClusterRoleList,
    k8s_api::rbac::v1beta1::ClusterRole {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_cluster_role_list_v1alpha1,
    k8s_api::rbac::v1alpha1::ClusterRoleList,
    k8s_api::rbac::v1::ClusterRoleList,
    k8s_api::rbac::v1alpha1::ClusterRole {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_cluster_role_binding_list_v1beta1,
    k8s_api::rbac::v1beta1::ClusterRoleBindingList,
    k8s_api::rbac::v1::ClusterRoleBindingList,
    k8s_api::rbac::v1beta1::ClusterRoleBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    rbac_cluster_role_binding_list_v1alpha1,
    k8s_api::rbac::v1alpha1::ClusterRoleBindingList,
    k8s_api::rbac::v1::ClusterRoleBindingList,
    k8s_api::rbac::v1alpha1::ClusterRoleBinding {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// resource
roundtrip_list!(
    resource_claim_list_v1beta1,
    k8s_api::resource::v1beta1::ResourceClaimList,
    k8s_api::resource::v1::ResourceClaimList,
    k8s_api::resource::v1beta1::ResourceClaim {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_claim_template_list_v1beta1,
    k8s_api::resource::v1beta1::ResourceClaimTemplateList,
    k8s_api::resource::v1::ResourceClaimTemplateList,
    k8s_api::resource::v1beta1::ResourceClaimTemplate {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_device_class_list_v1beta1,
    k8s_api::resource::v1beta1::DeviceClassList,
    k8s_api::resource::v1::DeviceClassList,
    k8s_api::resource::v1beta1::DeviceClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_slice_list_v1beta1,
    k8s_api::resource::v1beta1::ResourceSliceList,
    k8s_api::resource::v1::ResourceSliceList,
    k8s_api::resource::v1beta1::ResourceSlice {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_claim_list_v1beta2,
    k8s_api::resource::v1beta2::ResourceClaimList,
    k8s_api::resource::v1::ResourceClaimList,
    k8s_api::resource::v1beta2::ResourceClaim {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_claim_template_list_v1beta2,
    k8s_api::resource::v1beta2::ResourceClaimTemplateList,
    k8s_api::resource::v1::ResourceClaimTemplateList,
    k8s_api::resource::v1beta2::ResourceClaimTemplate {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_device_class_list_v1beta2,
    k8s_api::resource::v1beta2::DeviceClassList,
    k8s_api::resource::v1::DeviceClassList,
    k8s_api::resource::v1beta2::DeviceClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    resource_slice_list_v1beta2,
    k8s_api::resource::v1beta2::ResourceSliceList,
    k8s_api::resource::v1::ResourceSliceList,
    k8s_api::resource::v1beta2::ResourceSlice {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// scheduling
roundtrip_list!(
    scheduling_priority_class_list_v1beta1,
    k8s_api::scheduling::v1beta1::PriorityClassList,
    k8s_api::scheduling::v1::PriorityClassList,
    k8s_api::scheduling::v1beta1::PriorityClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    scheduling_priority_class_list_v1alpha1,
    k8s_api::scheduling::v1alpha1::PriorityClassList,
    k8s_api::scheduling::v1::PriorityClassList,
    k8s_api::scheduling::v1alpha1::PriorityClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

// storage
roundtrip_list!(
    storage_class_list_v1beta1,
    k8s_api::storage::v1beta1::StorageClassList,
    k8s_api::storage::v1::StorageClassList,
    k8s_api::storage::v1beta1::StorageClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_volume_attachment_list_v1beta1,
    k8s_api::storage::v1beta1::VolumeAttachmentList,
    k8s_api::storage::v1::VolumeAttachmentList,
    k8s_api::storage::v1beta1::VolumeAttachment {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_volume_attachment_list_v1alpha1,
    k8s_api::storage::v1alpha1::VolumeAttachmentList,
    k8s_api::storage::v1::VolumeAttachmentList,
    k8s_api::storage::v1alpha1::VolumeAttachment {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_csi_driver_list_v1beta1,
    k8s_api::storage::v1beta1::CSIDriverList,
    k8s_api::storage::v1::CSIDriverList,
    k8s_api::storage::v1beta1::CSIDriver {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_csi_node_list_v1beta1,
    k8s_api::storage::v1beta1::CSINodeList,
    k8s_api::storage::v1::CSINodeList,
    k8s_api::storage::v1beta1::CSINode {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_csi_storage_capacity_list_v1beta1,
    k8s_api::storage::v1beta1::CSIStorageCapacityList,
    k8s_api::storage::v1::CSIStorageCapacityList,
    k8s_api::storage::v1beta1::CSIStorageCapacity {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_csi_storage_capacity_list_v1alpha1,
    k8s_api::storage::v1alpha1::CSIStorageCapacityList,
    k8s_api::storage::v1::CSIStorageCapacityList,
    k8s_api::storage::v1alpha1::CSIStorageCapacity {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_volume_attributes_class_list_v1beta1,
    k8s_api::storage::v1beta1::VolumeAttributesClassList,
    k8s_api::storage::v1::VolumeAttributesClassList,
    k8s_api::storage::v1beta1::VolumeAttributesClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);
roundtrip_list!(
    storage_volume_attributes_class_list_v1alpha1,
    k8s_api::storage::v1alpha1::VolumeAttributesClassList,
    k8s_api::storage::v1::VolumeAttributesClassList,
    k8s_api::storage::v1alpha1::VolumeAttributesClass {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    }
);

#[test]
fn admissionregistration_mutating_webhook_missing_side_effects() {
    let config = k8s_api::admissionregistration::v1beta1::MutatingWebhookConfiguration {
        metadata: ObjectMeta::named("demo"),
        webhooks: vec![k8s_api::admissionregistration::v1beta1::MutatingWebhook {
            name: "hook".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let err = config.convert_to().expect_err("expected missing side effects");
    assert!(matches!(
        err,
        ConversionError::MissingField(field) if field == "mutatingWebhook.sideEffects"
    ));
}

#[test]
fn admissionregistration_validating_webhook_missing_side_effects() {
    let config = k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfiguration {
        metadata: ObjectMeta::named("demo"),
        webhooks: vec![k8s_api::admissionregistration::v1beta1::ValidatingWebhook {
            name: "hook".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let err = config.convert_to().expect_err("expected missing side effects");
    assert!(matches!(
        err,
        ConversionError::MissingField(field) if field == "validatingWebhook.sideEffects"
    ));
}

#[test]
fn certificates_csr_missing_signer_name() {
    let csr = k8s_api::certificates::v1beta1::CertificateSigningRequest {
        metadata: ObjectMeta::named("demo"),
        ..Default::default()
    };

    let err = csr.convert_to().expect_err("expected missing signer name");
    assert!(matches!(
        err,
        ConversionError::MissingField(field) if field == "spec.signerName"
    ));
}
