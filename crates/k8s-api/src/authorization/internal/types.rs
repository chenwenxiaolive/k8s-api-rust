//! Internal type definitions for authorization.

use k8s_apimachinery::apis::meta::v1::{
    FieldSelectorRequirement, LabelSelectorRequirement, ObjectMeta, TypeMeta,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ExtraValue = Vec<String>;


/// FieldSelectorAttributes indicates a field limited access.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelectorAttributes {
    /// RawSelector is the serialization of a field selector from a query parameter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub raw_selector: String,
    /// Requirements is the parsed interpretation of a field selector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<FieldSelectorRequirement>,
}


/// LabelSelectorAttributes indicates a label limited access.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorAttributes {
    /// RawSelector is the serialization of a label selector from a query parameter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub raw_selector: String,
    /// Requirements is the parsed interpretation of a label selector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<LabelSelectorRequirement>,
}


/// LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalSubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: SubjectAccessReviewSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}


/// NonResourceAttributes includes the authorization attributes available for non-resource requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Verb is the standard HTTP verb.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub verb: String,
}


/// NonResourceRule holds information that describes a rule for the non-resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonResourceRule {
    /// Verb is a list of kubernetes non-resource API verbs.
    pub verbs: Vec<String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_urls: Vec<String>,
}


/// ResourceAttributes includes the authorization attributes available for resource requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAttributes {
    /// Namespace is the namespace of the action being requested.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Verb is a kubernetes resource API verb.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub verb: String,
    /// Group is the API Group of the Resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API Version of the Resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resource is one of the existing resource types.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Subresource is one of the existing resource types.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// FieldSelector describes the limitation on access based on field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_selector: Option<FieldSelectorAttributes>,
    /// LabelSelector describes the limitation on access based on labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelectorAttributes>,
}


/// ResourceRule is the list of actions the subject is allowed to perform on resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRule {
    /// Verb is a list of kubernetes resource API verbs.
    pub verbs: Vec<String>,
    /// APIGroups is the name of the APIGroup that contains the resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
}


/// SelfSubjectAccessReview checks whether or the current user can perform an action.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: SelfSubjectAccessReviewSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}


/// SelfSubjectAccessReviewSpec is a description of the access request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReviewSpec {
    /// ResourceAttributes describes information for a resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<NonResourceAttributes>,
}


/// SelfSubjectRulesReview enumerates the set of actions the current user can perform within a namespace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectRulesReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: SelfSubjectRulesReviewSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectRulesReviewStatus>,
}


/// SelfSubjectRulesReviewSpec defines the specification for SelfSubjectRulesReview.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectRulesReviewSpec {
    /// Namespace to evaluate rules for.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}


/// SubjectAccessReview checks whether or not a user or group can perform an action.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: SubjectAccessReviewSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}


/// SubjectAccessReviewSpec is a description of the access request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReviewSpec {
    /// ResourceAttributes describes information for a resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<NonResourceAttributes>,
    /// User is the user you're testing for.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// Groups is the groups you're testing for.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
    /// UID information about the requesting user.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
}


/// SubjectAccessReviewStatus represents the current state of a SubjectAccessReview.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    pub allowed: bool,
    /// Denied is optional. True if the action would be denied, otherwise false.
    #[serde(default)]
    pub denied: bool,
    /// Reason is optional. It indicates why a request was allowed or denied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// EvaluationError is an indication that some error occurred during the authorization check.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub evaluation_error: String,
}


/// SubjectRulesReviewStatus contains the result of a rules check.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectRulesReviewStatus {
    /// ResourceRules is the list of actions the subject is allowed to perform on resources.
    pub resource_rules: Vec<ResourceRule>,
    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources.
    pub non_resource_rules: Vec<NonResourceRule>,
    /// Incomplete is true when the rules returned by this call are incomplete.
    #[serde(default)]
    pub incomplete: bool,
    /// EvaluationError can appear in combination with Rules.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub evaluation_error: String,
}
