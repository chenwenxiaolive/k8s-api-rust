use super::*;

impl InternalConversion for FlowSchema {
    type Internal = crate::flowcontrol::internal::FlowSchema;
}

impl InternalConversion for FlowSchemaList {
    type Internal = crate::flowcontrol::internal::FlowSchemaList;
}

impl InternalConversion for FlowSchemaSpec {
    type Internal = crate::flowcontrol::internal::FlowSchemaSpec;
}

impl InternalConversion for FlowSchemaStatus {
    type Internal = crate::flowcontrol::internal::FlowSchemaStatus;
}

impl InternalConversion for FlowSchemaCondition {
    type Internal = crate::flowcontrol::internal::FlowSchemaCondition;
}

impl InternalConversion for PriorityLevelConfigurationReference {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfigurationReference;
}

impl InternalConversion for FlowDistinguisherMethod {
    type Internal = crate::flowcontrol::internal::FlowDistinguisherMethod;
}

impl InternalConversion for PolicyRulesWithSubjects {
    type Internal = crate::flowcontrol::internal::PolicyRulesWithSubjects;
}

impl InternalConversion for Subject {
    type Internal = crate::flowcontrol::internal::Subject;
}

impl InternalConversion for UserSubject {
    type Internal = crate::flowcontrol::internal::UserSubject;
}

impl InternalConversion for GroupSubject {
    type Internal = crate::flowcontrol::internal::GroupSubject;
}

impl InternalConversion for ServiceAccountSubject {
    type Internal = crate::flowcontrol::internal::ServiceAccountSubject;
}

impl InternalConversion for ResourcePolicyRule {
    type Internal = crate::flowcontrol::internal::ResourcePolicyRule;
}

impl InternalConversion for NonResourcePolicyRule {
    type Internal = crate::flowcontrol::internal::NonResourcePolicyRule;
}

impl InternalConversion for PriorityLevelConfiguration {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfiguration;
}

impl InternalConversion for PriorityLevelConfigurationList {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfigurationList;
}

impl InternalConversion for PriorityLevelConfigurationSpec {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfigurationSpec;
}

impl InternalConversion for PriorityLevelConfigurationStatus {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfigurationStatus;
}

impl InternalConversion for PriorityLevelConfigurationCondition {
    type Internal = crate::flowcontrol::internal::PriorityLevelConfigurationCondition;
}

impl InternalConversion for LimitedPriorityLevelConfiguration {
    type Internal = crate::flowcontrol::internal::LimitedPriorityLevelConfiguration;
}

impl InternalConversion for ExemptPriorityLevelConfiguration {
    type Internal = crate::flowcontrol::internal::ExemptPriorityLevelConfiguration;
}

impl InternalConversion for LimitResponse {
    type Internal = crate::flowcontrol::internal::LimitResponse;
}

impl InternalConversion for QueuingConfiguration {
    type Internal = crate::flowcontrol::internal::QueuingConfiguration;
}
