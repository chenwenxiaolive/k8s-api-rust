use super::*;

impl InternalConversion for RuntimeClass {
    type Internal = crate::node::internal::RuntimeClass;
}

impl InternalConversion for Overhead {
    type Internal = crate::node::internal::Overhead;
}

impl InternalConversion for Scheduling {
    type Internal = crate::node::internal::Scheduling;
}

impl InternalConversion for RuntimeClassList {
    type Internal = crate::node::internal::RuntimeClassList;
}
