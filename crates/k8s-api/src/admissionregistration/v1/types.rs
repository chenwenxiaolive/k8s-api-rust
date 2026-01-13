//! Admission Registration v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// MutatingWebhookConfiguration
// =============================================================================

/// MutatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and may change the object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfiguration {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<MutatingWebhook>,
}

/// MutatingWebhookConfigurationList is a list of MutatingWebhookConfiguration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfigurationList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<MutatingWebhookConfiguration>,
}

/// MutatingWebhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhook {
    /// Name is the name of the admission webhook.
    pub name: String,
    /// ClientConfig defines how to communicate with the hook.
    pub client_config: WebhookClientConfig,
    /// Rules describes what operations on what resources/subresources the webhook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RuleWithOperations>,
    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<String>,
    /// MatchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<String>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    pub side_effects: String,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    pub admission_review_versions: Vec<String>,
    /// ReinvocationPolicy indicates whether this webhook should be called multiple times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: Option<String>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}

// =============================================================================
// ValidatingWebhookConfiguration
// =============================================================================

/// ValidatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and object without changing it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfiguration {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<ValidatingWebhook>,
}

/// ValidatingWebhookConfigurationList is a list of ValidatingWebhookConfiguration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfigurationList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ValidatingWebhookConfiguration>,
}

/// ValidatingWebhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhook {
    /// Name is the name of the admission webhook.
    pub name: String,
    /// ClientConfig defines how to communicate with the hook.
    pub client_config: WebhookClientConfig,
    /// Rules describes what operations on what resources/subresources the webhook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RuleWithOperations>,
    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<String>,
    /// MatchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<String>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    pub side_effects: String,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    pub admission_review_versions: Vec<String>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}

// =============================================================================
// Supporting Types
// =============================================================================

/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookClientConfig {
    /// URL gives the location of the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Service is a reference to the service for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,
    /// CABundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ca_bundle: String,
}

/// ServiceReference holds a reference to Service.legacy.k8s.io.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service.
    pub namespace: String,
    /// Name is the name of the service.
    pub name: String,
    /// Path is an optional URL path which will be sent in any request to this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port is an optional service port which will be used when communicating with this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// RuleWithOperations is a tuple of Operations and Resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleWithOperations {
    /// Operations is the operations the admission hook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<String>,
    /// APIGroups is the API groups the resources belong to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    /// APIVersions is the API versions the resources belong to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    /// Scope specifies the scope of this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// MatchCondition represents a condition which must be fulfilled for a request to be sent to a webhook.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchCondition {
    /// Name is an identifier for this match condition.
    pub name: String,
    /// Expression represents the expression which will be evaluated by CEL.
    pub expression: String,
}
