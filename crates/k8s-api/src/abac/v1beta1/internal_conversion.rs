use super::*;

impl InternalConversion for Policy {
    type Internal = crate::abac::internal::Policy;
}

impl InternalConversion for PolicySpec {
    type Internal = crate::abac::internal::PolicySpec;
}
