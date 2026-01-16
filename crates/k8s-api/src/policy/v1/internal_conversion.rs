use super::*;

impl InternalConversion for PodDisruptionBudget {
    type Internal = crate::policy::internal::PodDisruptionBudget;
}

impl InternalConversion for PodDisruptionBudgetList {
    type Internal = crate::policy::internal::PodDisruptionBudgetList;
}

impl InternalConversion for PodDisruptionBudgetSpec {
    type Internal = crate::policy::internal::PodDisruptionBudgetSpec;
}

impl InternalConversion for PodDisruptionBudgetStatus {
    type Internal = crate::policy::internal::PodDisruptionBudgetStatus;
}

impl InternalConversion for Eviction {
    type Internal = crate::policy::internal::Eviction;
}
