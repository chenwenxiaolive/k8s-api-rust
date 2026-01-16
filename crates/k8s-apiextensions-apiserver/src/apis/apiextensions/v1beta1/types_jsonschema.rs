//! JSON Schema types for CustomResourceDefinition validation
//!
//! This module provides Rust type definitions for JSON Schema following
//! Specification Draft 4 (http://json-schema.org/).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JSON represents any valid JSON value.
/// These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
pub type JSON = serde_json::Value;

/// FieldValueErrorReason is a machine-readable value providing more detail about why a field failed validation.
pub type FieldValueErrorReason = String;

/// JSONSchemaURL is a JSON schema URL.
pub type JSONSchemaURL = String;

/// JSONSchemaDefinitions is a map of schema definitions.
pub type JSONSchemaDefinitions = HashMap<String, JSONSchemaProps>;

/// JSONSchemaDependencies specifies property dependencies.
pub type JSONSchemaDependencies = HashMap<String, JSONSchemaPropsOrStringArray>;

/// ValidationRules describes a list of validation rules written in the CEL expression language.
pub type ValidationRules = Vec<ValidationRule>;

// FieldValueErrorReason constants
pub const FIELD_VALUE_REQUIRED: &str = "FieldValueRequired";
pub const FIELD_VALUE_DUPLICATE: &str = "FieldValueDuplicate";
pub const FIELD_VALUE_INVALID: &str = "FieldValueInvalid";
pub const FIELD_VALUE_FORBIDDEN: &str = "FieldValueForbidden";

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONSchemaProps {
    /// ID of the schema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// Schema URL
    #[serde(rename = "$schema", default, skip_serializing_if = "String::is_empty")]
    pub schema: JSONSchemaURL,
    /// Ref is a JSON reference to a schema.
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    /// Description of this schema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Type of this schema (e.g. "object", "string", "integer", "array", "boolean", "number")
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: String,
    /// Nullable indicates if null is allowed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    /// Format is the format of the value (e.g. "int64", "date-time")
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub format: String,
    /// Title of this schema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    /// Default value for this schema
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<JSON>,
    /// Maximum value for numeric types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// ExclusiveMaximum indicates whether maximum is exclusive
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    /// Minimum value for numeric types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// ExclusiveMinimum indicates whether minimum is exclusive
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    /// MaxLength is the maximum length for string types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// MinLength is the minimum length for string types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    /// Pattern is a regex pattern for string types
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pattern: String,
    /// MaxItems is the maximum number of items for array types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// MinItems is the minimum number of items for array types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    /// UniqueItems indicates if array items must be unique
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    /// MultipleOf specifies that numeric values must be a multiple of this value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    /// Enum is a list of allowed values
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<JSON>,
    /// MaxProperties is the maximum number of properties for object types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    /// MinProperties is the minimum number of properties for object types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    /// Required is a list of required properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    /// Items specifies the schema for array items
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<JSONSchemaPropsOrArray>,
    /// AllOf requires all schemas to match
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<JSONSchemaProps>,
    /// OneOf requires exactly one schema to match
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<JSONSchemaProps>,
    /// AnyOf requires at least one schema to match
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<JSONSchemaProps>,
    /// Not must not match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<JSONSchemaProps>>,
    /// Properties defines the schema for object properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, JSONSchemaProps>,
    /// AdditionalProperties specifies the schema for additional properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<JSONSchemaPropsOrBool>,
    /// PatternProperties defines schemas for properties matching patterns
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub pattern_properties: HashMap<String, JSONSchemaProps>,
    /// Dependencies specifies property dependencies
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub dependencies: JSONSchemaDependencies,
    /// AdditionalItems specifies the schema for additional array items
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<JSONSchemaPropsOrBool>,
    /// Definitions contains schema definitions
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub definitions: JSONSchemaDefinitions,
    /// ExternalDocs provides external documentation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Example is an example value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<JSON>,

    // x-kubernetes extensions
    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning
    /// fields which are not specified in the validation schema.
    #[serde(rename = "x-kubernetes-preserve-unknown-fields", default, skip_serializing_if = "Option::is_none")]
    pub x_preserve_unknown_fields: Option<bool>,
    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object.
    #[serde(rename = "x-kubernetes-embedded-resource", default, skip_serializing_if = "Option::is_none")]
    pub x_embedded_resource: Option<bool>,
    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string.
    #[serde(rename = "x-kubernetes-int-or-string", default, skip_serializing_if = "Option::is_none")]
    pub x_int_or_string: Option<bool>,
    /// x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map`
    /// by specifying the keys used as the index of the map.
    #[serde(rename = "x-kubernetes-list-map-keys", default, skip_serializing_if = "Vec::is_empty")]
    pub x_list_map_keys: Vec<String>,
    /// x-kubernetes-list-type annotates an array to further describe its topology.
    #[serde(rename = "x-kubernetes-list-type", default, skip_serializing_if = "Option::is_none")]
    pub x_list_type: Option<String>,
    /// x-kubernetes-map-type annotates an object to further describe its topology.
    #[serde(rename = "x-kubernetes-map-type", default, skip_serializing_if = "Option::is_none")]
    pub x_map_type: Option<String>,
    /// x-kubernetes-validations describes a list of validation rules written in the CEL expression language.
    #[serde(rename = "x-kubernetes-validations", default, skip_serializing_if = "Vec::is_empty")]
    pub x_validations: ValidationRules,
}

/// ValidationRule describes a validation rule written in the CEL expression language.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRule {
    /// Rule represents the expression which will be evaluated by CEL.
    pub rule: String,
    /// Message represents the message displayed when validation fails.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// MessageExpression declares a CEL expression that evaluates to the validation failure message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message_expression: String,
    /// Reason provides a machine-readable validation failure reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<FieldValueErrorReason>,
    /// FieldPath represents the field path returned when the validation fails.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
    /// OptionalOldSelf is used to opt a transition rule into evaluation even when the object is first created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_old_self: Option<bool>,
}

/// JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps
/// or an array of JSONSchemaProps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONSchemaPropsOrArray {
    /// A single schema
    Schema(Box<JSONSchemaProps>),
    /// An array of schemas
    Schemas(Vec<JSONSchemaProps>),
}

impl Default for JSONSchemaPropsOrArray {
    fn default() -> Self {
        JSONSchemaPropsOrArray::Schema(Box::default())
    }
}

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONSchemaPropsOrBool {
    /// A boolean value
    Bool(bool),
    /// A schema
    Schema(Box<JSONSchemaProps>),
}

impl Default for JSONSchemaPropsOrBool {
    fn default() -> Self {
        JSONSchemaPropsOrBool::Bool(true)
    }
}

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONSchemaPropsOrStringArray {
    /// An array of strings (property names)
    Strings(Vec<String>),
    /// A schema
    Schema(Box<JSONSchemaProps>),
}

impl Default for JSONSchemaPropsOrStringArray {
    fn default() -> Self {
        JSONSchemaPropsOrStringArray::Strings(Vec::new())
    }
}

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentation {
    /// Description of the external documentation
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// URL of the external documentation
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
}
