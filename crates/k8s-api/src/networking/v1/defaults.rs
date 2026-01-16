use super::{NetworkPolicy, NetworkPolicySpec, POLICY_TYPE_EGRESS, POLICY_TYPE_INGRESS};

pub fn apply_defaults_network_policy(policy: &mut NetworkPolicy) {
    if let Some(spec) = policy.spec.as_mut() {
        apply_defaults_network_policy_spec(spec);
    }
}

fn apply_defaults_network_policy_spec(spec: &mut NetworkPolicySpec) {
    if spec.policy_types.is_empty() {
        if spec.egress.is_empty() {
            spec.policy_types = vec![POLICY_TYPE_INGRESS.to_string()];
        } else {
            spec.policy_types = vec![
                POLICY_TYPE_INGRESS.to_string(),
                POLICY_TYPE_EGRESS.to_string(),
            ];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::LabelSelector;

    #[test]
    fn test_default_network_policy_empty() {
        let mut policy = NetworkPolicy {
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_network_policy(&mut policy);
        assert_eq!(
            policy.spec.as_ref().unwrap().policy_types,
            vec![POLICY_TYPE_INGRESS.to_string()]
        );
    }

    #[test]
    fn test_default_network_policy_ingress_only() {
        let mut policy = NetworkPolicy {
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_network_policy(&mut policy);
        assert_eq!(
            policy.spec.as_ref().unwrap().policy_types,
            vec![POLICY_TYPE_INGRESS.to_string()]
        );
    }

    #[test]
    fn test_default_network_policy_ingress_and_egress() {
        let mut policy = NetworkPolicy {
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![Default::default()],
                egress: vec![Default::default()],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_network_policy(&mut policy);
        assert_eq!(
            policy.spec.as_ref().unwrap().policy_types,
            vec![
                POLICY_TYPE_INGRESS.to_string(),
                POLICY_TYPE_EGRESS.to_string(),
            ]
        );
    }

    #[test]
    fn test_default_network_policy_egress_only() {
        let mut policy = NetworkPolicy {
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                egress: vec![Default::default()],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_network_policy(&mut policy);
        assert_eq!(
            policy.spec.as_ref().unwrap().policy_types,
            vec![
                POLICY_TYPE_INGRESS.to_string(),
                POLICY_TYPE_EGRESS.to_string(),
            ]
        );
    }
}
