//! Authentication v1alpha1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub use crate::authentication::v1::UserInfo;

// =============================================================================
// SelfSubjectReview
// =============================================================================

/// SelfSubjectReview contains the user information that the kube-apiserver has about the user making this request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SelfSubjectReviewStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReviewStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
}
