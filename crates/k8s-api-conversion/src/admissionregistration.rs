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
