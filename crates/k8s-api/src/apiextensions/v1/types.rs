//! API Extensions v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{Condition, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// CustomResourceDefinition
// =============================================================================

/// CustomResourceDefinition represents a resource that should be exposed on the API server.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinition {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: CustomResourceDefinitionSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomResourceDefinitionStatus>,
}

/// CustomResourceDefinitionList is a list of CustomResourceDefinition objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<CustomResourceDefinition>,
}

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionSpec {
    /// Group is the API group of the defined custom resource.
    pub group: String,
    /// Names specify the resource and kind names for the custom resource.
    pub names: CustomResourceDefinitionNames,
    /// Scope indicates whether the defined custom resource is cluster- or namespace-scoped.
    pub scope: String,
    /// Versions is the list of all API versions of the defined custom resource.
    pub versions: Vec<CustomResourceDefinitionVersion>,
    /// Conversion defines conversion settings for the CRD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion: Option<CustomResourceConversion>,
    /// PreserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_unknown_fields: Option<bool>,
}

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_names: Option<CustomResourceDefinitionNames>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stored_versions: Vec<String>,
}

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionNames {
    /// Plural is the plural name of the resource to serve.
    pub plural: String,
    /// Singular is the singular name of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular: String,
    /// ShortNames are short names for the resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// Kind is the serialized kind of the resource.
    pub kind: String,
    /// ListKind is the serialized kind of the list for this resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub list_kind: String,
    /// Categories is a list of grouped resources this custom resource belongs to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionVersion {
    /// Name is the version name.
    pub name: String,
    /// Served is a flag enabling/disabling this version from being served via REST APIs.
    pub served: bool,
    /// Storage indicates this version should be used when persisting custom resources to storage.
    pub storage: bool,
    /// Deprecated indicates this version of the custom resource API is deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// DeprecationWarning overrides the default warning returned to API clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: Option<String>,
    /// Schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<CustomResourceValidation>,
    /// Subresources specify what subresources this version of the defined custom resource have.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresources: Option<CustomResourceSubresources>,
    /// AdditionalPrinterColumns specifies additional columns returned in Table output.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_printer_columns: Vec<CustomResourceColumnDefinition>,
}

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceValidation {
    /// OpenAPIV3Schema is the OpenAPI v3 schema to use for validation and pruning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_apiv3_schema: Option<JSONSchemaProps>,
}

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONSchemaProps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JSONSchemaProps>>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub properties: std::collections::BTreeMap<String, JSONSchemaProps>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<JSONSchemaPropsOrBool>>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_embedded_resource: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_int_or_string: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x_kubernetes_list_map_keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_list_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_map_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x_kubernetes_validations: Vec<ValidationRule>,
}

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONSchemaPropsOrBool {
    Schema(JSONSchemaProps),
    Bool(bool),
}

impl Default for JSONSchemaPropsOrBool {
    fn default() -> Self {
        JSONSchemaPropsOrBool::Bool(true)
    }
}

/// ValidationRule describes a validation rule written in the CEL expression language.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRule {
    /// Rule represents the expression which will be evaluated by CEL.
    pub rule: String,
    /// Message represents the message displayed when validation fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// MessageExpression declares a CEL expression that evaluates to the validation failure message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_expression: Option<String>,
    /// Reason provides a machine-readable validation failure reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// FieldPath represents the field path returned when the validation fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresources {
    /// Status indicates the custom resource should serve a /status subresource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomResourceSubresourceStatus>,
    /// Scale indicates the custom resource should serve a /scale subresource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<CustomResourceSubresourceScale>,
}

/// CustomResourceSubresourceStatus defines how to serve the status subresource for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresourceStatus {}

/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresourceScale {
    /// SpecReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale.Spec.Replicas.
    pub spec_replicas_path: String,
    /// StatusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale.Status.Replicas.
    pub status_replicas_path: String,
    /// LabelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale.Status.Selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector_path: Option<String>,
}

/// CustomResourceColumnDefinition specifies a column for server side printing.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceColumnDefinition {
    /// Name is a human readable name for the column.
    pub name: String,
    /// Type is an OpenAPI type definition for this column.
    #[serde(rename = "type")]
    pub type_: String,
    /// Format is an optional OpenAPI type definition for this column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub format: String,
    /// Description is a human readable description of this column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Priority is an integer defining the relative importance of this column.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// JSONPath is a simple JSON path which is evaluated against each custom resource.
    pub json_path: String,
}

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceConversion {
    /// Strategy specifies how custom resources are converted between versions.
    pub strategy: String,
    /// Webhook describes how to call the conversion webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookConversion>,
}

/// WebhookConversion describes how to call a conversion webhook.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookConversion {
    /// ClientConfig is the instructions for how to call the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_config: Option<WebhookClientConfig>,
    /// ConversionReviewVersions is an ordered list of preferred ConversionReview versions.
    pub conversion_review_versions: Vec<String>,
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

/// ServiceReference holds a reference to Service.legacy.k8s.io.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service.
    pub namespace: String,
    /// Name is the name of the service.
    pub name: String,
    /// Path is an optional URL path at which the webhook will be contacted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port is an optional service port at which the webhook will be contacted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
