use super::*;

impl InternalConversion for Rule {
    type Internal = crate::admissionregistration::internal::Rule;
}

impl InternalConversion for ValidatingAdmissionPolicy {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicy;
}

impl InternalConversion for ValidatingAdmissionPolicyStatus {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyStatus;
}

impl InternalConversion for TypeChecking {
    type Internal = crate::admissionregistration::internal::TypeChecking;
}

impl InternalConversion for ExpressionWarning {
    type Internal = crate::admissionregistration::internal::ExpressionWarning;
}

impl InternalConversion for ValidatingAdmissionPolicyList {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyList;
}

impl InternalConversion for ValidatingAdmissionPolicySpec {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicySpec;
}

impl InternalConversion for ParamKind {
    type Internal = crate::admissionregistration::internal::ParamKind;
}

impl InternalConversion for Validation {
    type Internal = crate::admissionregistration::internal::Validation;
}

impl InternalConversion for Variable {
    type Internal = crate::admissionregistration::internal::Variable;
}

impl InternalConversion for AuditAnnotation {
    type Internal = crate::admissionregistration::internal::AuditAnnotation;
}

impl InternalConversion for ValidatingAdmissionPolicyBinding {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyBinding;
}

impl InternalConversion for ValidatingAdmissionPolicyBindingList {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyBindingList;
}

impl InternalConversion for ValidatingAdmissionPolicyBindingSpec {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyBindingSpec;
}

impl InternalConversion for ParamRef {
    type Internal = crate::admissionregistration::internal::ParamRef;
}

impl InternalConversion for MatchResources {
    type Internal = crate::admissionregistration::internal::MatchResources;
}

impl InternalConversion for NamedRuleWithOperations {
    type Internal = crate::admissionregistration::internal::NamedRuleWithOperations;
}

impl InternalConversion for MutatingWebhookConfiguration {
    type Internal = crate::admissionregistration::internal::MutatingWebhookConfiguration;
}

impl InternalConversion for MutatingWebhookConfigurationList {
    type Internal = crate::admissionregistration::internal::MutatingWebhookConfigurationList;
}

impl InternalConversion for MutatingWebhook {
    type Internal = crate::admissionregistration::internal::MutatingWebhook;
}

impl InternalConversion for ValidatingWebhookConfiguration {
    type Internal = crate::admissionregistration::internal::ValidatingWebhookConfiguration;
}

impl InternalConversion for ValidatingWebhookConfigurationList {
    type Internal = crate::admissionregistration::internal::ValidatingWebhookConfigurationList;
}

impl InternalConversion for ValidatingWebhook {
    type Internal = crate::admissionregistration::internal::ValidatingWebhook;
}

impl InternalConversion for WebhookClientConfig {
    type Internal = crate::admissionregistration::internal::WebhookClientConfig;
}

impl InternalConversion for ServiceReference {
    type Internal = crate::admissionregistration::internal::ServiceReference;
}

impl InternalConversion for RuleWithOperations {
    type Internal = crate::admissionregistration::internal::RuleWithOperations;
}

impl InternalConversion for MatchCondition {
    type Internal = crate::admissionregistration::internal::MatchCondition;
}
