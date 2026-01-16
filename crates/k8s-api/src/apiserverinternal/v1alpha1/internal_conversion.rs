use super::*;

impl InternalConversion for StorageVersionList {
    type Internal = crate::apiserverinternal::internal::StorageVersionList;
}

impl InternalConversion for StorageVersion {
    type Internal = crate::apiserverinternal::internal::StorageVersion;
}

impl InternalConversion for StorageVersionSpec {
    type Internal = crate::apiserverinternal::internal::StorageVersionSpec;
}

impl InternalConversion for StorageVersionStatus {
    type Internal = crate::apiserverinternal::internal::StorageVersionStatus;
}

impl InternalConversion for ServerStorageVersion {
    type Internal = crate::apiserverinternal::internal::ServerStorageVersion;
}

impl InternalConversion for StorageVersionCondition {
    type Internal = crate::apiserverinternal::internal::StorageVersionCondition;
}
