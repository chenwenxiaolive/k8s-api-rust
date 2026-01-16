use super::*;

impl InternalConversion for CustomResourceDefinition {
    type Internal = crate::apiextensions::internal::CustomResourceDefinition;
}

impl InternalConversion for CustomResourceDefinitionList {
    type Internal = crate::apiextensions::internal::CustomResourceDefinitionList;
}

impl InternalConversion for CustomResourceDefinitionSpec {
    type Internal = crate::apiextensions::internal::CustomResourceDefinitionSpec;
}

impl InternalConversion for CustomResourceDefinitionStatus {
    type Internal = crate::apiextensions::internal::CustomResourceDefinitionStatus;
}

impl InternalConversion for CustomResourceDefinitionNames {
    type Internal = crate::apiextensions::internal::CustomResourceDefinitionNames;
}

impl InternalConversion for CustomResourceDefinitionVersion {
    type Internal = crate::apiextensions::internal::CustomResourceDefinitionVersion;
}

impl InternalConversion for CustomResourceValidation {
    type Internal = crate::apiextensions::internal::CustomResourceValidation;
}

impl InternalConversion for JSONSchemaProps {
    type Internal = crate::apiextensions::internal::JSONSchemaProps;
}

impl InternalConversion for JSONSchemaPropsOrBool {
    type Internal = crate::apiextensions::internal::JSONSchemaPropsOrBool;
}

impl InternalConversion for ValidationRule {
    type Internal = crate::apiextensions::internal::ValidationRule;
}

impl InternalConversion for CustomResourceSubresources {
    type Internal = crate::apiextensions::internal::CustomResourceSubresources;
}

impl InternalConversion for CustomResourceSubresourceStatus {
    type Internal = crate::apiextensions::internal::CustomResourceSubresourceStatus;
}

impl InternalConversion for CustomResourceSubresourceScale {
    type Internal = crate::apiextensions::internal::CustomResourceSubresourceScale;
}

impl InternalConversion for CustomResourceColumnDefinition {
    type Internal = crate::apiextensions::internal::CustomResourceColumnDefinition;
}

impl InternalConversion for CustomResourceConversion {
    type Internal = crate::apiextensions::internal::CustomResourceConversion;
}

impl InternalConversion for WebhookConversion {
    type Internal = crate::apiextensions::internal::WebhookConversion;
}

impl InternalConversion for WebhookClientConfig {
    type Internal = crate::apiextensions::internal::WebhookClientConfig;
}

impl InternalConversion for ServiceReference {
    type Internal = crate::apiextensions::internal::ServiceReference;
}
