use crate::core::v1::PREEMPT_LOWER_PRIORITY;

use super::PriorityClass;

pub fn apply_defaults_priority_class(class: &mut PriorityClass) {
    if class.preemption_policy.is_none() {
        class.preemption_policy = Some(PREEMPT_LOWER_PRIORITY.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_priority_class_preemption_policy() {
        let mut class = PriorityClass::default();
        apply_defaults_priority_class(&mut class);
        assert_eq!(
            class.preemption_policy.as_deref(),
            Some(PREEMPT_LOWER_PRIORITY)
        );
    }
}
