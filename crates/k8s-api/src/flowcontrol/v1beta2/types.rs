//! Flow Control v1beta2 type definitions (deprecated)

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

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
    /// spec is the specification of the desired behavior of a FlowSchema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<FlowSchemaSpec>,
    /// status is the current status of a FlowSchema.
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
    pub metadata: ListMeta,
    /// items is a list of FlowSchemas.
    pub items: Vec<FlowSchema>,
}

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaSpec {
    /// priorityLevelConfiguration should reference a PriorityLevelConfiguration in the cluster.
    pub priority_level_configuration: PriorityLevelConfigurationReference,
    /// matchingPrecedence is used to choose among the FlowSchemas that match a given request.
    #[serde(default)]
    pub matching_precedence: i32,
    /// distinguisherMethod defines how to compute the flow distinguisher for requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinguisher_method: Option<FlowDistinguisherMethod>,
    /// rules describes which requests will match this flow schema.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRulesWithSubjects>,
}

/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDistinguisherMethod {
    /// type is the type of flow distinguisher method ("ByUser" or "ByNamespace").
    #[serde(rename = "type")]
    pub type_: String,
}

// Flow distinguisher method type constants
pub const FLOW_DISTINGUISHER_METHOD_BY_USER: &str = "ByUser";
pub const FLOW_DISTINGUISHER_METHOD_BY_NAMESPACE: &str = "ByNamespace";

/// PriorityLevelConfigurationReference contains information that points to the "request-priority" being used.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationReference {
    /// name is the name of the priority level configuration being referenced.
    pub name: String,
}

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRulesWithSubjects {
    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about.
    pub subjects: Vec<Subject>,
    /// resourceRules is a slice of ResourcePolicyRules that identify matching requests.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<ResourcePolicyRule>,
    /// nonResourceRules is a list of NonResourcePolicyRules that identify matching requests.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_rules: Vec<NonResourcePolicyRule>,
}

/// Subject matches the originator of a request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// kind indicates which one of the other fields is non-empty.
    pub kind: String,
    /// user matches based on username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSubject>,
    /// group matches based on user group name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupSubject>,
    /// serviceAccount matches ServiceAccounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ServiceAccountSubject>,
}

// Subject kind constants
pub const SUBJECT_KIND_USER: &str = "User";
pub const SUBJECT_KIND_GROUP: &str = "Group";
pub const SUBJECT_KIND_SERVICE_ACCOUNT: &str = "ServiceAccount";

/// UserSubject holds detailed information for user-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubject {
    /// name is the username that matches, or "*" to match all usernames.
    pub name: String,
}

/// GroupSubject holds detailed information for group-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSubject {
    /// name is the user group that matches, or "*" to match all user groups.
    pub name: String,
}

/// ServiceAccountSubject holds detailed information for service-account-kind subject.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountSubject {
    /// namespace is the namespace of matching ServiceAccount objects.
    pub namespace: String,
    /// name is the name of matching ServiceAccount objects, or "*" to match regardless of name.
    pub name: String,
}

/// ResourcePolicyRule is a predicate that matches some resource requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePolicyRule {
    /// verbs is a list of matching verbs and may not be empty.
    pub verbs: Vec<String>,
    /// apiGroups is a list of matching API groups and may not be empty.
    pub api_groups: Vec<String>,
    /// resources is a list of matching resources.
    pub resources: Vec<String>,
    /// clusterScope indicates whether to match requests that do not specify a namespace.
    #[serde(default)]
    pub cluster_scope: bool,
    /// namespaces is a list of target namespaces that restricts matches.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
}

/// NonResourcePolicyRule is a predicate that matches non-resource requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonResourcePolicyRule {
    /// verbs is a list of matching verbs and may not be empty.
    pub verbs: Vec<String>,
    /// nonResourceURLs is a set of url prefixes that a user should have access to.
    #[serde(rename = "nonResourceURLs")]
    pub non_resource_urls: Vec<String>,
}

/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaStatus {
    /// conditions is a list of the current states of FlowSchema.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<FlowSchemaCondition>,
}

/// FlowSchemaCondition describes conditions for a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaCondition {
    /// type is the type of the condition.
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: String,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
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
    /// spec is the specification of the desired behavior of a "request-priority".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PriorityLevelConfigurationSpec>,
    /// status is the current status of a "request-priority".
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
    pub metadata: ListMeta,
    /// items is a list of request-priorities.
    pub items: Vec<PriorityLevelConfiguration>,
}

/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationSpec {
    /// type indicates whether this priority level is subject to limitation on request execution.
    #[serde(rename = "type")]
    pub type_: String,
    /// limited specifies how requests are handled for a Limited priority level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limited: Option<LimitedPriorityLevelConfiguration>,
    /// exempt specifies how requests are handled for an exempt priority level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt: Option<ExemptPriorityLevelConfiguration>,
}

// Priority level enablement constants
pub const PRIORITY_LEVEL_ENABLEMENT_EXEMPT: &str = "Exempt";
pub const PRIORITY_LEVEL_ENABLEMENT_LIMITED: &str = "Limited";

/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedPriorityLevelConfiguration {
    /// assuredConcurrencyShares (ACS) configures the execution limit for this priority level.
    #[serde(default)]
    pub assured_concurrency_shares: i32,
    /// limitResponse indicates what to do with requests that can not be executed right now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_response: Option<LimitResponse>,
    /// lendablePercent prescribes the fraction of the level's NominalCL that can be borrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    /// borrowingLimitPercent, if present, configures a limit on how many seats can be borrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borrowing_limit_percent: Option<i32>,
}

/// ExemptPriorityLevelConfiguration describes the configurable aspects of the handling of exempt requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExemptPriorityLevelConfiguration {
    /// nominalConcurrencyShares (NCS) contributes to the computation of the NominalConcurrencyLimit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
    /// lendablePercent prescribes the fraction of the level's NominalCL that can be borrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
}

/// LimitResponse defines how to handle requests that can not be executed right now.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitResponse {
    /// type is "Queue" or "Reject".
    #[serde(rename = "type")]
    pub type_: String,
    /// queuing holds the configuration parameters for queuing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queuing: Option<QueuingConfiguration>,
}

// Limit response type constants
pub const LIMIT_RESPONSE_TYPE_QUEUE: &str = "Queue";
pub const LIMIT_RESPONSE_TYPE_REJECT: &str = "Reject";

/// QueuingConfiguration holds the configuration parameters for queuing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueuingConfiguration {
    /// queues is the number of queues for this priority level.
    #[serde(default)]
    pub queues: i32,
    /// handSize is a small positive number that configures the shuffle sharding of requests.
    #[serde(default)]
    pub hand_size: i32,
    /// queueLengthLimit is the maximum number of requests allowed to be waiting in a given queue.
    #[serde(default)]
    pub queue_length_limit: i32,
}

/// PriorityLevelConfigurationStatus represents the current state of a "request-priority".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationStatus {
    /// conditions is the current state of "request-priority".
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PriorityLevelConfigurationCondition>,
}

/// PriorityLevelConfigurationCondition defines the condition of priority level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationCondition {
    /// type is the type of the condition.
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: String,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
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

// Condition status constants
pub const CONDITION_TRUE: &str = "True";
pub const CONDITION_FALSE: &str = "False";
pub const CONDITION_UNKNOWN: &str = "Unknown";
