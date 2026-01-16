//! Certificates API validation
//!
//! This module provides validation for certificates API types including:
//! - CertificateSigningRequest

use crate::common::{validate_dns_subdomain_name, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::certificates::v1::{CertificateSigningRequest, CertificateSigningRequestSpec};

/// Valid key usages for certificate signing requests
const VALID_KEY_USAGES: &[&str] = &[
    "signing",
    "digital signature",
    "content commitment",
    "key encipherment",
    "key agreement",
    "data encipherment",
    "cert sign",
    "crl sign",
    "encipher only",
    "decipher only",
    "any",
    "server auth",
    "client auth",
    "code signing",
    "email protection",
    "s/mime",
    "ipsec end system",
    "ipsec tunnel",
    "ipsec user",
    "timestamping",
    "ocsp signing",
    "microsoft sgc",
    "netscape sgc",
];

/// Maximum length for signer name
const MAX_SIGNER_NAME_LENGTH: usize = 571; // 253 (domain) + 1 (/) + 63 (name) + 253 (path) + 1 (extra)

/// Maximum size for the request (PEM-encoded CSR) in bytes
const MAX_REQUEST_SIZE: usize = 3 * 1024 * 1024; // 3 MB

/// Minimum expiration seconds (10 minutes)
const MIN_EXPIRATION_SECONDS: i32 = 600;

/// Maximum expiration seconds (about 2 years in seconds - safe for i32)
const MAX_EXPIRATION_SECONDS: i32 = 2 * 365 * 24 * 60 * 60; // ~63 million

// =============================================================================
// CertificateSigningRequest Validation
// =============================================================================

/// Validates a CertificateSigningRequest resource.
pub fn validate_certificate_signing_request(csr: &CertificateSigningRequest) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&csr.metadata, "metadata", true));

    // Validate spec
    errors.extend(validate_csr_spec(&csr.spec, "spec"));

    errors
}

pub mod internal {
    use super::*;
    use k8s_api::certificates::internal as api;

    pub fn validate_certificate_signing_request(
        csr: &api::CertificateSigningRequest,
    ) -> ValidationResult {
        crate::internal::validate_with(
            csr,
            "certificateSigningRequest",
            super::validate_certificate_signing_request,
        )
    }
}

/// Validates CertificateSigningRequestSpec.
fn validate_csr_spec(spec: &CertificateSigningRequestSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Request is required
    if spec.request.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.request", field),
            "request is required",
        ));
    } else {
        // Validate request size
        if spec.request.len() > MAX_REQUEST_SIZE {
            errors.push(ValidationError::too_long(
                format!("{}.request", field),
                MAX_REQUEST_SIZE,
                spec.request.len(),
            ));
        }

        // Validate that request looks like a PEM-encoded CSR
        if !is_valid_pem_csr(&spec.request) {
            errors.push(ValidationError::invalid(
                format!("{}.request", field),
                "must be a valid PEM-encoded certificate signing request",
            ));
        }
    }

    // SignerName is required
    if spec.signer_name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.signerName", field),
            "signerName is required",
        ));
    } else {
        errors.extend(validate_signer_name(
            &spec.signer_name,
            &format!("{}.signerName", field),
        ));
    }

    // Validate expirationSeconds if present
    if let Some(expiration) = spec.expiration_seconds {
        if expiration < MIN_EXPIRATION_SECONDS {
            errors.push(ValidationError::out_of_range(
                format!("{}.expirationSeconds", field),
                MIN_EXPIRATION_SECONDS as i64,
                MAX_EXPIRATION_SECONDS as i64,
                expiration as i64,
            ));
        }
        if expiration > MAX_EXPIRATION_SECONDS {
            errors.push(ValidationError::out_of_range(
                format!("{}.expirationSeconds", field),
                MIN_EXPIRATION_SECONDS as i64,
                MAX_EXPIRATION_SECONDS as i64,
                expiration as i64,
            ));
        }
    }

    // Validate usages
    let mut seen_usages = std::collections::HashSet::new();
    for (i, usage) in spec.usages.iter().enumerate() {
        let usage_lower = usage.to_lowercase();
        if !VALID_KEY_USAGES.contains(&usage_lower.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.usages[{}]", field, i),
                usage,
                VALID_KEY_USAGES,
            ));
        }
        if !seen_usages.insert(usage_lower.clone()) {
            errors.push(ValidationError::duplicate(
                format!("{}.usages[{}]", field, i),
                usage,
            ));
        }
    }

    errors
}

/// Validates the signer name.
fn validate_signer_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.len() > MAX_SIGNER_NAME_LENGTH {
        errors.push(ValidationError::too_long(
            field,
            MAX_SIGNER_NAME_LENGTH,
            name.len(),
        ));
    }

    // Signer name should be in the format: <domain>/<name>
    // e.g., "kubernetes.io/kube-apiserver-client"
    if let Some(slash_idx) = name.find('/') {
        let domain = &name[..slash_idx];
        let signer = &name[slash_idx + 1..];

        // Validate domain as DNS subdomain
        if domain.is_empty() {
            errors.push(ValidationError::required(
                format!("{} (domain)", field),
                "signer domain is required",
            ));
        } else {
            errors.extend(validate_dns_subdomain_name(
                domain,
                &format!("{} (domain)", field),
            ));
        }

        // Validate signer name part
        if signer.is_empty() {
            errors.push(ValidationError::required(
                format!("{} (name)", field),
                "signer name is required",
            ));
        } else {
            // Signer name can contain path segments
            for segment in signer.split('/') {
                if segment.is_empty() {
                    errors.push(ValidationError::invalid(
                        format!("{} (path)", field),
                        "path segments cannot be empty",
                    ));
                }
            }
        }
    } else {
        errors.push(ValidationError::invalid(
            field,
            "signerName must be in the format '<domain>/<name>' (e.g., 'kubernetes.io/kube-apiserver-client')",
        ));
    }

    errors
}

/// Checks if a string looks like a valid PEM-encoded CSR.
fn is_valid_pem_csr(request: &str) -> bool {
    // A valid PEM CSR should contain the header and footer
    let has_begin = request.contains("-----BEGIN CERTIFICATE REQUEST-----")
        || request.contains("-----BEGIN NEW CERTIFICATE REQUEST-----");
    let has_end = request.contains("-----END CERTIFICATE REQUEST-----")
        || request.contains("-----END NEW CERTIFICATE REQUEST-----");

    has_begin && has_end
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    fn sample_pem_csr() -> String {
        // A sample (invalid but structurally correct) PEM CSR
        "-----BEGIN CERTIFICATE REQUEST-----\n\
         MIICijCCAXICAQAwRTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUx\n\
         ITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDCCASIwDQYJKoZIhvcN\n\
         AQEBBQADggEPADCCAQoCggEBAMabnSPxruPP2HIDZhPK4ElxRnIV7ZbIcQQyPvhP\n\
         MFjjhNqQOuoVoAjGMjJlSFENQvnsqVCDWwB51BjGJ7OHLZIc8a5S2JmkJ5qB8mUF\n\
         -----END CERTIFICATE REQUEST-----"
            .to_string()
    }

    #[test]
    fn test_validate_csr_valid() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                usages: vec!["client auth".to_string(), "digital signature".to_string()],
                expiration_seconds: Some(86400), // 1 day
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csr_missing_request() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: String::new(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("request")));
    }

    #[test]
    fn test_validate_csr_invalid_pem() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: "not a valid PEM".to_string(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("request")));
    }

    #[test]
    fn test_validate_csr_missing_signer_name() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: String::new(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("signerName")));
    }

    #[test]
    fn test_validate_csr_invalid_signer_name_format() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "no-slash-in-name".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("signerName")));
    }

    #[test]
    fn test_validate_csr_invalid_usage() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                usages: vec!["invalid-usage".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("usages")));
    }

    #[test]
    fn test_validate_csr_duplicate_usage() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                usages: vec![
                    "client auth".to_string(),
                    "Client Auth".to_string(), // Duplicate (case insensitive)
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn test_validate_csr_expiration_too_low() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "kubernetes.io/kube-apiserver-client".to_string(),
                expiration_seconds: Some(60), // 1 minute - too low
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("expirationSeconds")));
    }

    #[test]
    fn test_validate_csr_custom_signer() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "my-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "my-company.com/internal-ca".to_string(),
                usages: vec!["server auth".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csr_with_all_valid_usages() {
        let csr = CertificateSigningRequest {
            metadata: ObjectMeta {
                name: "full-csr".to_string(),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: sample_pem_csr(),
                signer_name: "kubernetes.io/kubelet-serving".to_string(),
                usages: vec![
                    "signing".to_string(),
                    "digital signature".to_string(),
                    "key encipherment".to_string(),
                    "server auth".to_string(),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_certificate_signing_request(&csr);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_is_valid_pem_csr() {
        assert!(is_valid_pem_csr(
            "-----BEGIN CERTIFICATE REQUEST-----\ndata\n-----END CERTIFICATE REQUEST-----"
        ));
        assert!(is_valid_pem_csr(
            "-----BEGIN NEW CERTIFICATE REQUEST-----\ndata\n-----END NEW CERTIFICATE REQUEST-----"
        ));
        assert!(!is_valid_pem_csr("not a pem"));
        assert!(!is_valid_pem_csr(
            "-----BEGIN CERTIFICATE REQUEST-----\nno end"
        ));
    }
}
