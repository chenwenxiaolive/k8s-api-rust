//! Resource API conversions
//!
//! This module provides conversions between resource API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// ResourceClaim: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaim>
    for k8s_api::resource::v1beta2::ResourceClaim
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaim, ConversionError> {
        let mut converted: k8s_api::resource::v1::ResourceClaim = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1",
            "ResourceClaim",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceClaim) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::resource::v1beta2::ResourceClaim = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1beta2",
            "ResourceClaim",
        );
        Ok(converted)
    }
}

// =============================================================================
// ResourceClaimTemplate: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaimTemplate>
    for k8s_api::resource::v1beta2::ResourceClaimTemplate
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaimTemplate, ConversionError> {
        let mut converted: k8s_api::resource::v1::ResourceClaimTemplate =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1",
            "ResourceClaimTemplate",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::resource::v1::ResourceClaimTemplate,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::resource::v1beta2::ResourceClaimTemplate =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1beta2",
            "ResourceClaimTemplate",
        );
        Ok(converted)
    }
}

// =============================================================================
// DeviceClass: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::DeviceClass> for k8s_api::resource::v1beta2::DeviceClass {
    fn convert_to(&self) -> Result<k8s_api::resource::v1::DeviceClass, ConversionError> {
        let mut converted: k8s_api::resource::v1::DeviceClass = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1",
            "DeviceClass",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::resource::v1::DeviceClass) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::resource::v1beta2::DeviceClass = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1beta2",
            "DeviceClass",
        );
        Ok(converted)
    }
}

// =============================================================================
// ResourceSlice: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceSlice> for k8s_api::resource::v1beta2::ResourceSlice {
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceSlice, ConversionError> {
        let mut converted: k8s_api::resource::v1::ResourceSlice = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1",
            "ResourceSlice",
        );
        Ok(converted)
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceSlice) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::resource::v1beta2::ResourceSlice = convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "resource.k8s.io/v1beta2",
            "ResourceSlice",
        );
        Ok(converted)
    }
}

// =============================================================================
// List conversions: v1beta2 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaimList>
    for k8s_api::resource::v1beta2::ResourceClaimList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaimList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceClaimList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaimList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceClaimList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta2::ResourceClaim::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta2",
                "ResourceClaimList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::ResourceClaimTemplateList>
    for k8s_api::resource::v1beta2::ResourceClaimTemplateList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::resource::v1::ResourceClaimTemplateList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceClaimTemplateList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaimTemplateList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::resource::v1::ResourceClaimTemplateList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta2::ResourceClaimTemplate::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta2",
                "ResourceClaimTemplateList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::DeviceClassList>
    for k8s_api::resource::v1beta2::DeviceClassList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::DeviceClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::DeviceClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "DeviceClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::DeviceClassList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta2::DeviceClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta2",
                "DeviceClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::ResourceSliceList>
    for k8s_api::resource::v1beta2::ResourceSliceList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceSliceList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceSliceList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceSliceList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceSliceList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta2::ResourceSlice::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta2",
                "ResourceSliceList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// ResourceClaim: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaim>
    for k8s_api::resource::v1beta1::ResourceClaim
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaim, ConversionError> {
        Ok(k8s_api::resource::v1::ResourceClaim {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaim",
            ),
            metadata: self.metadata.clone(),
            spec: convert_resource_claim_spec_v1beta1_to_v1(&self.spec)?,
            status: self
                .status
                .as_ref()
                .map(convert_resource_claim_status_v1beta1_to_v1)
                .transpose()?,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceClaim) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceClaim",
            ),
            metadata: other.metadata.clone(),
            spec: convert_resource_claim_spec_v1beta1_from_v1(&other.spec)?,
            status: other
                .status
                .as_ref()
                .map(convert_resource_claim_status_v1beta1_from_v1)
                .transpose()?,
        })
    }
}

fn convert_resource_claim_spec_v1beta1_to_v1(
    spec: &k8s_api::resource::v1beta1::ResourceClaimSpec,
) -> Result<k8s_api::resource::v1::ResourceClaimSpec, ConversionError> {
    Ok(k8s_api::resource::v1::ResourceClaimSpec {
        devices: spec
            .devices
            .as_ref()
            .map(convert_device_claim_v1beta1_to_v1)
            .transpose()?,
    })
}

fn convert_resource_claim_spec_v1beta1_from_v1(
    spec: &k8s_api::resource::v1::ResourceClaimSpec,
) -> Result<k8s_api::resource::v1beta1::ResourceClaimSpec, ConversionError> {
    Ok(k8s_api::resource::v1beta1::ResourceClaimSpec {
        devices: spec
            .devices
            .as_ref()
            .map(convert_device_claim_v1beta1_from_v1)
            .transpose()?,
    })
}

fn convert_resource_claim_status_v1beta1_to_v1(
    status: &k8s_api::resource::v1beta1::ResourceClaimStatus,
) -> Result<k8s_api::resource::v1::ResourceClaimStatus, ConversionError> {
    convert_via_json(status)
}

fn convert_resource_claim_status_v1beta1_from_v1(
    status: &k8s_api::resource::v1::ResourceClaimStatus,
) -> Result<k8s_api::resource::v1beta1::ResourceClaimStatus, ConversionError> {
    convert_via_json(status)
}

fn convert_device_claim_v1beta1_to_v1(
    claim: &k8s_api::resource::v1beta1::DeviceClaim,
) -> Result<k8s_api::resource::v1::DeviceClaim, ConversionError> {
    Ok(k8s_api::resource::v1::DeviceClaim {
        requests: claim
            .requests
            .iter()
            .map(convert_device_request_v1beta1_to_v1)
            .collect::<Result<Vec<_>, _>>()?,
        constraints: convert_via_json(&claim.constraints)?,
        config: convert_via_json(&claim.config)?,
    })
}

fn convert_device_claim_v1beta1_from_v1(
    claim: &k8s_api::resource::v1::DeviceClaim,
) -> Result<k8s_api::resource::v1beta1::DeviceClaim, ConversionError> {
    Ok(k8s_api::resource::v1beta1::DeviceClaim {
        requests: claim
            .requests
            .iter()
            .map(convert_device_request_v1beta1_from_v1)
            .collect::<Result<Vec<_>, _>>()?,
        constraints: convert_via_json(&claim.constraints)?,
        config: convert_via_json(&claim.config)?,
    })
}

fn convert_device_request_v1beta1_to_v1(
    request: &k8s_api::resource::v1beta1::DeviceRequest,
) -> Result<k8s_api::resource::v1::DeviceRequest, ConversionError> {
    Ok(k8s_api::resource::v1::DeviceRequest {
        name: request.name.clone(),
        exactly: Some(k8s_api::resource::v1::ExactDeviceRequest {
            device_class_name: request.device_class_name.clone(),
            selectors: convert_via_json(&request.selectors)?,
            allocation_mode: request.allocation_mode.clone(),
            count: request.count,
            admin_access: request.admin_access,
            tolerations: convert_via_json(&request.tolerations)?,
            capacity: request
                .capacity
                .as_ref()
                .map(convert_via_json)
                .transpose()?,
        }),
        first_available: request
            .first_available
            .iter()
            .map(convert_device_sub_request_v1beta1_to_v1)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn convert_device_request_v1beta1_from_v1(
    request: &k8s_api::resource::v1::DeviceRequest,
) -> Result<k8s_api::resource::v1beta1::DeviceRequest, ConversionError> {
    let (device_class_name, selectors, allocation_mode, count, admin_access, tolerations, capacity) =
        if let Some(exactly) = request.exactly.as_ref() {
            (
                exactly.device_class_name.clone(),
                convert_via_json(&exactly.selectors)?,
                exactly.allocation_mode.clone(),
                exactly.count,
                exactly.admin_access,
                convert_via_json(&exactly.tolerations)?,
                exactly.capacity.as_ref().map(convert_via_json).transpose()?,
            )
        } else if let Some(first) = request.first_available.first() {
            (
                first.device_class_name.clone(),
                convert_via_json(&first.selectors)?,
                first.allocation_mode.clone(),
                first.count,
                None,
                convert_via_json(&first.tolerations)?,
                first.capacity.as_ref().map(convert_via_json).transpose()?,
            )
        } else {
            (
                String::new(),
                Vec::new(),
                String::new(),
                None,
                None,
                Vec::new(),
                None,
            )
        };

    Ok(k8s_api::resource::v1beta1::DeviceRequest {
        name: request.name.clone(),
        device_class_name,
        selectors,
        allocation_mode,
        count,
        admin_access,
        first_available: request
            .first_available
            .iter()
            .map(convert_device_sub_request_v1beta1_from_v1)
            .collect::<Result<Vec<_>, _>>()?,
        tolerations,
        capacity,
    })
}

fn convert_device_sub_request_v1beta1_to_v1(
    request: &k8s_api::resource::v1beta1::DeviceSubRequest,
) -> Result<k8s_api::resource::v1::DeviceSubRequest, ConversionError> {
    Ok(k8s_api::resource::v1::DeviceSubRequest {
        name: request.name.clone(),
        device_class_name: request.device_class_name.clone(),
        selectors: convert_via_json(&request.selectors)?,
        allocation_mode: request.allocation_mode.clone(),
        count: request.count,
        tolerations: convert_via_json(&request.tolerations)?,
        capacity: request
            .capacity
            .as_ref()
            .map(convert_via_json)
            .transpose()?,
    })
}

fn convert_device_sub_request_v1beta1_from_v1(
    request: &k8s_api::resource::v1::DeviceSubRequest,
) -> Result<k8s_api::resource::v1beta1::DeviceSubRequest, ConversionError> {
    Ok(k8s_api::resource::v1beta1::DeviceSubRequest {
        name: request.name.clone(),
        device_class_name: request.device_class_name.clone(),
        selectors: convert_via_json(&request.selectors)?,
        allocation_mode: request.allocation_mode.clone(),
        count: request.count,
        tolerations: convert_via_json(&request.tolerations)?,
        capacity: request
            .capacity
            .as_ref()
            .map(convert_via_json)
            .transpose()?,
    })
}

// =============================================================================
// ResourceClaimTemplate: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaimTemplate>
    for k8s_api::resource::v1beta1::ResourceClaimTemplate
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaimTemplate, ConversionError> {
        Ok(k8s_api::resource::v1::ResourceClaimTemplate {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaimTemplate",
            ),
            metadata: self.metadata.clone(),
            spec: convert_resource_claim_template_spec_v1beta1_to_v1(&self.spec)?,
        })
    }

    fn convert_from(
        other: &k8s_api::resource::v1::ResourceClaimTemplate,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceClaimTemplate",
            ),
            metadata: other.metadata.clone(),
            spec: convert_resource_claim_template_spec_v1beta1_from_v1(&other.spec)?,
        })
    }
}

fn convert_resource_claim_template_spec_v1beta1_to_v1(
    spec: &k8s_api::resource::v1beta1::ResourceClaimTemplateSpec,
) -> Result<k8s_api::resource::v1::ResourceClaimTemplateSpec, ConversionError> {
    Ok(k8s_api::resource::v1::ResourceClaimTemplateSpec {
        metadata: spec.metadata.clone(),
        spec: convert_resource_claim_spec_v1beta1_to_v1(&spec.spec)?,
    })
}

fn convert_resource_claim_template_spec_v1beta1_from_v1(
    spec: &k8s_api::resource::v1::ResourceClaimTemplateSpec,
) -> Result<k8s_api::resource::v1beta1::ResourceClaimTemplateSpec, ConversionError> {
    Ok(k8s_api::resource::v1beta1::ResourceClaimTemplateSpec {
        metadata: spec.metadata.clone(),
        spec: convert_resource_claim_spec_v1beta1_from_v1(&spec.spec)?,
    })
}

// =============================================================================
// DeviceClass: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::DeviceClass> for k8s_api::resource::v1beta1::DeviceClass {
    fn convert_to(&self) -> Result<k8s_api::resource::v1::DeviceClass, ConversionError> {
        Ok(k8s_api::resource::v1::DeviceClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "DeviceClass",
            ),
            metadata: self.metadata.clone(),
            spec: convert_via_json(&self.spec)?,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::DeviceClass) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "DeviceClass",
            ),
            metadata: other.metadata.clone(),
            spec: convert_via_json(&other.spec)?,
        })
    }
}

// =============================================================================
// ResourceSlice: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceSlice> for k8s_api::resource::v1beta1::ResourceSlice {
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceSlice, ConversionError> {
        Ok(k8s_api::resource::v1::ResourceSlice {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceSlice",
            ),
            metadata: self.metadata.clone(),
            spec: convert_resource_slice_spec_v1beta1_to_v1(&self.spec)?,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceSlice) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceSlice",
            ),
            metadata: other.metadata.clone(),
            spec: convert_resource_slice_spec_v1beta1_from_v1(&other.spec)?,
        })
    }
}

// =============================================================================
// List conversions: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::resource::v1::ResourceClaimList>
    for k8s_api::resource::v1beta1::ResourceClaimList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceClaimList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceClaimList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaimList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceClaimList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta1::ResourceClaim::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceClaimList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::ResourceClaimTemplateList>
    for k8s_api::resource::v1beta1::ResourceClaimTemplateList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::resource::v1::ResourceClaimTemplateList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceClaimTemplateList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceClaimTemplateList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::resource::v1::ResourceClaimTemplateList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta1::ResourceClaimTemplate::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceClaimTemplateList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::DeviceClassList>
    for k8s_api::resource::v1beta1::DeviceClassList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::DeviceClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::DeviceClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "DeviceClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::DeviceClassList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta1::DeviceClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "DeviceClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::resource::v1::ResourceSliceList>
    for k8s_api::resource::v1beta1::ResourceSliceList
{
    fn convert_to(&self) -> Result<k8s_api::resource::v1::ResourceSliceList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::resource::v1::ResourceSliceList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1",
                "ResourceSliceList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::resource::v1::ResourceSliceList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::resource::v1beta1::ResourceSlice::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "resource.k8s.io/v1beta1",
                "ResourceSliceList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_resource_slice_spec_v1beta1_to_v1(
    spec: &k8s_api::resource::v1beta1::ResourceSliceSpec,
) -> Result<k8s_api::resource::v1::ResourceSliceSpec, ConversionError> {
    Ok(k8s_api::resource::v1::ResourceSliceSpec {
        driver: spec.driver.clone(),
        pool: convert_via_json(&spec.pool)?,
        node_name: if spec.node_name.is_empty() {
            None
        } else {
            Some(spec.node_name.clone())
        },
        node_selector: spec.node_selector.clone(),
        all_nodes: spec.all_nodes,
        devices: spec
            .devices
            .iter()
            .map(convert_device_v1beta1_to_v1)
            .collect::<Result<Vec<_>, _>>()?,
        per_device_node_selection: spec.per_device_node_selection,
        shared_counters: convert_via_json(&spec.shared_counters)?,
    })
}

fn convert_resource_slice_spec_v1beta1_from_v1(
    spec: &k8s_api::resource::v1::ResourceSliceSpec,
) -> Result<k8s_api::resource::v1beta1::ResourceSliceSpec, ConversionError> {
    Ok(k8s_api::resource::v1beta1::ResourceSliceSpec {
        driver: spec.driver.clone(),
        pool: convert_via_json(&spec.pool)?,
        node_name: spec.node_name.clone().unwrap_or_default(),
        node_selector: spec.node_selector.clone(),
        all_nodes: spec.all_nodes,
        devices: spec
            .devices
            .iter()
            .map(convert_device_v1beta1_from_v1)
            .collect::<Result<Vec<_>, _>>()?,
        per_device_node_selection: spec.per_device_node_selection,
        shared_counters: convert_via_json(&spec.shared_counters)?,
    })
}

fn convert_device_v1beta1_to_v1(
    device: &k8s_api::resource::v1beta1::Device,
) -> Result<k8s_api::resource::v1::Device, ConversionError> {
    let mut converted = k8s_api::resource::v1::Device {
        name: device.name.clone(),
        ..Default::default()
    };

    if let Some(basic) = device.basic.as_ref() {
        converted.attributes = convert_via_json(&basic.attributes)?;
        converted.capacity = convert_via_json(&basic.capacity)?;
        converted.consumes_counters = convert_via_json(&basic.consumes_counters)?;
        converted.node_name = basic.node_name.clone();
        converted.node_selector = basic.node_selector.clone();
        converted.all_nodes = basic.all_nodes;
        converted.taints = convert_via_json(&basic.taints)?;
        converted.binds_to_node = basic.binds_to_node;
        converted.binding_conditions = basic.binding_conditions.clone();
        converted.binding_failure_conditions = basic.binding_failure_conditions.clone();
        converted.allow_multiple_allocations = basic.allow_multiple_allocations;
    }

    Ok(converted)
}

fn convert_device_v1beta1_from_v1(
    device: &k8s_api::resource::v1::Device,
) -> Result<k8s_api::resource::v1beta1::Device, ConversionError> {
    let basic = k8s_api::resource::v1beta1::BasicDevice {
        attributes: convert_via_json(&device.attributes)?,
        capacity: convert_via_json(&device.capacity)?,
        consumes_counters: convert_via_json(&device.consumes_counters)?,
        node_name: device.node_name.clone(),
        node_selector: device.node_selector.clone(),
        all_nodes: device.all_nodes,
        taints: convert_via_json(&device.taints)?,
        binds_to_node: device.binds_to_node,
        binding_conditions: device.binding_conditions.clone(),
        binding_failure_conditions: device.binding_failure_conditions.clone(),
        allow_multiple_allocations: device.allow_multiple_allocations,
    };

    Ok(k8s_api::resource::v1beta1::Device {
        name: device.name.clone(),
        basic: Some(basic),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_resource_claim_v1beta1_to_v1_conversion() {
        let mut capacity = BTreeMap::new();
        capacity.insert("memory".to_string(), "1Gi".to_string());

        let v1beta1_claim = k8s_api::resource::v1beta1::ResourceClaim {
            metadata: ObjectMeta::named("claim"),
            spec: k8s_api::resource::v1beta1::ResourceClaimSpec {
                devices: Some(k8s_api::resource::v1beta1::DeviceClaim {
                    requests: vec![k8s_api::resource::v1beta1::DeviceRequest {
                        name: "req".to_string(),
                        device_class_name: "gpu".to_string(),
                        selectors: vec![],
                        allocation_mode: "ExactCount".to_string(),
                        count: Some(2),
                        admin_access: Some(true),
                        first_available: vec![k8s_api::resource::v1beta1::DeviceSubRequest {
                            name: "fallback".to_string(),
                            device_class_name: "fpga".to_string(),
                            selectors: vec![],
                            allocation_mode: "ExactCount".to_string(),
                            count: Some(1),
                            tolerations: vec![],
                            capacity: None,
                        }],
                        tolerations: vec![],
                        capacity: Some(k8s_api::resource::v1beta1::CapacityRequirements {
                            requests: capacity,
                        }),
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        };

        let v1_claim: k8s_api::resource::v1::ResourceClaim = v1beta1_claim.convert_to().unwrap();
        let devices = v1_claim.spec.devices.as_ref().unwrap();
        let request = &devices.requests[0];
        let exactly = request.exactly.as_ref().unwrap();
        assert_eq!(exactly.device_class_name, "gpu");
        assert_eq!(exactly.count, Some(2));
        assert_eq!(request.first_available.len(), 1);
        assert_eq!(request.first_available[0].device_class_name, "fpga");
    }

    #[test]
    fn test_resource_slice_v1beta1_device_basic_conversion() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "vendor".to_string(),
            k8s_api::resource::v1beta1::DeviceAttribute {
                string_value: Some("nvidia".to_string()),
                ..Default::default()
            },
        );

        let v1beta1_slice = k8s_api::resource::v1beta1::ResourceSlice {
            metadata: ObjectMeta::named("slice"),
            spec: k8s_api::resource::v1beta1::ResourceSliceSpec {
                driver: "dra.example.com".to_string(),
                pool: k8s_api::resource::v1beta1::ResourcePool {
                    name: "pool".to_string(),
                    generation: 1,
                    resource_slice_count: 1,
                },
                node_name: "node-a".to_string(),
                devices: vec![k8s_api::resource::v1beta1::Device {
                    name: "dev0".to_string(),
                    basic: Some(k8s_api::resource::v1beta1::BasicDevice {
                        attributes,
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let v1_slice: k8s_api::resource::v1::ResourceSlice = v1beta1_slice.convert_to().unwrap();
        let device = &v1_slice.spec.devices[0];
        assert_eq!(device.name, "dev0");
        assert!(device.attributes.contains_key("vendor"));
        assert_eq!(v1_slice.spec.node_name, Some("node-a".to_string()));
    }

    #[test]
    fn test_resource_claim_list_roundtrip() {
        let list = k8s_api::resource::v1beta1::ResourceClaimList {
            metadata: ListMeta {
                resource_version: "4".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::resource::v1beta1::ResourceClaim {
                metadata: ObjectMeta::named("claim-list"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::resource::v1::ResourceClaimList = list.convert_to().unwrap();
        assert_eq!(v1_list.metadata.resource_version, "4");
        assert_eq!(v1_list.items[0].metadata.name, "claim-list");

        let roundtrip: k8s_api::resource::v1beta1::ResourceClaimList =
            k8s_api::resource::v1beta1::ResourceClaimList::convert_from(&v1_list).unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
