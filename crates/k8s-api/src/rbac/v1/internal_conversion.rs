use super::*;

impl InternalConversion for Role {
    type Internal = crate::rbac::internal::Role;
}

impl InternalConversion for RoleList {
    type Internal = crate::rbac::internal::RoleList;
}

impl InternalConversion for ClusterRole {
    type Internal = crate::rbac::internal::ClusterRole;
}

impl InternalConversion for ClusterRoleList {
    type Internal = crate::rbac::internal::ClusterRoleList;
}

impl InternalConversion for AggregationRule {
    type Internal = crate::rbac::internal::AggregationRule;
}

impl InternalConversion for RoleBinding {
    type Internal = crate::rbac::internal::RoleBinding;
}

impl InternalConversion for RoleBindingList {
    type Internal = crate::rbac::internal::RoleBindingList;
}

impl InternalConversion for ClusterRoleBinding {
    type Internal = crate::rbac::internal::ClusterRoleBinding;
}

impl InternalConversion for ClusterRoleBindingList {
    type Internal = crate::rbac::internal::ClusterRoleBindingList;
}

impl InternalConversion for PolicyRule {
    type Internal = crate::rbac::internal::PolicyRule;
}

impl InternalConversion for Subject {
    type Internal = crate::rbac::internal::Subject;
}

impl InternalConversion for RoleRef {
    type Internal = crate::rbac::internal::RoleRef;
}
