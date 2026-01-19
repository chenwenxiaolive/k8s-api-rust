use crate::core::helper::quantity;
use chrono::{DateTime, SecondsFormat, Utc};
use k8s_apimachinery::apis::meta::v1::Time;
use std::collections::BTreeMap;
#[cfg(feature = "apps")]
use std::collections::HashMap;

use super::*;

pub type LabelSet = BTreeMap<String, String>;
pub type QueryParams = BTreeMap<String, Vec<String>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologySelector {
    match_nothing: bool,
    requirements: Vec<(String, Vec<String>)>,
}

impl TopologySelector {
    pub fn matches(&self, labels: &LabelSet) -> bool {
        if self.match_nothing {
            return false;
        }
        for (key, values) in &self.requirements {
            let label_value = match labels.get(key) {
                Some(value) => value,
                None => return false,
            };
            if !values.iter().any(|candidate| candidate == label_value) {
                return false;
            }
        }
        true
    }

    pub fn is_nothing(&self) -> bool {
        self.match_nothing
    }
}

pub fn is_prefixed_native_resource(name: &str) -> bool {
    name.contains(RESOURCE_DEFAULT_NAMESPACE_PREFIX)
}

pub fn is_native_resource(name: &str) -> bool {
    !name.contains('/') || is_prefixed_native_resource(name)
}

pub fn is_huge_page_resource_name(name: &str) -> bool {
    name.starts_with(RESOURCE_HUGE_PAGES_PREFIX)
}

pub fn huge_page_resource_name(page_size: &k8s_api_core::resource::Quantity) -> String {
    format!("{}{}", RESOURCE_HUGE_PAGES_PREFIX, page_size.as_str())
}

pub fn huge_page_size_from_resource_name(name: &str) -> Result<k8s_api_core::resource::Quantity, String> {
    if !is_huge_page_resource_name(name) {
        return Err(format!("resource name: {name} is an invalid hugepage name"));
    }
    let page_size = name
        .strip_prefix(RESOURCE_HUGE_PAGES_PREFIX)
        .unwrap_or("");
    if page_size.is_empty() {
        return Err(format!("resource name: {name} is an invalid hugepage name"));
    }
    quantity::parse_quantity_rational(page_size)
        .ok_or_else(|| format!("resource name: {name} is an invalid hugepage name"))?;
    Ok(k8s_api_core::resource::Quantity::new(page_size))
}

pub fn huge_page_unit_size_from_byte_size(size: i64) -> Result<String, String> {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size;
    let mut idx = 0usize;
    let last = units.len().saturating_sub(1);
    while size % 1024 == 0 && idx < last {
        size /= 1024;
        idx += 1;
    }
    if size > 1024 && idx < last {
        return Err(format!(
            "size: {size}{} must be guaranteed to divisible into the largest units",
            units[idx]
        ));
    }
    Ok(format!("{}{}", size, units[idx]))
}

pub fn is_huge_page_medium(medium: &StorageMedium) -> bool {
    medium == STORAGE_MEDIUM_HUGE_PAGES || medium.starts_with(STORAGE_MEDIUM_HUGE_PAGES_PREFIX)
}

pub fn huge_page_size_from_medium(
    medium: &StorageMedium,
) -> Result<k8s_api_core::resource::Quantity, String> {
    if !is_huge_page_medium(medium) {
        return Err(format!("medium: {medium} is not a hugepage medium"));
    }
    if medium == STORAGE_MEDIUM_HUGE_PAGES {
        return Err(format!("medium: {medium} doesn't have size information"));
    }
    let page_size = medium
        .strip_prefix(STORAGE_MEDIUM_HUGE_PAGES_PREFIX)
        .unwrap_or("");
    quantity::parse_quantity_rational(page_size)
        .ok_or_else(|| format!("medium: {medium} doesn't have size information"))?;
    Ok(k8s_api_core::resource::Quantity::new(page_size))
}

pub fn is_overcommit_allowed(name: &str) -> bool {
    is_native_resource(name) && !is_huge_page_resource_name(name)
}

pub fn is_attachable_volume_resource_name(name: &str) -> bool {
    name.starts_with(RESOURCE_ATTACHABLE_VOLUMES_PREFIX)
}

pub fn is_service_ip_set(service: &Service) -> bool {
    let spec = match service.spec.as_ref() {
        Some(spec) => spec,
        None => return false,
    };
    spec.cluster_i_p != CLUSTER_IP_NONE && !spec.cluster_i_p.is_empty()
}

pub fn get_access_modes_from_string(modes: &str) -> Vec<PersistentVolumeAccessMode> {
    let mut access_modes = Vec::new();
    for raw in modes.split(',') {
        let mode = raw.trim();
        match mode {
            "RWO" => access_modes.push(PV_ACCESS_READ_WRITE_ONCE.to_string()),
            "ROX" => access_modes.push(PV_ACCESS_READ_ONLY_MANY.to_string()),
            "RWX" => access_modes.push(PV_ACCESS_READ_WRITE_MANY.to_string()),
            "RWOP" => access_modes.push(PV_ACCESS_READ_WRITE_ONCE_POD.to_string()),
            _ => {}
        }
    }
    access_modes
}

pub fn contains_access_mode(modes: &[PersistentVolumeAccessMode], mode: &str) -> bool {
    modes.iter().any(|candidate| candidate == mode)
}

pub fn remove_duplicate_access_modes(
    modes: &[PersistentVolumeAccessMode],
) -> Vec<PersistentVolumeAccessMode> {
    let mut access_modes = Vec::new();
    for mode in modes {
        if !contains_access_mode(&access_modes, mode) {
            access_modes.push(mode.clone());
        }
    }
    access_modes
}

pub fn node_selector_requirement_keys_exist_in_node_selector_terms(
    reqs: &[NodeSelectorRequirement],
    terms: &[NodeSelectorTerm],
) -> bool {
    for req in reqs {
        for term in terms {
            for expression in &term.match_expressions {
                if expression.key == req.key {
                    return true;
                }
            }
        }
    }
    false
}

pub fn topology_selector_requirements_as_selector(
    requirements: &[TopologySelectorLabelRequirement],
) -> Result<TopologySelector, String> {
    if requirements.is_empty() {
        return Ok(TopologySelector {
            match_nothing: true,
            requirements: Vec::new(),
        });
    }

    let mut selector_requirements = Vec::with_capacity(requirements.len());
    for requirement in requirements {
        if requirement.values.is_empty() {
            return Err("topology selector values must not be empty".to_string());
        }
        if !is_valid_label_key(&requirement.key) {
            return Err(format!("invalid topology selector key: {}", requirement.key));
        }
        selector_requirements.push((requirement.key.clone(), requirement.values.clone()));
    }

    Ok(TopologySelector {
        match_nothing: false,
        requirements: selector_requirements,
    })
}

pub fn match_topology_selector_terms(terms: &[TopologySelectorTerm], labels: &LabelSet) -> bool {
    if terms.is_empty() {
        return true;
    }

    for term in terms {
        if term.match_label_expressions.is_empty() {
            continue;
        }
        let selector = match topology_selector_requirements_as_selector(&term.match_label_expressions) {
            Ok(selector) => selector,
            Err(_) => continue,
        };
        if selector.matches(labels) {
            return true;
        }
    }

    false
}

pub fn pod_log_options_to_params(options: &PodLogOptions) -> QueryParams {
    let mut params = QueryParams::new();
    if !options.container.is_empty() {
        push_param(&mut params, "container", &options.container);
    }
    if options.follow {
        push_param(&mut params, "follow", "true");
    }
    if options.previous {
        push_param(&mut params, "previous", "true");
    }
    if let Some(seconds) = options.since_seconds {
        push_param(&mut params, "sinceSeconds", seconds);
    }
    if let Some(time) = options.since_time.as_ref().and_then(format_time) {
        push_param(&mut params, "sinceTime", time);
    }
    if options.timestamps {
        push_param(&mut params, "timestamps", "true");
    }
    if let Some(lines) = options.tail_lines {
        push_param(&mut params, "tailLines", lines);
    }
    if let Some(bytes) = options.limit_bytes {
        push_param(&mut params, "limitBytes", bytes);
    }
    if options.insecure_skip_tls_verify_backend {
        push_param(&mut params, "insecureSkipTLSVerifyBackend", "true");
    }
    if let Some(stream) = options.stream.as_ref() {
        if !stream.is_empty() {
            push_param(&mut params, "stream", stream);
        }
    }
    params
}

pub fn pod_log_options_from_params(params: &QueryParams) -> Result<PodLogOptions, String> {
    let mut options = PodLogOptions::default();
    if let Some(value) = get_param(params, "container") {
        options.container = value.to_string();
    }
    if let Some(value) = get_param(params, "follow") {
        options.follow = parse_bool(value, "follow")?;
    }
    if let Some(value) = get_param(params, "previous") {
        options.previous = parse_bool(value, "previous")?;
    }
    if let Some(value) = get_param(params, "sinceSeconds") {
        options.since_seconds = Some(parse_i64(value, "sinceSeconds")?);
    }
    if let Some(value) = get_param(params, "sinceTime") {
        options.since_time = Some(parse_time(value, "sinceTime")?);
    }
    if let Some(value) = get_param(params, "timestamps") {
        options.timestamps = parse_bool(value, "timestamps")?;
    }
    if let Some(value) = get_param(params, "tailLines") {
        options.tail_lines = Some(parse_i64(value, "tailLines")?);
    }
    if let Some(value) = get_param(params, "limitBytes") {
        options.limit_bytes = Some(parse_i64(value, "limitBytes")?);
    }
    if let Some(value) = get_param(params, "insecureSkipTLSVerifyBackend") {
        options.insecure_skip_tls_verify_backend =
            parse_bool(value, "insecureSkipTLSVerifyBackend")?;
    }
    if let Some(value) = get_param(params, "stream") {
        options.stream = Some(value.to_string());
    }
    Ok(options)
}

fn push_param(params: &mut QueryParams, key: &str, value: impl ToString) {
    params
        .entry(key.to_string())
        .or_default()
        .push(value.to_string());
}

fn get_param<'a>(params: &'a QueryParams, key: &str) -> Option<&'a str> {
    params
        .get(key)
        .and_then(|values| values.first())
        .map(String::as_str)
}

fn parse_bool(value: &str, key: &str) -> Result<bool, String> {
    if value.eq_ignore_ascii_case("true") {
        return Ok(true);
    }
    if value.eq_ignore_ascii_case("false") {
        return Ok(false);
    }
    Err(format!("invalid {key}: {value}"))
}

fn parse_i64(value: &str, key: &str) -> Result<i64, String> {
    value
        .parse::<i64>()
        .map_err(|_| format!("invalid {key}: {value}"))
}

fn format_time(time: &Time) -> Option<String> {
    time.0
        .as_ref()
        .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true))
}

fn parse_time(value: &str, key: &str) -> Result<Time, String> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map_err(|err| format!("invalid {key}: {err}"))?;
    Ok(Time(Some(parsed.with_timezone(&Utc))))
}

fn is_valid_label_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    if key.starts_with('/') || key.ends_with('/') {
        return false;
    }
    if key.contains("//") {
        return false;
    }
    true
}

#[cfg(feature = "apps")]
pub fn replication_controller_to_replica_set(
    controller: &ReplicationController,
) -> Result<crate::apps::v1::ReplicaSet, String> {
    let spec = controller
        .spec
        .as_ref()
        .map(replication_controller_spec_to_replica_set_spec);
    let status = controller
        .status
        .as_ref()
        .map(replication_controller_status_to_replica_set_status);
    Ok(crate::apps::v1::ReplicaSet {
        metadata: controller.metadata.clone(),
        spec,
        status,
        ..Default::default()
    })
}

#[cfg(feature = "apps")]
pub fn replica_set_to_replication_controller(
    replica_set: &crate::apps::v1::ReplicaSet,
) -> Result<ReplicationController, String> {
    let spec = match replica_set.spec.as_ref() {
        Some(spec) => Some(replica_set_spec_to_replication_controller_spec(spec)?),
        None => None,
    };
    let status = replica_set
        .status
        .as_ref()
        .map(replica_set_status_to_replication_controller_status);
    Ok(ReplicationController {
        metadata: replica_set.metadata.clone(),
        spec,
        status,
        ..Default::default()
    })
}

#[cfg(feature = "apps")]
fn replication_controller_spec_to_replica_set_spec(
    spec: &ReplicationControllerSpec,
) -> crate::apps::v1::ReplicaSetSpec {
    let selector = if spec.selector.is_empty() {
        None
    } else {
        Some(label_selector_from_map(&spec.selector))
    };
    let min_ready_seconds = if spec.min_ready_seconds == 0 {
        None
    } else {
        Some(spec.min_ready_seconds)
    };
    crate::apps::v1::ReplicaSetSpec {
        replicas: spec.replicas,
        min_ready_seconds,
        selector,
        template: spec.template.clone(),
    }
}

#[cfg(feature = "apps")]
fn replica_set_spec_to_replication_controller_spec(
    spec: &crate::apps::v1::ReplicaSetSpec,
) -> Result<ReplicationControllerSpec, String> {
    let selector = match spec.selector.as_ref() {
        Some(selector) => label_selector_as_map(selector)?,
        None => HashMap::new(),
    };
    Ok(ReplicationControllerSpec {
        replicas: spec.replicas,
        selector,
        template: spec.template.clone(),
        min_ready_seconds: spec.min_ready_seconds.unwrap_or_default(),
    })
}

#[cfg(feature = "apps")]
fn replication_controller_status_to_replica_set_status(
    status: &ReplicationControllerStatus,
) -> crate::apps::v1::ReplicaSetStatus {
    crate::apps::v1::ReplicaSetStatus {
        replicas: status.replicas,
        fully_labeled_replicas: optional_i32(status.fully_labeled_replicas),
        ready_replicas: optional_i32(status.ready_replicas),
        available_replicas: optional_i32(status.available_replicas),
        observed_generation: optional_i64(status.observed_generation),
        conditions: status
            .conditions
            .iter()
            .map(|condition| crate::apps::v1::ReplicaSetCondition {
                condition_type: condition.condition_type.clone(),
                status: condition.status.clone(),
                last_transition_time: condition.last_transition_time.clone(),
                reason: condition.reason.clone(),
                message: condition.message.clone(),
            })
            .collect(),
    }
}

#[cfg(feature = "apps")]
fn replica_set_status_to_replication_controller_status(
    status: &crate::apps::v1::ReplicaSetStatus,
) -> ReplicationControllerStatus {
    ReplicationControllerStatus {
        replicas: status.replicas,
        fully_labeled_replicas: status.fully_labeled_replicas.unwrap_or_default(),
        ready_replicas: status.ready_replicas.unwrap_or_default(),
        available_replicas: status.available_replicas.unwrap_or_default(),
        observed_generation: status.observed_generation.unwrap_or_default(),
        conditions: status
            .conditions
            .iter()
            .map(|condition| ReplicationControllerCondition {
                condition_type: condition.condition_type.clone(),
                status: condition.status.clone(),
                last_transition_time: condition.last_transition_time.clone(),
                reason: condition.reason.clone(),
                message: condition.message.clone(),
            })
            .collect(),
    }
}

#[cfg(feature = "apps")]
fn label_selector_from_map(
    selector: &HashMap<String, String>,
) -> k8s_apimachinery::apis::meta::v1::LabelSelector {
    let match_labels = selector
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    k8s_apimachinery::apis::meta::v1::LabelSelector {
        match_labels,
        match_expressions: Vec::new(),
    }
}

#[cfg(feature = "apps")]
fn label_selector_as_map(
    selector: &k8s_apimachinery::apis::meta::v1::LabelSelector,
) -> Result<HashMap<String, String>, String> {
    let mut out = HashMap::new();
    for (key, value) in &selector.match_labels {
        out.insert(key.clone(), value.clone());
    }
    for expr in &selector.match_expressions {
        match expr.operator.as_str() {
            "In" => {
                if expr.values.len() != 1 {
                    return Err(format!(
                        "operator \"{}\" without a single value cannot be converted into the old label selector format",
                        expr.operator
                    ));
                }
                out.insert(expr.key.clone(), expr.values[0].clone());
            }
            "NotIn" | "Exists" | "DoesNotExist" => {
                return Err(format!(
                    "operator \"{}\" cannot be converted into the old label selector format",
                    expr.operator
                ));
            }
            _ => {
                return Err(format!(
                    "\"{}\" is not a valid selector operator",
                    expr.operator
                ));
            }
        }
    }
    Ok(out)
}

#[cfg(feature = "apps")]
fn optional_i32(value: i32) -> Option<i32> {
    if value == 0 {
        None
    } else {
        Some(value)
    }
}

#[cfg(feature = "apps")]
fn optional_i64(value: i64) -> Option<i64> {
    if value == 0 {
        None
    } else {
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    #[cfg(feature = "apps")]
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_is_native_resource() {
        let cases = vec![
            ("pod.alpha.kubernetes.io/opaque-int-resource-foo", true),
            ("kubernetes.io/resource-foo", true),
            ("foo", true),
            ("a/b", false),
            ("", true),
        ];
        for (name, expected) in cases {
            assert_eq!(is_native_resource(name), expected, "resourceName input={name}");
        }
    }

    #[test]
    fn test_huge_page_size_from_resource_name() {
        let cases = vec![
            ("pod.alpha.kubernetes.io/opaque-int-resource-foo", None),
            ("hugepages-", None),
            ("hugepages-100m", Some("100m")),
            ("", None),
        ];
        for (index, (name, expected)) in cases.into_iter().enumerate() {
            let result = huge_page_size_from_resource_name(name).ok();
            let result = result.as_ref().map(|value| value.as_str());
            assert_eq!(result, expected, "case {index}");
        }
    }

    #[test]
    fn test_huge_page_size_from_medium() {
        let cases = vec![
            (STORAGE_MEDIUM_MEMORY.to_string(), true, None),
            (STORAGE_MEDIUM_MEMORY.to_string(), true, None),
            (STORAGE_MEDIUM_HUGE_PAGES.to_string(), true, None),
            (STORAGE_MEDIUM_HUGE_PAGES.to_string(), true, None),
            (
                format!("{}1Gi", STORAGE_MEDIUM_HUGE_PAGES_PREFIX),
                false,
                Some("1Gi"),
            ),
            (
                format!("{}2Mi", STORAGE_MEDIUM_HUGE_PAGES_PREFIX),
                false,
                Some("2Mi"),
            ),
            (
                format!("{}64Ki", STORAGE_MEDIUM_HUGE_PAGES_PREFIX),
                false,
                Some("64Ki"),
            ),
        ];

        for (index, (medium, expect_err, expected)) in cases.into_iter().enumerate() {
            let result = huge_page_size_from_medium(&medium);
            if expect_err {
                assert!(result.is_err(), "case {index}");
                continue;
            }
            let result = result.expect("quantity");
            assert_eq!(result.as_str(), expected.unwrap_or_default(), "case {index}");
        }
    }

    #[test]
    fn test_is_overcommit_allowed() {
        let cases = vec![
            ("pod.alpha.kubernetes.io/opaque-int-resource-foo", true),
            ("kubernetes.io/resource-foo", true),
            ("hugepages-100m", false),
            ("", true),
        ];
        for (name, expected) in cases {
            assert_eq!(is_overcommit_allowed(name), expected, "resourceName input={name}");
        }
    }

    #[test]
    fn test_get_access_modes_from_string() {
        let modes = get_access_modes_from_string("ROX");
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_ONLY_MANY));

        let modes = get_access_modes_from_string("ROX,RWX");
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_ONLY_MANY));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_MANY));

        let modes = get_access_modes_from_string("RWO,ROX,RWX");
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_ONCE));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_ONLY_MANY));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_MANY));

        let modes = get_access_modes_from_string("RWO,ROX,RWX,RWOP");
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_ONCE));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_ONLY_MANY));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_MANY));
        assert!(contains_access_mode(&modes, PV_ACCESS_READ_WRITE_ONCE_POD));
    }

    #[test]
    fn test_remove_duplicate_access_modes() {
        let modes = vec![
            PV_ACCESS_READ_WRITE_ONCE.to_string(),
            PV_ACCESS_READ_ONLY_MANY.to_string(),
            PV_ACCESS_READ_ONLY_MANY.to_string(),
            PV_ACCESS_READ_ONLY_MANY.to_string(),
        ];
        let deduped = remove_duplicate_access_modes(&modes);
        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_topology_selector_requirements_as_selector() {
        let cases = vec![
            (Vec::new(), None, false),
            (Vec::new(), None, false),
            (
                vec![TopologySelectorLabelRequirement {
                    key: "foo".to_string(),
                    values: vec!["bar".to_string(), "baz".to_string()],
                }],
                Some(("foo", "bar")),
                false,
            ),
            (
                vec![TopologySelectorLabelRequirement {
                    key: "foo".to_string(),
                    values: Vec::new(),
                }],
                None,
                true,
            ),
            (
                vec![
                    TopologySelectorLabelRequirement {
                        key: "foo".to_string(),
                        values: vec!["bar".to_string(), "baz".to_string()],
                    },
                    TopologySelectorLabelRequirement {
                        key: "invalid".to_string(),
                        values: Vec::new(),
                    },
                ],
                None,
                true,
            ),
            (
                vec![TopologySelectorLabelRequirement {
                    key: "/invalidkey".to_string(),
                    values: vec!["bar".to_string(), "baz".to_string()],
                }],
                None,
                true,
            ),
        ];

        for (index, (requirements, label, expect_err)) in cases.into_iter().enumerate() {
            let selector = topology_selector_requirements_as_selector(&requirements);
            if expect_err {
                assert!(selector.is_err(), "case {index}");
                continue;
            }
            let selector = selector.expect("selector");
            if let Some((key, value)) = label {
                let labels = LabelSet::from([(key.to_string(), value.to_string())]);
                assert!(selector.matches(&labels), "case {index}");
            } else {
                assert!(selector.is_nothing(), "case {index}");
            }
        }
    }

    #[test]
    fn test_match_topology_selector_terms() {
        struct Case {
            name: &'static str,
            terms: Vec<TopologySelectorTerm>,
            labels: LabelSet,
            expected: bool,
        }

        let cases = vec![
            Case {
                name: "nil term list",
                terms: Vec::new(),
                labels: LabelSet::new(),
                expected: true,
            },
            Case {
                name: "nil term",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: Vec::new(),
                }],
                labels: LabelSet::new(),
                expected: false,
            },
            Case {
                name: "label matches MatchLabelExpressions terms",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: vec![TopologySelectorLabelRequirement {
                        key: "label_1".to_string(),
                        values: vec!["label_1_val".to_string()],
                    }],
                }],
                labels: LabelSet::from([("label_1".to_string(), "label_1_val".to_string())]),
                expected: true,
            },
            Case {
                name: "label does not match MatchLabelExpressions terms",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: vec![TopologySelectorLabelRequirement {
                        key: "label_1".to_string(),
                        values: vec!["label_1_val".to_string()],
                    }],
                }],
                labels: LabelSet::from([("label_1".to_string(), "label_1_val-failed".to_string())]),
                expected: false,
            },
            Case {
                name: "multi-values in one requirement, one matched",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: vec![TopologySelectorLabelRequirement {
                        key: "label_1".to_string(),
                        values: vec!["label_1_val1".to_string(), "label_1_val2".to_string()],
                    }],
                }],
                labels: LabelSet::from([("label_1".to_string(), "label_1_val2".to_string())]),
                expected: true,
            },
            Case {
                name: "multi-terms was set, one matched",
                terms: vec![
                    TopologySelectorTerm {
                        match_label_expressions: vec![TopologySelectorLabelRequirement {
                            key: "label_1".to_string(),
                            values: vec!["label_1_val".to_string()],
                        }],
                    },
                    TopologySelectorTerm {
                        match_label_expressions: vec![TopologySelectorLabelRequirement {
                            key: "label_2".to_string(),
                            values: vec!["label_2_val".to_string()],
                        }],
                    },
                ],
                labels: LabelSet::from([("label_2".to_string(), "label_2_val".to_string())]),
                expected: true,
            },
            Case {
                name: "multi-requirement in one term, fully matched",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: vec![
                        TopologySelectorLabelRequirement {
                            key: "label_1".to_string(),
                            values: vec!["label_1_val".to_string()],
                        },
                        TopologySelectorLabelRequirement {
                            key: "label_2".to_string(),
                            values: vec!["label_2_val".to_string()],
                        },
                    ],
                }],
                labels: LabelSet::from([
                    ("label_1".to_string(), "label_1_val".to_string()),
                    ("label_2".to_string(), "label_2_val".to_string()),
                ]),
                expected: true,
            },
            Case {
                name: "multi-requirement in one term, partial matched",
                terms: vec![TopologySelectorTerm {
                    match_label_expressions: vec![
                        TopologySelectorLabelRequirement {
                            key: "label_1".to_string(),
                            values: vec!["label_1_val".to_string()],
                        },
                        TopologySelectorLabelRequirement {
                            key: "label_2".to_string(),
                            values: vec!["label_2_val".to_string()],
                        },
                    ],
                }],
                labels: LabelSet::from([
                    ("label_1".to_string(), "label_1_val-failed".to_string()),
                    ("label_2".to_string(), "label_2_val".to_string()),
                ]),
                expected: false,
            },
        ];

        for case in cases {
            assert_eq!(
                match_topology_selector_terms(&case.terms, &case.labels),
                case.expected,
                "{}",
                case.name
            );
        }
    }

    #[test]
    fn test_node_selector_requirement_key_exists_in_node_selector_terms() {
        let cases = vec![
            (
                "empty set of keys in empty set of terms",
                Vec::new(),
                Vec::new(),
                false,
            ),
            (
                "key existence in terms with all keys specified",
                vec![
                    NodeSelectorRequirement {
                        key: "key1".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value1".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key2".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value2".to_string()],
                    },
                ],
                vec![
                    NodeSelectorTerm {
                        match_expressions: vec![
                            NodeSelectorRequirement {
                                key: "key2".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value2".to_string()],
                            },
                            NodeSelectorRequirement {
                                key: "key3".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value3".to_string()],
                            },
                        ],
                        ..Default::default()
                    },
                    NodeSelectorTerm {
                        match_expressions: vec![
                            NodeSelectorRequirement {
                                key: "key1".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value11, test-value12".to_string()],
                            },
                            NodeSelectorRequirement {
                                key: "key4".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value41, test-value42".to_string()],
                            },
                        ],
                        ..Default::default()
                    },
                ],
                true,
            ),
            (
                "key existence in terms with one of the keys specified",
                vec![
                    NodeSelectorRequirement {
                        key: "key1".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value1".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key2".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value2".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key3".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value3".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key6".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value6".to_string()],
                    },
                ],
                vec![
                    NodeSelectorTerm {
                        match_expressions: vec![
                            NodeSelectorRequirement {
                                key: "key2".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value2".to_string()],
                            },
                            NodeSelectorRequirement {
                                key: "key4".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value4".to_string()],
                            },
                        ],
                        ..Default::default()
                    },
                    NodeSelectorTerm {
                        match_expressions: vec![NodeSelectorRequirement {
                            key: "key5".to_string(),
                            operator: NODE_SELECTOR_OP_IN.to_string(),
                            values: vec!["test-value5".to_string()],
                        }],
                        ..Default::default()
                    },
                ],
                true,
            ),
            (
                "key existence in terms without any of the keys specified",
                vec![
                    NodeSelectorRequirement {
                        key: "key2".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value2".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key3".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value3".to_string()],
                    },
                ],
                vec![
                    NodeSelectorTerm {
                        match_expressions: vec![
                            NodeSelectorRequirement {
                                key: "key4".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value".to_string()],
                            },
                            NodeSelectorRequirement {
                                key: "key5".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value".to_string()],
                            },
                        ],
                        ..Default::default()
                    },
                    NodeSelectorTerm {
                        match_expressions: vec![NodeSelectorRequirement {
                            key: "key6".to_string(),
                            operator: NODE_SELECTOR_OP_IN.to_string(),
                            values: vec!["test-value".to_string()],
                        }],
                        ..Default::default()
                    },
                    NodeSelectorTerm {
                        match_expressions: vec![
                            NodeSelectorRequirement {
                                key: "key7".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value".to_string()],
                            },
                            NodeSelectorRequirement {
                                key: "key8".to_string(),
                                operator: NODE_SELECTOR_OP_IN.to_string(),
                                values: vec!["test-value".to_string()],
                            },
                        ],
                        ..Default::default()
                    },
                ],
                false,
            ),
            (
                "key existence in empty set of terms",
                vec![
                    NodeSelectorRequirement {
                        key: "key2".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value2".to_string()],
                    },
                    NodeSelectorRequirement {
                        key: "key3".to_string(),
                        operator: NODE_SELECTOR_OP_IN.to_string(),
                        values: vec!["test-value3".to_string()],
                    },
                ],
                Vec::new(),
                false,
            ),
        ];

        for (name, reqs, terms, expected) in cases {
            assert_eq!(
                node_selector_requirement_keys_exist_in_node_selector_terms(&reqs, &terms),
                expected,
                "{name}"
            );
        }
    }

    #[test]
    fn test_huge_page_unit_size_from_byte_size() {
        let cases = vec![
            (1024, Some("1KB")),
            (33_554_432, Some("32MB")),
            (3_221_225_472, Some("3GB")),
            (1024 * 1024 * 1023 * 3, None),
        ];
        for (size, expected) in cases {
            let result = huge_page_unit_size_from_byte_size(size);
            match expected {
                Some(expected) => assert_eq!(result.expect("result"), expected),
                None => assert!(result.is_err()),
            }
        }
    }

    #[test]
    fn test_pod_log_options_params_roundtrip() {
        let since_time = DateTime::parse_from_rfc3339("2000-01-01T12:34:56Z")
            .expect("parse time")
            .with_timezone(&Utc);
        let options = PodLogOptions {
            container: "mycontainer".to_string(),
            follow: true,
            previous: true,
            since_seconds: Some(1),
            since_time: Some(Time(Some(since_time))),
            timestamps: true,
            tail_lines: Some(2),
            limit_bytes: Some(3),
            stream: Some(LOG_STREAM_STDERR.to_string()),
            ..Default::default()
        };

        let params = pod_log_options_to_params(&options);
        let mut expected = QueryParams::new();
        expected.insert("container".to_string(), vec!["mycontainer".to_string()]);
        expected.insert("follow".to_string(), vec!["true".to_string()]);
        expected.insert("previous".to_string(), vec!["true".to_string()]);
        expected.insert("sinceSeconds".to_string(), vec!["1".to_string()]);
        expected.insert(
            "sinceTime".to_string(),
            vec!["2000-01-01T12:34:56Z".to_string()],
        );
        expected.insert("timestamps".to_string(), vec!["true".to_string()]);
        expected.insert("tailLines".to_string(), vec!["2".to_string()]);
        expected.insert("limitBytes".to_string(), vec!["3".to_string()]);
        expected.insert("stream".to_string(), vec![LOG_STREAM_STDERR.to_string()]);
        assert_eq!(params, expected);

        let decoded = pod_log_options_from_params(&params).expect("decode params");
        assert_eq!(decoded, options);
    }

    #[cfg(feature = "apps")]
    #[test]
    fn test_replication_controller_replica_set_roundtrip() {
        let mut selector = std::collections::HashMap::new();
        selector.insert("foo".to_string(), "bar".to_string());
        selector.insert("bar".to_string(), "foo".to_string());

        let labels: std::collections::BTreeMap<String, String> = selector
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let template = PodTemplateSpec {
            metadata: ObjectMeta {
                labels,
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: vec![Container {
                    name: "container".to_string(),
                    image: "image".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        };

        let controller = ReplicationController {
            metadata: ObjectMeta::named("name"),
            spec: Some(ReplicationControllerSpec {
                replicas: Some(1),
                min_ready_seconds: 32,
                selector,
                template: Some(template),
            }),
            status: Some(ReplicationControllerStatus {
                replicas: 1,
                fully_labeled_replicas: 2,
                ready_replicas: 3,
                available_replicas: 4,
                observed_generation: 5,
                conditions: vec![ReplicationControllerCondition {
                    condition_type: REPLICATION_CONTROLLER_CONDITION_REPLICA_FAILURE.to_string(),
                    status: CONDITION_TRUE.to_string(),
                    last_transition_time: None,
                    reason: "Reason".to_string(),
                    message: "Message".to_string(),
                }],
            }),
            ..Default::default()
        };

        let rs = replication_controller_to_replica_set(&controller).expect("rc to rs");
        let back = replica_set_to_replication_controller(&rs).expect("rs to rc");
        assert_eq!(back, controller);
    }

    #[cfg(feature = "apps")]
    #[test]
    fn test_replica_set_replication_controller_roundtrip() {
        let selector_labels: std::collections::BTreeMap<String, String> =
            [("foo".to_string(), "bar".to_string())]
                .into_iter()
                .collect();
        let selector = k8s_apimachinery::apis::meta::v1::LabelSelector {
            match_labels: selector_labels.clone(),
            match_expressions: Vec::new(),
        };

        let template = PodTemplateSpec {
            metadata: ObjectMeta {
                labels: selector_labels,
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: vec![Container {
                    name: "container".to_string(),
                    image: "image".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        };

        let rs = crate::apps::v1::ReplicaSet {
            metadata: ObjectMeta::named("name"),
            spec: Some(crate::apps::v1::ReplicaSetSpec {
                replicas: Some(1),
                min_ready_seconds: Some(32),
                selector: Some(selector),
                template: Some(template),
            }),
            status: Some(crate::apps::v1::ReplicaSetStatus {
                replicas: 1,
                fully_labeled_replicas: Some(2),
                ready_replicas: Some(3),
                available_replicas: Some(4),
                observed_generation: Some(5),
                conditions: vec![crate::apps::v1::ReplicaSetCondition {
                    condition_type: REPLICATION_CONTROLLER_CONDITION_REPLICA_FAILURE.to_string(),
                    status: CONDITION_TRUE.to_string(),
                    last_transition_time: None,
                    reason: "Reason".to_string(),
                    message: "Message".to_string(),
                }],
            }),
            ..Default::default()
        };

        let controller = replica_set_to_replication_controller(&rs).expect("rs to rc");
        let back = replication_controller_to_replica_set(&controller).expect("rc to rs");
        assert_eq!(back, rs);
    }

    #[cfg(feature = "apps")]
    #[test]
    fn test_replica_set_selector_expression_invalid() {
        let selector = k8s_apimachinery::apis::meta::v1::LabelSelector {
            match_labels: std::collections::BTreeMap::new(),
            match_expressions: vec![k8s_apimachinery::apis::meta::v1::LabelSelectorRequirement {
                key: "key".to_string(),
                operator: "NotIn".to_string(),
                values: vec!["value".to_string()],
            }],
        };

        let rs = crate::apps::v1::ReplicaSet {
            spec: Some(crate::apps::v1::ReplicaSetSpec {
                replicas: Some(1),
                selector: Some(selector),
                template: Some(PodTemplateSpec::default()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let err = replica_set_to_replication_controller(&rs).expect_err("selector error");
        assert!(err.contains("operator"), "unexpected error: {err}");
    }
}
