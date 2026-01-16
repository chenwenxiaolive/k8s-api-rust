use super::*;

impl InternalConversion for VolumeAttributesClass {
    type Internal = crate::storage::internal::VolumeAttributesClass;
}

impl InternalConversion for VolumeAttributesClassList {
    type Internal = crate::storage::internal::VolumeAttributesClassList;
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

impl InternalConversion for CSIStorageCapacity {
    type Internal = crate::storage::internal::CSIStorageCapacity;
}

impl InternalConversion for CSIStorageCapacityList {
    type Internal = crate::storage::internal::CSIStorageCapacityList;
}
