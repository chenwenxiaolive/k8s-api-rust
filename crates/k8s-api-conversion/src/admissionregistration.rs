//! Admissionregistration API conversions
//!
//! This module provides conversions between admissionregistration API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// MutatingWebhookConfiguration: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::MutatingWebhookConfiguration>
    for k8s_api::admissionregistration::v1beta1::MutatingWebhookConfiguration
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1::MutatingWebhookConfiguration, ConversionError>
    {
        let webhooks = self
            .webhooks
            .iter()
            .map(convert_mutating_webhook_to_v1)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::MutatingWebhookConfiguration {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "MutatingWebhookConfiguration",
            ),
            metadata: self.metadata.clone(),
            webhooks,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::MutatingWebhookConfiguration,
    ) -> Result<Self, ConversionError> {
        let webhooks = other
            .webhooks
            .iter()
            .map(convert_mutating_webhook_from_v1)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "MutatingWebhookConfiguration",
            ),
            metadata: other.metadata.clone(),
            webhooks,
        })
    }
}

// =============================================================================
// MutatingWebhookConfigurationList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::MutatingWebhookConfigurationList>
    for k8s_api::admissionregistration::v1beta1::MutatingWebhookConfigurationList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1::MutatingWebhookConfigurationList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::MutatingWebhookConfigurationList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "MutatingWebhookConfigurationList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::MutatingWebhookConfigurationList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1beta1::MutatingWebhookConfiguration::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "MutatingWebhookConfigurationList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_mutating_webhook_to_v1(
    webhook: &k8s_api::admissionregistration::v1beta1::MutatingWebhook,
) -> Result<k8s_api::admissionregistration::v1::MutatingWebhook, ConversionError> {
    let side_effects = webhook.side_effects.clone().ok_or_else(|| {
        ConversionError::MissingField("mutatingWebhook.sideEffects".to_string())
    })?;
    Ok(k8s_api::admissionregistration::v1::MutatingWebhook {
        name: webhook.name.clone(),
        client_config: convert_via_json(&webhook.client_config)?,
        rules: webhook.rules.clone(),
        failure_policy: webhook.failure_policy.clone(),
        match_policy: webhook.match_policy.clone(),
        namespace_selector: webhook.namespace_selector.clone(),
        object_selector: webhook.object_selector.clone(),
        side_effects,
        timeout_seconds: webhook.timeout_seconds,
        admission_review_versions: webhook.admission_review_versions.clone(),
        reinvocation_policy: webhook.reinvocation_policy.clone(),
        match_conditions: convert_via_json(&webhook.match_conditions)?,
    })
}

fn convert_mutating_webhook_from_v1(
    webhook: &k8s_api::admissionregistration::v1::MutatingWebhook,
) -> Result<k8s_api::admissionregistration::v1beta1::MutatingWebhook, ConversionError> {
    Ok(k8s_api::admissionregistration::v1beta1::MutatingWebhook {
        name: webhook.name.clone(),
        client_config: convert_via_json(&webhook.client_config)?,
        rules: webhook.rules.clone(),
        failure_policy: webhook.failure_policy.clone(),
        match_policy: webhook.match_policy.clone(),
        namespace_selector: webhook.namespace_selector.clone(),
        object_selector: webhook.object_selector.clone(),
        side_effects: Some(webhook.side_effects.clone()),
        timeout_seconds: webhook.timeout_seconds,
        admission_review_versions: webhook.admission_review_versions.clone(),
        reinvocation_policy: webhook.reinvocation_policy.clone(),
        match_conditions: convert_via_json(&webhook.match_conditions)?,
    })
}

// =============================================================================
// ValidatingWebhookConfiguration: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingWebhookConfiguration>
    for k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfiguration
{
    fn convert_to(
        &self,
    )
        -> Result<k8s_api::admissionregistration::v1::ValidatingWebhookConfiguration, ConversionError>
    {
        let webhooks = self
            .webhooks
            .iter()
            .map(convert_validating_webhook_to_v1)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingWebhookConfiguration {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingWebhookConfiguration",
            ),
            metadata: self.metadata.clone(),
            webhooks,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingWebhookConfiguration,
    ) -> Result<Self, ConversionError> {
        let webhooks = other
            .webhooks
            .iter()
            .map(convert_validating_webhook_from_v1)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "ValidatingWebhookConfiguration",
            ),
            metadata: other.metadata.clone(),
            webhooks,
        })
    }
}

// =============================================================================
// ValidatingWebhookConfigurationList: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingWebhookConfigurationList>
    for k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfigurationList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1::ValidatingWebhookConfigurationList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingWebhookConfigurationList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingWebhookConfigurationList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingWebhookConfigurationList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1beta1::ValidatingWebhookConfiguration::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "ValidatingWebhookConfigurationList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

fn convert_validating_webhook_to_v1(
    webhook: &k8s_api::admissionregistration::v1beta1::ValidatingWebhook,
) -> Result<k8s_api::admissionregistration::v1::ValidatingWebhook, ConversionError> {
    let side_effects = webhook.side_effects.clone().ok_or_else(|| {
        ConversionError::MissingField("validatingWebhook.sideEffects".to_string())
    })?;
    Ok(k8s_api::admissionregistration::v1::ValidatingWebhook {
        name: webhook.name.clone(),
        client_config: convert_via_json(&webhook.client_config)?,
        rules: webhook.rules.clone(),
        failure_policy: webhook.failure_policy.clone(),
        match_policy: webhook.match_policy.clone(),
        namespace_selector: webhook.namespace_selector.clone(),
        object_selector: webhook.object_selector.clone(),
        side_effects,
        timeout_seconds: webhook.timeout_seconds,
        admission_review_versions: webhook.admission_review_versions.clone(),
        match_conditions: convert_via_json(&webhook.match_conditions)?,
    })
}

fn convert_validating_webhook_from_v1(
    webhook: &k8s_api::admissionregistration::v1::ValidatingWebhook,
) -> Result<k8s_api::admissionregistration::v1beta1::ValidatingWebhook, ConversionError> {
    Ok(k8s_api::admissionregistration::v1beta1::ValidatingWebhook {
        name: webhook.name.clone(),
        client_config: convert_via_json(&webhook.client_config)?,
        rules: webhook.rules.clone(),
        failure_policy: webhook.failure_policy.clone(),
        match_policy: webhook.match_policy.clone(),
        namespace_selector: webhook.namespace_selector.clone(),
        object_selector: webhook.object_selector.clone(),
        side_effects: Some(webhook.side_effects.clone()),
        timeout_seconds: webhook.timeout_seconds,
        admission_review_versions: webhook.admission_review_versions.clone(),
        match_conditions: convert_via_json(&webhook.match_conditions)?,
    })
}

// =============================================================================
// ValidatingAdmissionPolicy: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy>
    for k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicy
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1",
            "ValidatingAdmissionPolicy",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicy =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1beta1",
            "ValidatingAdmissionPolicy",
        );
        Ok(converted)
    }
}

// =============================================================================
// ValidatingAdmissionPolicyList: v1beta1/v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList>
    for k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList, ConversionError>
    {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingAdmissionPolicyList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicy::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "ValidatingAdmissionPolicyList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList>
    for k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList, ConversionError>
    {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingAdmissionPolicyList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1alpha1",
                "ValidatingAdmissionPolicyList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy>
    for k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1",
            "ValidatingAdmissionPolicy",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1alpha1",
            "ValidatingAdmissionPolicy",
        );
        Ok(converted)
    }
}

// =============================================================================
// ValidatingAdmissionPolicyBinding: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding>
    for k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBinding
{
    fn convert_to(
        &self,
    )
        -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding, ConversionError>
    {
        let mut converted: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1",
            "ValidatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBinding =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1beta1",
            "ValidatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }
}

// =============================================================================
// ValidatingAdmissionPolicyBindingList: v1beta1/v1alpha1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList>
    for k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingAdmissionPolicyBindingList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBinding::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "ValidatingAdmissionPolicyBindingList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList>
    for k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBindingList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1",
                "ValidatingAdmissionPolicyBindingList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBindingList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBinding::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1alpha1",
                "ValidatingAdmissionPolicyBindingList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding>
    for k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBinding
{
    fn convert_to(
        &self,
    )
        -> Result<k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding, ConversionError>
    {
        let mut converted: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1",
            "ValidatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyBinding,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBinding =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1alpha1",
            "ValidatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }
}

// =============================================================================
// MutatingAdmissionPolicy: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicy>
    for k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicy
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicy, ConversionError>
    {
        let mut converted: k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicy =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1beta1",
            "MutatingAdmissionPolicy",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicy,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicy =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1alpha1",
            "MutatingAdmissionPolicy",
        );
        Ok(converted)
    }
}

// =============================================================================
// MutatingAdmissionPolicyList: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyList>
    for k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "MutatingAdmissionPolicyList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicy::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1alpha1",
                "MutatingAdmissionPolicyList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// MutatingAdmissionPolicyBinding: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding>
    for k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBinding
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding, ConversionError>
    {
        let mut converted: k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1beta1",
            "MutatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBinding =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admissionregistration.k8s.io/v1alpha1",
            "MutatingAdmissionPolicyBinding",
        );
        Ok(converted)
    }
}

// =============================================================================
// MutatingAdmissionPolicyBindingList: v1alpha1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingList>
    for k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingList
{
    fn convert_to(
        &self,
    ) -> Result<
        k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingList,
        ConversionError,
    > {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1beta1",
                "MutatingAdmissionPolicyBindingList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBinding::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "admissionregistration.k8s.io/v1alpha1",
                "MutatingAdmissionPolicyBindingList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta};

    #[test]
    fn test_validating_admission_policy_v1alpha1_to_v1() {
        let v1alpha1 = k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy {
            metadata: ObjectMeta::named("alpha-policy"),
            ..Default::default()
        };

        let v1: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicy =
            v1alpha1.convert_to().unwrap();
        assert_eq!(v1.metadata.name, "alpha-policy");
    }

    #[test]
    fn test_mutating_admission_policy_alpha_to_beta() {
        let v1alpha1 = k8s_api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
            metadata: ObjectMeta::named("alpha-mutation"),
            ..Default::default()
        };

        let v1beta1: k8s_api::admissionregistration::v1beta1::MutatingAdmissionPolicy =
            v1alpha1.convert_to().unwrap();
        assert_eq!(v1beta1.metadata.name, "alpha-mutation");
    }

    #[test]
    fn test_validating_admission_policy_list_roundtrip() {
        let list = k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyList {
            metadata: ListMeta {
                resource_version: "10".to_string(),
                ..Default::default()
            },
            items: vec![k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicy {
                metadata: ObjectMeta::named("policy"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let v1_list: k8s_api::admissionregistration::v1::ValidatingAdmissionPolicyList =
            list.convert_to().unwrap();
        assert_eq!(v1_list.items.len(), 1);
        assert_eq!(v1_list.items[0].metadata.name, "policy");
        assert_eq!(v1_list.metadata.resource_version, "10");

        let roundtrip: k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyList =
            k8s_api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyList::convert_from(
                &v1_list,
            )
            .unwrap();
        assert_eq!(roundtrip.items.len(), 1);
    }
}
