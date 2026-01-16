//! Internal type definitions for admissionregistration.

use k8s_apimachinery::apis::meta::v1::{Condition, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type FailurePolicyType = String;
pub type MatchPolicyType = String;
pub type OperationType = String;
pub type ParameterNotFoundActionType = String;
pub type PatchType = String;
pub type ReinvocationPolicyType = String;
pub type ScopeType = String;
pub type SideEffectClass = String;
pub type ValidatingAdmissionPolicyConditionType = String;
pub type ValidationAction = String;

pub const FAILURE_POLICY_FAIL: &str = "Fail";
pub const FAILURE_POLICY_IGNORE: &str = "Ignore";
pub const MATCH_POLICY_EQUIVALENT: &str = "Equivalent";
pub const MATCH_POLICY_EXACT: &str = "Exact";
pub const OPERATION_ALL: &str = "*";
pub const OPERATION_CONNECT: &str = "CONNECT";
pub const OPERATION_CREATE: &str = "CREATE";
pub const OPERATION_DELETE: &str = "DELETE";
pub const OPERATION_UPDATE: &str = "UPDATE";
pub const PARAMETER_NOT_FOUND_ACTION_ALLOW: &str = "Allow";
pub const PARAMETER_NOT_FOUND_ACTION_DENY: &str = "Deny";
pub const PATCH_TYPE_APPLY_CONFIGURATION: &str = "ApplyConfiguration";
pub const PATCH_TYPE_JSON_PATCH: &str = "JSONPatch";
pub const REINVOCATION_POLICY_IF_NEEDED: &str = "IfNeeded";
pub const REINVOCATION_POLICY_NEVER: &str = "Never";
pub const SCOPE_ALL: &str = "*";
pub const SCOPE_CLUSTER: &str = "Cluster";
pub const SCOPE_NAMESPACED: &str = "Namespaced";
pub const SIDE_EFFECT_CLASS_NONE: &str = "None";
pub const SIDE_EFFECT_CLASS_NONE_ON_DRY_RUN: &str = "NoneOnDryRun";
pub const SIDE_EFFECT_CLASS_SOME: &str = "Some";
pub const SIDE_EFFECT_CLASS_UNKNOWN: &str = "Unknown";
pub const VALIDATION_ACTION_AUDIT: &str = "Audit";
pub const VALIDATION_ACTION_DENY: &str = "Deny";
pub const VALIDATION_ACTION_WARN: &str = "Warn";


/// ApplyConfiguration defines the desired configuration values of an object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfiguration {
    /// Expression evaluates to an apply configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}


/// AuditAnnotation describes how to produce an audit annotation for an API request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAnnotation {
    /// Key specifies the audit annotation key.
    pub key: String,
    /// ValueExpression is evaluated to produce the audit annotation value.
    pub value_expression: String,
}


/// ExpressionWarning is a warning information that targets a specific expression.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionWarning {
    /// The path to the field that refers the expression.
    pub field_ref: String,
    /// The content of type checking information in a human-readable form.
    pub warning: String,
}


/// JSONPatch defines a JSON patch.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONPatch {
    /// Expression evaluates to a JSON patch array.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
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


/// MatchResources decides whether to run the admission control policy on an object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResources {
    /// NamespaceSelector decides whether to run the policy based on namespace labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the policy based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// ResourceRules describes what operations on what resources/subresources the policy matches.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<NamedRuleWithOperations>,
    /// ExcludeResourceRules describes what operations on what resources/subresources the policy should not care about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_resource_rules: Vec<NamedRuleWithOperations>,
    /// MatchPolicy defines how the MatchResources list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
}


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
    /// MatchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    pub side_effects: SideEffectClass,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    pub admission_review_versions: Vec<String>,
    /// ReinvocationPolicy indicates whether this webhook should be called multiple times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: Option<ReinvocationPolicyType>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}


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
    pub metadata: ListMeta,
    pub items: Vec<MutatingWebhookConfiguration>,
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


/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRuleWithOperations {
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
    /// RuleWithOperations is a tuple of Operations and Resources.
    #[serde(flatten)]
    pub rule_with_operations: RuleWithOperations,
}


/// ParamKind is a tuple of Group Kind and Version.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamKind {
    /// APIVersion is the API group version the resources belong to.
    pub api_version: String,
    /// Kind is the API kind the resources belong to.
    pub kind: String,
}


/// ParamRef describes how to locate the params to be used as input to expressions of rules.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
    /// ParameterNotFoundAction controls the behavior when no params are matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: Option<ParameterNotFoundActionType>,
}


/// Rule is a tuple of APIGroups, APIVersion, and Resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
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


/// RuleWithOperations is a tuple of Operations and Resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleWithOperations {
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


/// TypeChecking contains results of type checking the expressions in the policy.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeChecking {
    /// The type checking warnings for each expression.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_warnings: Vec<ExpressionWarning>,
}


/// ValidatingAdmissionPolicy describes the definition of an admission validation policy.
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


/// ValidatingAdmissionPolicyBinding binds the ValidatingAdmissionPolicy with parameterized resources.
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


/// ValidatingAdmissionPolicyBindingSpec is the specification of the binding.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// PolicyName references a ValidatingAdmissionPolicy name.
    pub policy_name: String,
    /// ParamRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,
    /// MatchResources declares what resources match this binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,
    /// ValidationActions declares how validations are enforced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_actions: Vec<ValidationAction>,
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


/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,
    /// MatchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,
    /// Validations contain CEL expressions which is used to apply the validation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<Validation>,
    /// FailurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,
    /// AuditAnnotations contains CEL expressions for audit annotations.
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
    /// The generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The results of type checking for each expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_checking: Option<TypeChecking>,
    /// The conditions represent the latest available observations of a policy's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
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
    /// MatchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
    /// NamespaceSelector decides whether to run the webhook on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,
    /// SideEffects states whether this webhook has side effects.
    pub side_effects: SideEffectClass,
    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    pub admission_review_versions: Vec<String>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}


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
    pub metadata: ListMeta,
    pub items: Vec<ValidatingWebhookConfiguration>,
}


/// Validation specifies the CEL expression which is used to apply the validation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    /// Expression represents the expression which will be evaluated by CEL.
    pub expression: String,
    /// Message represents the message displayed when validation fails.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Reason represents a machine-readable description of why this validation failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// MessageExpression declares a CEL expression that evaluates to the validation failure message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message_expression: String,
}


/// Variable is the definition of a variable used for composition.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// Name is the name of the variable.
    pub name: String,
    /// Expression is the expression that will be evaluated as the value of the variable.
    pub expression: String,
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
    /// CABundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ca_bundle: String,
}
