//! Internal type definitions for authentication.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ExtraValue = Vec<String>;


/// BoundObjectReference is a reference to an object that a token is bound to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundObjectReference {
    /// Kind of the referent.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// API version of the referent.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Name of the referent.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// UID of the referent.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub uid: String,
}


/// SelfSubjectReview contains the user information that the kube-apiserver has about the user making this request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SelfSubjectReviewStatus>,
}


/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReviewStatus {
    /// UserInfo holds the information about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
}


/// TokenRequest requests a token for a given service account.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    pub spec: TokenRequestSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenRequestStatus>,
}


/// TokenRequestSpec contains client provided parameters of a token request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestSpec {
    /// Audiences are the intendend audiences of the token.
    pub audiences: Vec<String>,
    /// ExpirationSeconds is the requested duration of validity of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
    /// BoundObjectRef is a reference to an object that the token will be bound to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_object_ref: Option<BoundObjectReference>,
}


/// TokenRequestStatus is the result of a token request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestStatus {
    /// Token is the opaque bearer token.
    pub token: String,
    /// ExpirationTimestamp is the time of expiration of the returned token.
    pub expiration_timestamp: String,
}


/// TokenReview attempts to authenticate a token to a known user.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    pub spec: TokenReviewSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenReviewStatus>,
}


/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
    /// Audiences is a list of the identifiers that the resource server presented with the token.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
}


/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    pub authenticated: bool,
    /// User is the UserInfo associated with the provided token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    /// Audiences are audience identifiers chosen by the authenticator.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
    /// Error indicates why the token was not authenticated.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
}


/// UserInfo holds the information about the user needed to implement the user.Info interface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// Username is the name that uniquely identifies this user among all active users.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// UID is a unique value that identifies this user across time.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// Groups are the names of groups this user is a part of.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Extra holds additional information provided by the authenticator.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
}
