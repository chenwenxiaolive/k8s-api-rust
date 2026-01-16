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

impl InternalConversion for LeaseCandidate {
    type Internal = crate::coordination::internal::LeaseCandidate;
}

impl InternalConversion for LeaseCandidateList {
    type Internal = crate::coordination::internal::LeaseCandidateList;
}

impl InternalConversion for LeaseCandidateSpec {
    type Internal = crate::coordination::internal::LeaseCandidateSpec;
}
