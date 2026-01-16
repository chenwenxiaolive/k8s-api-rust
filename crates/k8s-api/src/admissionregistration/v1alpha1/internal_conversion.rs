use super::*;

impl InternalConversion for MutatingAdmissionPolicy {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicy;
}

impl InternalConversion for MutatingAdmissionPolicyList {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicyList;
}

impl InternalConversion for MutatingAdmissionPolicySpec {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicySpec;
}

impl InternalConversion for MutatingAdmissionPolicyBinding {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicyBinding;
}

impl InternalConversion for MutatingAdmissionPolicyBindingList {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicyBindingList;
}

impl InternalConversion for MutatingAdmissionPolicyBindingSpec {
    type Internal = crate::admissionregistration::internal::MutatingAdmissionPolicyBindingSpec;
}

impl InternalConversion for ValidatingAdmissionPolicy {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicy;
}

impl InternalConversion for ValidatingAdmissionPolicyList {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyList;
}

impl InternalConversion for ValidatingAdmissionPolicySpec {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicySpec;
}

impl InternalConversion for ValidatingAdmissionPolicyStatus {
    type Internal = crate::admissionregistration::internal::ValidatingAdmissionPolicyStatus;
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

impl InternalConversion for ParamKind {
    type Internal = crate::admissionregistration::internal::ParamKind;
}

impl InternalConversion for MatchResources {
    type Internal = crate::admissionregistration::internal::MatchResources;
}

impl InternalConversion for NamedRuleWithOperations {
    type Internal = crate::admissionregistration::internal::NamedRuleWithOperations;
}

impl InternalConversion for Validation {
    type Internal = crate::admissionregistration::internal::Validation;
}

impl InternalConversion for AuditAnnotation {
    type Internal = crate::admissionregistration::internal::AuditAnnotation;
}

impl InternalConversion for MatchCondition {
    type Internal = crate::admissionregistration::internal::MatchCondition;
}

impl InternalConversion for Variable {
    type Internal = crate::admissionregistration::internal::Variable;
}

impl InternalConversion for TypeChecking {
    type Internal = crate::admissionregistration::internal::TypeChecking;
}

impl InternalConversion for ExpressionWarning {
    type Internal = crate::admissionregistration::internal::ExpressionWarning;
}

impl InternalConversion for ParamRef {
    type Internal = crate::admissionregistration::internal::ParamRef;
}

impl InternalConversion for Mutation {
    type Internal = crate::admissionregistration::internal::Mutation;
}

impl InternalConversion for ApplyConfiguration {
    type Internal = crate::admissionregistration::internal::ApplyConfiguration;
}

impl InternalConversion for JSONPatch {
    type Internal = crate::admissionregistration::internal::JSONPatch;
}
