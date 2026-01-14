//! CustomResourceDefinition v1 type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

use super::types_jsonschema::JSONSchemaProps;

// =============================================================================
// CustomResourceDefinition
// =============================================================================

/// CustomResourceDefinition represents a resource that should be exposed on the API server.
/// Its name MUST be in the format <.spec.name>.<.spec.group>.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinition {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec describes how the user wants the resources to appear
    pub spec: CustomResourceDefinitionSpec,
    /// Status indicates the actual state of the CustomResourceDefinition
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
    /// Items individual CustomResourceDefinitions
    pub items: Vec<CustomResourceDefinition>,
}

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionSpec {
    /// Group is the group this resource belongs in
    pub group: String,
    /// Names are the names used to describe this custom resource
    pub names: CustomResourceDefinitionNames,
    /// Scope indicates whether this resource is cluster or namespace scoped. Default is namespaced
    pub scope: String,
    /// Versions is the list of all supported versions for this resource.
    pub versions: Vec<CustomResourceDefinitionVersion>,
    /// Conversion defines conversion settings for the CRD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion: Option<CustomResourceConversion>,
    /// PreserveUnknownFields disables pruning of object fields which are not
    /// specified in the OpenAPI schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_unknown_fields: Option<bool>,
}

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionNames {
    /// Plural is the plural name of the resource to serve.
    pub plural: String,
    /// Singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased <kind>
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular: String,
    /// ShortNames are short names for the resource. It must be all lowercase.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// Kind is the serialized kind of the resource. It is normally CamelCase and singular.
    pub kind: String,
    /// ListKind is the serialized kind of the list for this resource. Defaults to <kind>List.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub list_kind: String,
    /// Categories is a list of grouped resources custom resources belong to (e.g. 'all')
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionVersion {
    /// Name is the version name, e.g. "v1", "v2beta1", etc.
    pub name: String,
    /// Served is a flag enabling/disabling this version from being served via REST APIs
    pub served: bool,
    /// Storage flags the version as storage version. There must be exactly one flagged as storage version.
    pub storage: bool,
    /// Deprecated indicates this version of the custom resource API is deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// DeprecationWarning overrides the default warning returned to API clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: Option<String>,
    /// Schema describes the schema for CustomResource used in validation, pruning, and defaulting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<CustomResourceValidation>,
    /// Subresources describes the subresources for CustomResource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresources: Option<CustomResourceSubresources>,
    /// AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_printer_columns: Vec<CustomResourceColumnDefinition>,
    /// SelectableFields specifies paths to fields that may be used as field selectors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectable_fields: Vec<SelectableField>,
}

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceValidation {
    /// OpenAPIV3Schema is the OpenAPI v3 schema to be validated against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_apiv3_schema: Option<JSONSchemaProps>,
}

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresources {
    /// Status denotes the status subresource for CustomResources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomResourceSubresourceStatus>,
    /// Scale denotes the scale subresource for CustomResources
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
    /// SpecReplicasPath defines the JSON path inside of a CustomResource that corresponds to Scale.Spec.Replicas.
    pub spec_replicas_path: String,
    /// StatusReplicasPath defines the JSON path inside of a CustomResource that corresponds to Scale.Status.Replicas.
    pub status_replicas_path: String,
    /// LabelSelectorPath defines the JSON path inside of a CustomResource that corresponds to Scale.Status.Selector.
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
    /// Priority is an integer defining the relative importance of this column compared to others.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// JSONPath is a simple JSON path, i.e. without array notation.
    pub json_path: String,
}

/// SelectableField specifies the JSON path of a field that may be used with field selectors.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectableField {
    /// JSONPath is a simple JSON path which is evaluated against each custom resource to produce a field selector value.
    pub json_path: String,
}

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceConversion {
    /// Strategy specifies the conversion strategy. Allowed values are:
    /// - "None": The converter only change the apiVersion and would not touch any other field in the CR.
    /// - "Webhook": API Server will call to an external webhook to do the conversion.
    pub strategy: String,
    /// Webhook describes how to call the conversion webhook. Required when strategy is "Webhook".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookConversion>,
}

/// WebhookConversion describes how to call a conversion webhook.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookConversion {
    /// ClientConfig is the instructions for how to call the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_config: Option<WebhookClientConfig>,
    /// ConversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects.
    pub conversion_review_versions: Vec<String>,
}

/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookClientConfig {
    /// URL gives the location of the webhook, in standard URL form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Service is a reference to the service for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,
    /// CABundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_bundle: Option<Vec<u8>>,
}

/// ServiceReference holds a reference to Service.legacy.k8s.io
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
    /// Port is the port on the service that hosting webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

// =============================================================================
// CustomResourceDefinitionStatus
// =============================================================================

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionStatus {
    /// Conditions indicate state for particular aspects of a CustomResourceDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<CustomResourceDefinitionCondition>,
    /// AcceptedNames are the names that are actually being used to serve discovery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_names: Option<CustomResourceDefinitionNames>,
    /// StoredVersions are all versions of CustomResources that were ever persisted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stored_versions: Vec<String>,
}

/// CustomResourceDefinitionCondition contains details for the current condition of this pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionCondition {
    /// Type is the type of the condition. Types include Established, NamesAccepted and Terminating.
    #[serde(rename = "type")]
    pub type_: String,
    /// Status is the status of the condition. Can be True, False, Unknown.
    pub status: String,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
