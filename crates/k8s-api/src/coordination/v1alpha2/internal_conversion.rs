use super::*;

impl InternalConversion for LeaseCandidate {
    type Internal = crate::coordination::internal::LeaseCandidate;
}

impl InternalConversion for LeaseCandidateList {
    type Internal = crate::coordination::internal::LeaseCandidateList;
}

impl InternalConversion for LeaseCandidateSpec {
    type Internal = crate::coordination::internal::LeaseCandidateSpec;
}
