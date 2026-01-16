use super::*;

impl InternalConversion for Lease {
    type Internal = crate::coordination::internal::Lease;
}

impl InternalConversion for LeaseList {
    type Internal = crate::coordination::internal::LeaseList;
}

impl InternalConversion for LeaseSpec {
    type Internal = crate::coordination::internal::LeaseSpec;
}
