//! Flow Control v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

pub type FlowDistinguisherMethodType = String;
pub type SubjectKind = String;
pub type FlowSchemaConditionType = String;
pub type PriorityLevelConfigurationConditionType = String;
pub type PriorityLevelEnablement = String;
pub type LimitResponseType = String;
pub type ConditionStatus = String;

// Wildcard constants
pub const API_GROUP_ALL: &str = "*";
pub const RESOURCE_ALL: &str = "*";
pub const VERB_ALL: &str = "*";
pub const NON_RESOURCE_ALL: &str = "*";
pub const NAME_ALL: &str = "*";
pub const NAMESPACE_EVERY: &str = "*";

// System preset priority level names
pub const PRIORITY_LEVEL_CONFIGURATION_NAME_EXEMPT: &str = "exempt";
pub const PRIORITY_LEVEL_CONFIGURATION_NAME_CATCH_ALL: &str = "catch-all";
pub const FLOW_SCHEMA_NAME_EXEMPT: &str = "exempt";
pub const FLOW_SCHEMA_NAME_CATCH_ALL: &str = "catch-all";

// Condition types
pub const FLOW_SCHEMA_CONDITION_DANGLING: &str = "Dangling";
pub const PRIORITY_LEVEL_CONFIGURATION_CONDITION_CONCURRENCY_SHARED: &str = "ConcurrencyShared";

// API validation constants
pub const FLOW_SCHEMA_MAX_MATCHING_PRECEDENCE: i32 = 10000;

// Response headers
pub const RESPONSE_HEADER_MATCHED_PRIORITY_LEVEL_CONFIGURATION_UID: &str = "X-Kubernetes-PF-PriorityLevel-UID";
pub const RESPONSE_HEADER_MATCHED_FLOW_SCHEMA_UID: &str = "X-Kubernetes-PF-FlowSchema-UID";

// Annotation keys
pub const AUTO_UPDATE_ANNOTATION_KEY: &str = "apf.kubernetes.io/autoupdate-spec";

// Flow distinguisher method type constants
pub const FLOW_DISTINGUISHER_METHOD_BY_USER: &str = "ByUser";
pub const FLOW_DISTINGUISHER_METHOD_BY_NAMESPACE: &str = "ByNamespace";

// Subject kind constants
pub const SUBJECT_KIND_USER: &str = "User";
pub const SUBJECT_KIND_GROUP: &str = "Group";
pub const SUBJECT_KIND_SERVICE_ACCOUNT: &str = "ServiceAccount";

// Priority level enablement constants
pub const PRIORITY_LEVEL_ENABLEMENT_EXEMPT: &str = "Exempt";
pub const PRIORITY_LEVEL_ENABLEMENT_LIMITED: &str = "Limited";

// Limit response type constants
pub const LIMIT_RESPONSE_TYPE_QUEUE: &str = "Queue";
pub const LIMIT_RESPONSE_TYPE_REJECT: &str = "Reject";

// Condition status constants
pub const CONDITION_TRUE: &str = "True";
pub const CONDITION_FALSE: &str = "False";
pub const CONDITION_UNKNOWN: &str = "Unknown";

// =============================================================================
// FlowSchema
// =============================================================================

/// FlowSchema defines the schema of a group of flows.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchema {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<FlowSchemaSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FlowSchemaStatus>,
}

/// FlowSchemaList is a list of FlowSchema objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<FlowSchema>,
}

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaSpec {
    /// PriorityLevelConfiguration should be used to process requests.
    pub priority_level_configuration: PriorityLevelConfigurationReference,
    /// MatchingPrecedence is used to choose among the FlowSchemas that match a given request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_precedence: Option<i32>,
    /// DistinguisherMethod defines how to compute the flow distinguisher for requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinguisher_method: Option<FlowDistinguisherMethod>,
    /// Rules describes which requests will match this flow schema.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRulesWithSubjects>,
}

/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<FlowSchemaCondition>,
}

/// FlowSchemaCondition describes conditions for a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaCondition {
    /// type is the type of the condition.
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: FlowSchemaConditionType,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: ConditionStatus,
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    /// reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// message is a human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// PriorityLevelConfigurationReference contains information that points to the priority level configuration being used.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationReference {
    pub name: String,
}

/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDistinguisherMethod {
    #[serde(rename = "type")]
    pub type_: FlowDistinguisherMethodType,
}

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRulesWithSubjects {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<ResourcePolicyRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_rules: Vec<NonResourcePolicyRule>,
}

/// Subject matches the originator of a request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub kind: SubjectKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSubject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupSubject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ServiceAccountSubject>,
}

/// UserSubject holds detailed information for user-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubject {
    pub name: String,
}

/// GroupSubject holds detailed information for group-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSubject {
    pub name: String,
}

/// ServiceAccountSubject holds detailed information for service-account-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountSubject {
    pub namespace: String,
    pub name: String,
}

/// ResourcePolicyRule is a predicate that matches some resource requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePolicyRule {
    pub verbs: Vec<String>,
    pub api_groups: Vec<String>,
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
}

/// NonResourcePolicyRule is a predicate that matches non-resource requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonResourcePolicyRule {
    pub verbs: Vec<String>,
    #[serde(rename = "nonResourceURLs")]
    pub non_resource_urls: Vec<String>,
}

// =============================================================================
// PriorityLevelConfiguration
// =============================================================================

/// PriorityLevelConfiguration represents the configuration of a priority level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfiguration {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PriorityLevelConfigurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PriorityLevelConfigurationStatus>,
}

/// PriorityLevelConfigurationList is a list of PriorityLevelConfiguration objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<PriorityLevelConfiguration>,
}

/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationSpec {
    #[serde(rename = "type")]
    pub type_: PriorityLevelEnablement,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limited: Option<LimitedPriorityLevelConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt: Option<ExemptPriorityLevelConfiguration>,
}

/// PriorityLevelConfigurationStatus represents the current state of a priority-level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PriorityLevelConfigurationCondition>,
}

/// PriorityLevelConfigurationCondition defines the condition of priority level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationCondition {
    /// type is the type of the condition.
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: PriorityLevelConfigurationConditionType,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: ConditionStatus,
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    /// reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// message is a human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedPriorityLevelConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_response: Option<LimitResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borrowing_limit_percent: Option<i32>,
}

/// ExemptPriorityLevelConfiguration describes the configurable aspects of the handling of exempt requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExemptPriorityLevelConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
}

/// LimitResponse defines how to handle requests that can not be executed right now.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitResponse {
    #[serde(rename = "type")]
    pub type_: LimitResponseType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queuing: Option<QueuingConfiguration>,
}

/// QueuingConfiguration holds the configuration parameters for queuing.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueuingConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queues: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hand_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_length_limit: Option<i32>,
}
