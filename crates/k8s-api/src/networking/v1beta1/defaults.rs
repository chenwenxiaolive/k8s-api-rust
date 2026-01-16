use super::{HTTPIngressPath, Ingress, PATH_TYPE_IMPLEMENTATION_SPECIFIC};

pub fn apply_defaults_ingress(ingress: &mut Ingress) {
    if let Some(spec) = ingress.spec.as_mut() {
        for rule in &mut spec.rules {
            if let Some(http) = rule.ingress_rule_value.http.as_mut() {
                for path in &mut http.paths {
                    apply_defaults_http_ingress_path(path);
                }
            }
        }
    }
}

fn apply_defaults_http_ingress_path(path: &mut HTTPIngressPath) {
    if path.path_type.is_none() {
        path.path_type = Some(PATH_TYPE_IMPLEMENTATION_SPECIFIC.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::v1beta1::{
        HTTPIngressRuleValue, IngressRule, IngressRuleValue, IngressSpec, PATH_TYPE_EXACT,
    };

    fn wrap_path(path: HTTPIngressPath) -> Ingress {
        Ingress {
            spec: Some(IngressSpec {
                rules: vec![IngressRule {
                    host: String::new(),
                    ingress_rule_value: IngressRuleValue {
                        http: Some(HTTPIngressRuleValue { paths: vec![path] }),
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_default_ingress_path_type() {
        let mut ingress = wrap_path(HTTPIngressPath::default());
        apply_defaults_ingress(&mut ingress);
        let path = &ingress
            .spec
            .as_ref()
            .unwrap()
            .rules[0]
            .ingress_rule_value
            .http
            .as_ref()
            .unwrap()
            .paths[0];
        assert_eq!(
            path.path_type.as_deref(),
            Some(PATH_TYPE_IMPLEMENTATION_SPECIFIC)
        );
    }

    #[test]
    fn test_default_ingress_path_type_preserve() {
        let mut ingress = wrap_path(HTTPIngressPath {
            path_type: Some(PATH_TYPE_EXACT.to_string()),
            ..Default::default()
        });
        apply_defaults_ingress(&mut ingress);
        let path = &ingress
            .spec
            .as_ref()
            .unwrap()
            .rules[0]
            .ingress_rule_value
            .http
            .as_ref()
            .unwrap()
            .paths[0];
        assert_eq!(path.path_type.as_deref(), Some(PATH_TYPE_EXACT));
    }
}
