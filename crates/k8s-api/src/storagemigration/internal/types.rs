//! Internal type definitions for storagemigration.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type MigrationConditionType = String;

pub const MIGRATION_FAILED: &str = "Failed";
pub const MIGRATION_RUNNING: &str = "Running";
pub const MIGRATION_SUCCEEDED: &str = "Succeeded";


/// GroupVersionResource contains the names of the group, the version, and the resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    /// The name of the group.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// The name of the version.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// The name of the resource.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource: String,
}


/// MigrationCondition describes the state of a migration at a certain point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationCondition {
    /// Type of the condition.
    #[serde(rename = "type")]
    pub type_: MigrationConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: crate::core::v1::ConditionStatus,
    /// The last time this condition was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// StorageVersionMigration represents a migration of stored data to the latest
/// storage version.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigration {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Specification of the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<StorageVersionMigrationSpec>,
    /// Status of the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StorageVersionMigrationStatus>,
}


/// StorageVersionMigrationList is a collection of storage version migrations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    /// Items is the list of StorageVersionMigration.
    pub items: Vec<StorageVersionMigration>,
}


/// StorageVersionMigrationSpec is the specification of the migration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationSpec {
    /// The resource that is being migrated.
    pub resource: GroupVersionResource,
    /// The token used in the list options to get the next chunk of objects to migrate.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub continue_token: String,
}


/// StorageVersionMigrationStatus is the status of the storage version migration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationStatus {
    /// The latest available observations of the migration's current state.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<MigrationCondition>,
    /// ResourceVersion to compare with the GC cache for performing the migration.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
}
