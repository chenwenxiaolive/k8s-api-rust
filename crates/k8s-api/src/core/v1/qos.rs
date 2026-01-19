use crate::core::helper::quantity;
use k8s_api_core::resource::Quantity;
use std::collections::BTreeMap;

use super::*;

type QuantityMap = BTreeMap<ResourceName, quantity::Rational>;

fn is_supported_qos_compute_resource(name: &str) -> bool {
    name == RESOURCE_CPU || name == RESOURCE_MEMORY
}

fn parse_positive_quantity(quantity: &Quantity) -> Option<quantity::Rational> {
    let parsed = quantity::parse_quantity_rational(quantity.as_str())?;
    if quantity::is_zero(parsed) {
        None
    } else {
        Some(parsed)
    }
}

fn add_quantity(target: &mut QuantityMap, name: &ResourceName, value: quantity::Rational) {
    match target.get_mut(name) {
        Some(existing) => {
            if let Some(sum) = quantity::add(*existing, value) {
                *existing = sum;
            }
        }
        None => {
            target.insert(name.clone(), value);
        }
    }
}

fn process_resource_list(target: &mut QuantityMap, list: &ResourceList) {
    for (name, quantity) in list {
        if !is_supported_qos_compute_resource(name) {
            continue;
        }
        if let Some(parsed) = parse_positive_quantity(quantity) {
            add_quantity(target, name, parsed);
        }
    }
}

pub fn get_pod_qos(pod: &Pod) -> PodQOSClass {
    if let Some(status) = pod.status.as_ref() {
        if !status.qos_class.is_empty() {
            return status.qos_class.clone();
        }
    }
    compute_pod_qos(pod)
}

pub fn compute_pod_qos(pod: &Pod) -> PodQOSClass {
    let mut requests = QuantityMap::new();
    let mut limits = QuantityMap::new();
    let mut is_guaranteed = true;

    let spec = match pod.spec.as_ref() {
        Some(spec) => spec,
        None => return POD_QOS_BEST_EFFORT.to_string(),
    };

    if feature_pod_level_resources_enabled() && spec.resources.is_some() {
        if let Some(resources) = spec.resources.as_ref() {
            process_resource_list(&mut requests, &resources.requests);
            if !resources.limits.is_empty() {
                let mut has_cpu = false;
                let mut has_memory = false;
                for (name, quantity) in &resources.limits {
                    if !is_supported_qos_compute_resource(name) {
                        continue;
                    }
                    if let Some(parsed) = parse_positive_quantity(quantity) {
                        if name == RESOURCE_CPU {
                            has_cpu = true;
                        }
                        if name == RESOURCE_MEMORY {
                            has_memory = true;
                        }
                        add_quantity(&mut limits, name, parsed);
                    }
                }
                if !(has_cpu && has_memory) {
                    is_guaranteed = false;
                }
            }
        }
    } else {
        for container in spec
            .containers
            .iter()
            .chain(spec.init_containers.iter())
        {
            if let Some(resources) = container.resources.as_ref() {
                process_resource_list(&mut requests, &resources.requests);
                let mut has_cpu = false;
                let mut has_memory = false;
                for (name, quantity) in &resources.limits {
                    if !is_supported_qos_compute_resource(name) {
                        continue;
                    }
                    if let Some(parsed) = parse_positive_quantity(quantity) {
                        if name == RESOURCE_CPU {
                            has_cpu = true;
                        }
                        if name == RESOURCE_MEMORY {
                            has_memory = true;
                        }
                        add_quantity(&mut limits, name, parsed);
                    }
                }
                if !(has_cpu && has_memory) {
                    is_guaranteed = false;
                }
            } else {
                is_guaranteed = false;
            }
        }
    }

    if requests.is_empty() && limits.is_empty() {
        return POD_QOS_BEST_EFFORT.to_string();
    }
    if is_guaranteed {
        for (name, req) in &requests {
            match limits.get(name) {
                Some(lim) if *lim == *req => {}
                _ => {
                    is_guaranteed = false;
                    break;
                }
            }
        }
    }
    if is_guaranteed && requests.len() == limits.len() {
        return POD_QOS_GUARANTEED.to_string();
    }
    POD_QOS_BURSTABLE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::qos as internal_qos;
    use crate::core::v1::InternalConversion;
    use crate::core::v1::defaults::FEATURE_LOCK;
    use k8s_api_core::resource::Quantity;

    struct FeatureOverride<'a> {
        _lock: std::sync::MutexGuard<'a, ()>,
        prev_pod_level: bool,
    }

    impl<'a> FeatureOverride<'a> {
        fn new(pod_level: bool) -> Self {
            let lock = FEATURE_LOCK.lock().expect("feature lock");
            let prev_pod_level = feature_pod_level_resources_enabled();
            set_feature_pod_level_resources(pod_level);
            Self {
                _lock: lock,
                prev_pod_level,
            }
        }
    }

    impl Drop for FeatureOverride<'_> {
        fn drop(&mut self) {
            set_feature_pod_level_resources(self.prev_pod_level);
        }
    }

    fn get_resource_list(cpu: &str, memory: &str) -> ResourceList {
        let mut res = ResourceList::new();
        if !cpu.is_empty() {
            res.insert(RESOURCE_CPU.to_string(), Quantity::new(cpu));
        }
        if !memory.is_empty() {
            res.insert(RESOURCE_MEMORY.to_string(), Quantity::new(memory));
        }
        res
    }

    fn add_resource(name: &str, value: &str, mut list: ResourceList) -> ResourceList {
        list.insert(name.to_string(), Quantity::new(value));
        list
    }

    fn get_resource_requirements(requests: ResourceList, limits: ResourceList) -> ResourceRequirements {
        ResourceRequirements {
            requests,
            limits,
            ..Default::default()
        }
    }

    fn new_container(name: &str, requests: ResourceList, limits: ResourceList) -> Container {
        Container {
            name: name.to_string(),
            resources: Some(get_resource_requirements(requests, limits)),
            ..Default::default()
        }
    }

    fn new_pod(name: &str, containers: Vec<Container>) -> Pod {
        let mut pod = Pod::new(name);
        pod.spec = Some(PodSpec {
            containers,
            ..Default::default()
        });
        pod
    }

    fn new_pod_with_resources(
        name: &str,
        containers: Vec<Container>,
        pod_resources: Option<ResourceRequirements>,
    ) -> Pod {
        let mut pod = new_pod(name, containers);
        if let Some(spec) = pod.spec.as_mut() {
            spec.resources = pod_resources;
        }
        pod
    }

    fn new_pod_with_init_containers(
        name: &str,
        containers: Vec<Container>,
        init_containers: Vec<Container>,
    ) -> Pod {
        let mut pod = Pod::new(name);
        pod.spec = Some(PodSpec {
            containers,
            init_containers,
            ..Default::default()
        });
        pod
    }

    #[test]
    fn test_compute_pod_qos() {
        struct TestCase {
            pod: Pod,
            expected: PodQOSClass,
            pod_level_resources_enabled: bool,
        }

        let cases = vec![
            TestCase {
                pod: new_pod(
                    "guaranteed",
                    vec![new_container(
                        "guaranteed",
                        get_resource_list("100m", "100Mi"),
                        get_resource_list("100m", "100Mi"),
                    )],
                ),
                expected: POD_QOS_GUARANTEED.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "guaranteed-guaranteed",
                    vec![
                        new_container(
                            "guaranteed",
                            get_resource_list("100m", "100Mi"),
                            get_resource_list("100m", "100Mi"),
                        ),
                        new_container(
                            "guaranteed",
                            get_resource_list("100m", "100Mi"),
                            get_resource_list("100m", "100Mi"),
                        ),
                    ],
                ),
                expected: POD_QOS_GUARANTEED.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "best-effort-best-effort",
                    vec![
                        new_container(
                            "best-effort",
                            get_resource_list("", ""),
                            get_resource_list("", ""),
                        ),
                        new_container(
                            "best-effort",
                            get_resource_list("", ""),
                            get_resource_list("", ""),
                        ),
                    ],
                ),
                expected: POD_QOS_BEST_EFFORT.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "best-effort",
                    vec![new_container(
                        "best-effort",
                        get_resource_list("", ""),
                        get_resource_list("", ""),
                    )],
                ),
                expected: POD_QOS_BEST_EFFORT.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "best-effort-burstable",
                    vec![
                        new_container(
                            "best-effort",
                            get_resource_list("", ""),
                            get_resource_list("", ""),
                        ),
                        new_container(
                            "burstable",
                            get_resource_list("1", ""),
                            get_resource_list("2", ""),
                        ),
                    ],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "best-effort-guaranteed",
                    vec![
                        new_container(
                            "best-effort",
                            get_resource_list("", ""),
                            get_resource_list("", ""),
                        ),
                        new_container(
                            "guaranteed",
                            get_resource_list("10m", "100Mi"),
                            get_resource_list("10m", "100Mi"),
                        ),
                    ],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-cpu-guaranteed-memory",
                    vec![new_container(
                        "burstable",
                        get_resource_list("", "100Mi"),
                        get_resource_list("", "100Mi"),
                    )],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-no-limits",
                    vec![new_container(
                        "burstable",
                        get_resource_list("100m", "100Mi"),
                        get_resource_list("", ""),
                    )],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-guaranteed",
                    vec![
                        new_container(
                            "burstable",
                            get_resource_list("1", "100Mi"),
                            get_resource_list("2", "100Mi"),
                        ),
                        new_container(
                            "guaranteed",
                            get_resource_list("100m", "100Mi"),
                            get_resource_list("100m", "100Mi"),
                        ),
                    ],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-unbounded-but-requests-match-limits",
                    vec![
                        new_container(
                            "burstable",
                            get_resource_list("100m", "100Mi"),
                            get_resource_list("200m", "200Mi"),
                        ),
                        new_container(
                            "burstable-unbounded",
                            get_resource_list("100m", "100Mi"),
                            get_resource_list("", ""),
                        ),
                    ],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-1",
                    vec![new_container(
                        "burstable",
                        get_resource_list("10m", "100Mi"),
                        get_resource_list("100m", "200Mi"),
                    )],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "burstable-2",
                    vec![new_container(
                        "burstable",
                        get_resource_list("0", "0"),
                        get_resource_list("100m", "200Mi"),
                    )],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod(
                    "best-effort-hugepages",
                    vec![new_container(
                        "best-effort",
                        add_resource(
                            "hugepages-2Mi",
                            "1Gi",
                            get_resource_list("0", "0"),
                        ),
                        add_resource(
                            "hugepages-2Mi",
                            "1Gi",
                            get_resource_list("0", "0"),
                        ),
                    )],
                ),
                expected: POD_QOS_BEST_EFFORT.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod_with_init_containers(
                    "init-container",
                    vec![new_container(
                        "best-effort",
                        get_resource_list("", ""),
                        get_resource_list("", ""),
                    )],
                    vec![new_container(
                        "burstable",
                        get_resource_list("10m", "100Mi"),
                        get_resource_list("100m", "200Mi"),
                    )],
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: false,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "guaranteed-with-pod-level-resources",
                    vec![new_container(
                        "best-effort",
                        get_resource_list("", ""),
                        get_resource_list("", ""),
                    )],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "100Mi"),
                        get_resource_list("10m", "100Mi"),
                    )),
                ),
                expected: POD_QOS_GUARANTEED.to_string(),
                pod_level_resources_enabled: true,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "guaranteed-with-pod-and-container-level-resources",
                    vec![new_container(
                        "burstable",
                        get_resource_list("3m", "10Mi"),
                        get_resource_list("5m", "20Mi"),
                    )],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "100Mi"),
                        get_resource_list("10m", "100Mi"),
                    )),
                ),
                expected: POD_QOS_GUARANTEED.to_string(),
                pod_level_resources_enabled: true,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "burstable-with-pod-level-resources",
                    vec![new_container(
                        "best-effort",
                        get_resource_list("", ""),
                        get_resource_list("", ""),
                    )],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "10Mi"),
                        get_resource_list("20m", "50Mi"),
                    )),
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: true,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "burstable-with-pod-and-container-level-resources",
                    vec![new_container(
                        "burstable",
                        get_resource_list("5m", "10Mi"),
                        get_resource_list("5m", "10Mi"),
                    )],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "10Mi"),
                        get_resource_list("20m", "50Mi"),
                    )),
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: true,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "burstable-with-pod-and-container-level-requests",
                    vec![new_container(
                        "burstable",
                        get_resource_list("5m", "10Mi"),
                        get_resource_list("", ""),
                    )],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "10Mi"),
                        get_resource_list("", ""),
                    )),
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: true,
            },
            TestCase {
                pod: new_pod_with_resources(
                    "burstable-with-pod-and-container-level-resources-2",
                    vec![
                        new_container(
                            "burstable",
                            get_resource_list("5m", "10Mi"),
                            get_resource_list("", ""),
                        ),
                        new_container(
                            "guaranteed",
                            get_resource_list("5m", "10Mi"),
                            get_resource_list("5m", "10Mi"),
                        ),
                    ],
                    Some(get_resource_requirements(
                        get_resource_list("10m", "10Mi"),
                        get_resource_list("5m", ""),
                    )),
                ),
                expected: POD_QOS_BURSTABLE.to_string(),
                pod_level_resources_enabled: true,
            },
        ];

        for (index, case) in cases.iter().enumerate() {
            let _feature = FeatureOverride::new(case.pod_level_resources_enabled);
            let actual = compute_pod_qos(&case.pod);
            assert_eq!(
                actual, case.expected,
                "case {index}: expected pod qos {}, got {}",
                case.expected, actual
            );

            let internal = case
                .pod
                .clone()
                .into_internal()
                .expect("pod conversion");
            let internal_actual = internal_qos::compute_pod_qos(&internal);
            assert_eq!(
                internal_actual, case.expected,
                "case {index}: expected internal pod qos {}, got {}",
                case.expected, internal_actual
            );
        }
    }
}
