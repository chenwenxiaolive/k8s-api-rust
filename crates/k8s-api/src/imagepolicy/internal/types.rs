//! Internal type definitions for imagepolicy.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;


/// ImageReview checks if the set of images in a pod are allowed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: ImageReviewSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageReviewStatus>,
}


/// ImageReviewContainerSpec is a description of a container within the pod creation request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewContainerSpec {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
}


/// ImageReviewSpec is a description of the pod creation request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<ImageReviewContainerSpec>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}


/// ImageReviewStatus is the result of the review for the pod creation request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewStatus {
    pub allowed: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub audit_annotations: BTreeMap<String, String>,
}
