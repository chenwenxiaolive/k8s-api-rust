use super::*;

impl InternalConversion for PriorityClass {
    type Internal = crate::scheduling::internal::PriorityClass;
}

impl InternalConversion for PriorityClassList {
    type Internal = crate::scheduling::internal::PriorityClassList;
}
