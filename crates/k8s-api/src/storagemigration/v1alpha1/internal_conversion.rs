use super::*;

impl InternalConversion for StorageVersionMigration {
    type Internal = crate::storagemigration::internal::StorageVersionMigration;
}

impl InternalConversion for StorageVersionMigrationList {
    type Internal = crate::storagemigration::internal::StorageVersionMigrationList;
}

impl InternalConversion for StorageVersionMigrationSpec {
    type Internal = crate::storagemigration::internal::StorageVersionMigrationSpec;
}

impl InternalConversion for GroupVersionResource {
    type Internal = crate::storagemigration::internal::GroupVersionResource;
}

impl InternalConversion for StorageVersionMigrationStatus {
    type Internal = crate::storagemigration::internal::StorageVersionMigrationStatus;
}

impl InternalConversion for MigrationCondition {
    type Internal = crate::storagemigration::internal::MigrationCondition;
}
