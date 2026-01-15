//! Admission Registration v1beta1 API type definitions

use k8s_apimachinery::apis::meta::v1::{Condition, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type Rule = crate::admissionregistration::v1::Rule;
pub type ScopeType = crate::admissionregistration::v1::ScopeType;
pub type FailurePolicyType = String;
pub type ParameterNotFoundActionType = String;
pub type MatchPolicyType = String;
pub type SideEffectClass = String;
pub type ValidatingAdmissionPolicyConditionType = String;
pub type ValidationAction = String;
pub type ReinvocationPolicyType = crate::admissionregistration::v1::ReinvocationPolicyType;
pub type RuleWithOperations = crate::admissionregistration::v1::RuleWithOperations;
pub type OperationType = crate::admissionregistration::v1::OperationType;
pub type PatchType = String;

// ScopeType constants
pub const SCOPE_CLUSTER: &str = "Cluster";
pub const SCOPE_NAMESPACED: &str = "Namespaced";
pub const SCOPE_ALL: &str = "*";

// FailurePolicyType constants
pub const FAILURE_POLICY_IGNORE: &str = "Ignore";
pub const FAILURE_POLICY_FAIL: &str = "Fail";

// ParameterNotFoundActionType constants
pub const PARAMETER_NOT_FOUND_ACTION_ALLOW: &str = "Allow";
pub const PARAMETER_NOT_FOUND_ACTION_DENY: &str = "Deny";

// MatchPolicyType constants
pub const MATCH_POLICY_EXACT: &str = "Exact";
pub const MATCH_POLICY_EQUIVALENT: &str = "Equivalent";

// SideEffectClass constants
pub const SIDE_EFFECT_CLASS_UNKNOWN: &str = "Unknown";
pub const SIDE_EFFECT_CLASS_NONE: &str = "None";
pub const SIDE_EFFECT_CLASS_SOME: &str = "Some";
pub const SIDE_EFFECT_CLASS_NONE_ON_DRY_RUN: &str = "NoneOnDryRun";

// ValidationAction constants
pub const VALIDATION_ACTION_DENY: &str = "Deny";
pub const VALIDATION_ACTION_WARN: &str = "Warn";
pub const VALIDATION_ACTION_AUDIT: &str = "Audit";

// PatchType constants
pub const PATCH_TYPE_APPLY_CONFIGURATION: &str = "ApplyConfiguration";
pub const PATCH_TYPE_JSON_PATCH: &str = "JSONPatch";

// =============================================================================
// ValidatingAdmissionPolicy
// =============================================================================

/// ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ValidatingAdmissionPolicySpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ValidatingAdmissionPolicyStatus>,
}

/// ValidatingAdmissionPolicyList is a list of ValidatingAdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ValidatingAdmissionPolicy>,
}

/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,
    /// MatchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,
    /// Validations contain CEL expressions which are used to apply the validation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<Validation>,
    /// FailurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,
    /// AuditAnnotations contains CEL expressions which are used to produce audit annotations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audit_annotations: Vec<AuditAnnotation>,
    /// MatchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
    /// Variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,
}

/// ValidatingAdmissionPolicyStatus represents the status of an admission validation policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyStatus {
    /// ObservedGeneration is the generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_checking: Option<TypeChecking>,
    /// Conditions represent the latest available observations of a policy's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

/// ParamKind is a tuple of Group Kind and Version.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamKind {
    /// APIVersion is the API group version the resources belong to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Kind is the API kind the resources belong to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

/// MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResources {
    /// NamespaceSelector decides whether to run the admission control policy on an object based on whether the namespace for that object matches the selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the validation based on if the object has matching labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<NamedRuleWithOperations>,
    /// ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_resource_rules: Vec<NamedRuleWithOperations>,
    /// MatchPolicy defines how the "MatchResources" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
}

/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRuleWithOperations {
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
    /// Operations is the operations the admission hook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<OperationType>,
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
    pub scope: Option<ScopeType>,
}

/// Validation specifies the CEL expression which is used to apply the validation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    /// Expression represents the expression which will be evaluated by CEL.
    pub expression: String,
    /// Message represents the message displayed when validation fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Reason represents a machine-readable description of why this validation failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// MessageExpression declares a CEL expression that evaluates to the validation failure message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_expression: Option<String>,
}

/// AuditAnnotation describes how to produce an audit annotation for an API request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAnnotation {
    /// Key specifies the audit annotation key.
    pub key: String,
    /// ValueExpression represents the expression which is evaluated by CEL to produce an audit annotation value.
    pub value_expression: String,
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

/// Variable is the definition of a variable that is used for composition.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// Name is the name of the variable.
    pub name: String,
    /// Expression is the expression that will be evaluated as the value of the variable.
    pub expression: String,
}

/// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeChecking {
    /// ExpressionWarnings is a list of warnings from type checking the policy's expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_warnings: Vec<ExpressionWarning>,
}

/// ExpressionWarning is a warning information that targets a specific expression.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionWarning {
    /// FieldRef refers to the field that caused the warning.
    pub field_ref: String,
    /// Warning contains the warning message.
    pub warning: String,
}

// =============================================================================
// ValidatingAdmissionPolicyBinding
// =============================================================================

/// ValidatingAdmissionPolicyBinding binds the ValidatingAdmissionPolicy with paramerized resources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ValidatingAdmissionPolicyBindingSpec>,
}

/// ValidatingAdmissionPolicyBindingList is a list of ValidatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ValidatingAdmissionPolicyBinding>,
}

/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub policy_name: String,
    /// ParamRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,
    /// MatchResources declares what resources match this binding and will be validated by it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,
    /// ValidationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_actions: Vec<ValidationAction>,
}

/// ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamRef {
    /// Name is the name of the resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace is the namespace of the referenced resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Selector can be used to match multiple param objects based on their labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// ParameterNotFoundAction controls the behavior of the binding when the resource exists, but cannot be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: Option<ParameterNotFoundActionType>,
}

// =============================================================================
// WebhookConfiguration
// =============================================================================

/// ValidatingWebhookConfiguration describes the configuration of validating admission webhooks.
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
    pub metadata: ListMeta,
    pub items: Vec<ValidatingWebhookConfiguration>,
}

/// MutatingWebhookConfiguration describes the configuration of mutating admission webhooks.
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
    pub metadata: ListMeta,
    pub items: Vec<MutatingWebhookConfiguration>,
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
    pub failure_policy: Option<FailurePolicyType>,
    /// MatchPolicy defines how the rules list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<SideEffectClass>,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admission_review_versions: Vec<String>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
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
    pub failure_policy: Option<FailurePolicyType>,
    /// MatchPolicy defines how the rules list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<SideEffectClass>,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admission_review_versions: Vec<String>,
    /// ReinvocationPolicy indicates whether this webhook should be called multiple times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: Option<ReinvocationPolicyType>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}

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
    /// CABundle is a PEM encoded CA bundle.
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

// =============================================================================
// MutatingAdmissionPolicy
// =============================================================================

/// MutatingAdmissionPolicy describes the definition of an admission mutation policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MutatingAdmissionPolicySpec>,
}

/// MutatingAdmissionPolicyList is a list of MutatingAdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<MutatingAdmissionPolicy>,
}

/// MutatingAdmissionPolicySpec is the specification of the desired behavior of the admission policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,
    /// MatchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,
    /// Variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,
    /// Mutations contain operations to perform on matching objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<Mutation>,
    /// FailurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,
    /// MatchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
    /// ReinvocationPolicy indicates whether mutations may be called multiple times.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reinvocation_policy: ReinvocationPolicyType,
}

/// Mutation specifies the CEL expression which is used to apply the mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    /// PatchType indicates the patch strategy used.
    pub patch_type: PatchType,
    /// ApplyConfiguration defines the desired configuration values of an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_configuration: Option<ApplyConfiguration>,
    /// JSONPatch defines a JSON patch operation to perform a mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_patch: Option<JSONPatch>,
}

/// ApplyConfiguration defines the desired configuration values of an object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfiguration {
    /// Expression evaluates to an apply configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// JSONPatch defines a JSON patch.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONPatch {
    /// Expression evaluates to a JSON patch array.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// MutatingAdmissionPolicyBinding binds the MutatingAdmissionPolicy with parametrized resources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MutatingAdmissionPolicyBindingSpec>,
}

/// MutatingAdmissionPolicyBindingList is a list of MutatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<MutatingAdmissionPolicyBinding>,
}

/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingSpec {
    /// PolicyName references a MutatingAdmissionPolicy name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub policy_name: String,
    /// ParamRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,
    /// MatchResources limits what resources match this binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,
}
