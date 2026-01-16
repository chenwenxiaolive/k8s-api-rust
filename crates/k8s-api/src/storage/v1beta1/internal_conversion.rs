use super::*;

impl InternalConversion for VolumeAttributesClass {
    type Internal = crate::storage::internal::VolumeAttributesClass;
}

impl InternalConversion for VolumeAttributesClassList {
    type Internal = crate::storage::internal::VolumeAttributesClassList;
}

impl InternalConversion for CSIStorageCapacity {
    type Internal = crate::storage::internal::CSIStorageCapacity;
}

impl InternalConversion for CSIStorageCapacityList {
    type Internal = crate::storage::internal::CSIStorageCapacityList;
}

impl InternalConversion for StorageClass {
    type Internal = crate::storage::internal::StorageClass;
}

impl InternalConversion for StorageClassList {
    type Internal = crate::storage::internal::StorageClassList;
}

impl InternalConversion for VolumeAttachment {
    type Internal = crate::storage::internal::VolumeAttachment;
}

impl InternalConversion for VolumeAttachmentList {
    type Internal = crate::storage::internal::VolumeAttachmentList;
}

impl InternalConversion for VolumeAttachmentSpec {
    type Internal = crate::storage::internal::VolumeAttachmentSpec;
}

impl InternalConversion for VolumeAttachmentSource {
    type Internal = crate::storage::internal::VolumeAttachmentSource;
}

impl InternalConversion for VolumeAttachmentStatus {
    type Internal = crate::storage::internal::VolumeAttachmentStatus;
}

impl InternalConversion for VolumeError {
    type Internal = crate::storage::internal::VolumeError;
}

impl InternalConversion for CSIDriver {
    type Internal = crate::storage::internal::CSIDriver;
}

impl InternalConversion for CSIDriverList {
    type Internal = crate::storage::internal::CSIDriverList;
}

impl InternalConversion for CSIDriverSpec {
    type Internal = crate::storage::internal::CSIDriverSpec;
}

impl InternalConversion for TokenRequest {
    type Internal = crate::storage::internal::TokenRequest;
}

impl InternalConversion for CSINode {
    type Internal = crate::storage::internal::CSINode;
}

impl InternalConversion for CSINodeList {
    type Internal = crate::storage::internal::CSINodeList;
}

impl InternalConversion for CSINodeSpec {
    type Internal = crate::storage::internal::CSINodeSpec;
}

impl InternalConversion for CSINodeDriver {
    type Internal = crate::storage::internal::CSINodeDriver;
}

impl InternalConversion for VolumeNodeResources {
    type Internal = crate::storage::internal::VolumeNodeResources;
}
