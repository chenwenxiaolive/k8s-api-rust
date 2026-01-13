//! Admission Registration v1beta1 API type definitions

use k8s_apimachinery::apis::meta::v1::{Condition, LabelSelector, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
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
    pub failure_policy: Option<String>,
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
    pub match_policy: Option<String>,
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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
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
    pub validation_actions: Vec<String>,
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
    pub parameter_not_found_action: Option<String>,
}
