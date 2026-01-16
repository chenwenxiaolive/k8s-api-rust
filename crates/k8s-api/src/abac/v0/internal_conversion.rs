use super::*;

impl InternalConversion for Policy {
    type Internal = crate::abac::internal::Policy;
}
