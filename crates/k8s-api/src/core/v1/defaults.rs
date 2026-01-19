use super::*;
use k8s_api_core::{IntOrString, Quantity};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
#[cfg(test)]
use std::sync::Mutex;

const DEFAULT_TERMINATION_GRACE_PERIOD_SECONDS: i64 = 30;
const DEFAULT_TERMINATION_MESSAGE_PATH: &str = "/dev/termination-log";
const DEFAULT_PROBE_TIMEOUT_SECONDS: i32 = 1;
const DEFAULT_PROBE_PERIOD_SECONDS: i32 = 10;
const DEFAULT_PROBE_SUCCESS_THRESHOLD: i32 = 1;
const DEFAULT_PROBE_FAILURE_THRESHOLD: i32 = 3;
const DEFAULT_VOLUME_MODE: i32 = 420;
const DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS: i64 = 3600;
const DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS: i32 = 10800;
const NAMESPACE_METADATA_LABEL: &str = "kubernetes.io/metadata.name";
const DEFAULT_ENABLE_SERVICE_LINKS: bool = true;
const DEFAULT_ISCSI_INTERFACE: &str = "default";
const DEFAULT_RBD_POOL: &str = "rbd";
const DEFAULT_RBD_USER: &str = "admin";
const DEFAULT_RBD_KEYRING: &str = "/etc/ceph/keyring";
const DEFAULT_AZURE_DISK_FSTYPE: &str = "ext4";
const DEFAULT_SCALE_IO_STORAGE_MODE: &str = "ThinProvisioned";
const DEFAULT_SCALE_IO_FSTYPE: &str = "xfs";
static FEATURE_IMAGE_VOLUME: AtomicBool = AtomicBool::new(false);
static FEATURE_POD_LOGS_QUERY_SPLIT_STREAMS: AtomicBool = AtomicBool::new(false);
static FEATURE_POD_LEVEL_RESOURCES: AtomicBool = AtomicBool::new(true);
#[cfg(test)]
pub(crate) static FEATURE_LOCK: Mutex<()> = Mutex::new(());

pub fn set_feature_image_volume(enabled: bool) {
    FEATURE_IMAGE_VOLUME.store(enabled, AtomicOrdering::Relaxed);
}

pub fn set_feature_pod_logs_query_split_streams(enabled: bool) {
    FEATURE_POD_LOGS_QUERY_SPLIT_STREAMS.store(enabled, AtomicOrdering::Relaxed);
}

pub fn set_feature_pod_level_resources(enabled: bool) {
    FEATURE_POD_LEVEL_RESOURCES.store(enabled, AtomicOrdering::Relaxed);
}

fn feature_image_volume_enabled() -> bool {
    FEATURE_IMAGE_VOLUME.load(AtomicOrdering::Relaxed)
}

fn feature_pod_logs_query_split_streams_enabled() -> bool {
    FEATURE_POD_LOGS_QUERY_SPLIT_STREAMS.load(AtomicOrdering::Relaxed)
}

pub(crate) fn feature_pod_level_resources_enabled() -> bool {
    FEATURE_POD_LEVEL_RESOURCES.load(AtomicOrdering::Relaxed)
}

pub fn apply_defaults_pod(pod: &mut Pod) {
    if let Some(spec) = pod.spec.as_mut() {
        default_container_resource_requests(&mut spec.containers);
        default_container_resource_requests(&mut spec.init_containers);
    }
    if feature_pod_level_resources_enabled() {
        default_huge_page_pod_limits(pod);
        default_pod_requests(pod);
    }
    if let Some(spec) = pod.spec.as_mut() {
        if spec.enable_service_links.is_none() {
            spec.enable_service_links = Some(DEFAULT_ENABLE_SERVICE_LINKS);
        }
        apply_defaults_pod_spec(spec);
        if spec.host_network {
            default_host_network_ports(&mut spec.containers);
            default_host_network_ports(&mut spec.init_containers);
        }
    }
    if let Some(status) = pod.status.as_mut() {
        apply_defaults_pod_status(status);
    }
}

pub fn apply_defaults_pod_status(status: &mut PodStatus) {
    for container in status.init_container_statuses.iter_mut() {
        apply_defaults_container_status(container);
    }
    for container in status.container_statuses.iter_mut() {
        apply_defaults_container_status(container);
    }
    for container in status.ephemeral_container_statuses.iter_mut() {
        apply_defaults_container_status(container);
    }
}

fn apply_defaults_container_status(status: &mut ContainerStatus) {
    apply_defaults_resource_list(&mut status.allocated_resources);
    if let Some(resources) = status.resources.as_mut() {
        apply_defaults_resource_requirements(resources);
    }
}

pub fn apply_defaults_pod_status_result(result: &mut PodStatusResult) {
    if let Some(status) = result.status.as_mut() {
        apply_defaults_pod_status(status);
    }
}

pub fn apply_defaults_pod_list(list: &mut PodList) {
    for pod in list.items.iter_mut() {
        apply_defaults_pod(pod);
    }
}

pub fn apply_defaults_pod_template(template: &mut PodTemplate) {
    if let Some(spec) = template.template.as_mut() {
        if let Some(pod_spec) = spec.spec.as_mut() {
            apply_defaults_pod_spec(pod_spec);
        }
    }
}

pub fn apply_defaults_pod_template_list(list: &mut PodTemplateList) {
    for template in list.items.iter_mut() {
        apply_defaults_pod_template(template);
    }
}

fn default_container_resource_requests(containers: &mut [Container]) {
    for container in containers.iter_mut() {
        let resources = match container.resources.as_mut() {
            Some(resources) => resources,
            None => continue,
        };
        if resources.limits.is_empty() {
            continue;
        }
        for (key, value) in resources.limits.iter() {
            resources
                .requests
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }
}

fn default_pod_requests(pod: &mut Pod) {
    let aggr_ctr_reqs = aggregate_container_requests(pod);
    let resources = match pod.spec.as_mut().and_then(|spec| spec.resources.as_mut()) {
        Some(resources) => resources,
        None => return,
    };
    if resources.limits.is_empty() {
        return;
    }

    let mut pod_reqs = resources.requests.clone();

    for (name, quantity) in aggr_ctr_reqs {
        if pod_reqs.contains_key(&name) {
            continue;
        }
        if is_supported_pod_level_resource(&name) && is_overcommit_allowed(&name) {
            pod_reqs.insert(name, quantity);
        }
    }

    for (name, quantity) in resources.limits.iter() {
        if pod_reqs.contains_key(name) {
            continue;
        }
        if is_supported_pod_level_resource(name) {
            pod_reqs.insert(name.clone(), quantity.clone());
        }
    }

    if !pod_reqs.is_empty() {
        resources.requests = pod_reqs;
    }
}

fn default_huge_page_pod_limits(pod: &mut Pod) {
    let aggr_ctr_limits = aggregate_container_limits(pod);
    let resources = match pod.spec.as_mut().and_then(|spec| spec.resources.as_mut()) {
        Some(resources) => resources,
        None => return,
    };
    if resources.limits.is_empty() && resources.requests.is_empty() {
        return;
    }

    let mut pod_limits = resources.limits.clone();

    for (name, quantity) in aggr_ctr_limits {
        if !is_supported_pod_level_resource(&name) || !is_huge_page_resource_name(&name) {
            continue;
        }
        if resources.requests.contains_key(&name) {
            continue;
        }
        if !pod_limits.contains_key(&name) {
            pod_limits.insert(name, quantity);
        }
    }

    if !pod_limits.is_empty() {
        resources.limits = pod_limits;
    }
}

fn is_supported_pod_level_resource(name: &str) -> bool {
    name == RESOURCE_CPU || name == RESOURCE_MEMORY || name.starts_with(RESOURCE_HUGE_PAGES_PREFIX)
}

fn is_huge_page_resource_name(name: &str) -> bool {
    name.starts_with(RESOURCE_HUGE_PAGES_PREFIX)
}

fn is_native_resource(name: &str) -> bool {
    !name.contains('/') || name.contains(RESOURCE_DEFAULT_NAMESPACE_PREFIX)
}

fn is_overcommit_allowed(name: &str) -> bool {
    is_native_resource(name) && !is_huge_page_resource_name(name)
}

fn aggregate_container_requests(pod: &Pod) -> ResourceList {
    aggregate_container_resources(pod, |resources| &resources.requests)
}

fn aggregate_container_limits(pod: &Pod) -> ResourceList {
    aggregate_container_resources(pod, |resources| &resources.limits)
}

fn aggregate_container_resources(
    pod: &Pod,
    select: fn(&ResourceRequirements) -> &ResourceList,
) -> ResourceList {
    let spec = match pod.spec.as_ref() {
        Some(spec) => spec,
        None => return ResourceList::new(),
    };
    let mut total = QuantityMap::new();
    for container in &spec.containers {
        if let Some(resources) = container.resources.as_ref() {
            add_resource_list(&mut total, select(resources));
        }
    }

    let mut restartable_init = QuantityMap::new();
    let mut init_max = QuantityMap::new();
    for container in &spec.init_containers {
        let mut container_resources = QuantityMap::new();
        if let Some(resources) = container.resources.as_ref() {
            add_resource_list(&mut container_resources, select(resources));
        }

        if is_restartable_init(container) {
            add_quantity_map(&mut total, &container_resources);
            add_quantity_map(&mut restartable_init, &container_resources);
            container_resources = restartable_init.clone();
        } else {
            let mut tmp = container_resources.clone();
            add_quantity_map(&mut tmp, &restartable_init);
            container_resources = tmp;
        }

        max_quantity_map(&mut init_max, &container_resources);
    }

    max_quantity_map(&mut total, &init_max);
    resource_list_from_quantity_map(total)
}

fn is_restartable_init(container: &Container) -> bool {
    matches!(
        container.restart_policy.as_deref(),
        Some(CONTAINER_RESTART_POLICY_ALWAYS)
    )
}

type QuantityMap = BTreeMap<ResourceName, ParsedQuantity>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuantityFormat {
    Decimal,
    Binary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DecimalNumber {
    digits: String,
    scale: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedQuantity {
    value: DecimalNumber,
    format: QuantityFormat,
}

impl ParsedQuantity {
    fn parse(quantity: &Quantity) -> Option<Self> {
        parse_quantity_str(quantity.as_str())
    }

    fn add_assign(&mut self, other: &ParsedQuantity) {
        self.value = add_decimal(&self.value, &other.value);
    }

    fn cmp(&self, other: &ParsedQuantity) -> Ordering {
        cmp_decimal(&self.value, &other.value)
    }

    fn to_quantity(&self) -> Quantity {
        Quantity::new(format_quantity(&self.value, self.format))
    }
}

fn add_resource_list(target: &mut QuantityMap, list: &ResourceList) {
    for (name, quantity) in list {
        let parsed = match ParsedQuantity::parse(quantity) {
            Some(parsed) => parsed,
            None => continue,
        };
        match target.get_mut(name) {
            Some(existing) => existing.add_assign(&parsed),
            None => {
                target.insert(name.clone(), parsed);
            }
        }
    }
}

fn add_quantity_map(target: &mut QuantityMap, list: &QuantityMap) {
    for (name, quantity) in list {
        match target.get_mut(name) {
            Some(existing) => existing.add_assign(quantity),
            None => {
                target.insert(name.clone(), quantity.clone());
            }
        }
    }
}

fn max_quantity_map(target: &mut QuantityMap, list: &QuantityMap) {
    for (name, quantity) in list {
        let replace = match target.get(name) {
            Some(existing) => quantity.cmp(existing) == Ordering::Greater,
            None => true,
        };
        if replace {
            target.insert(name.clone(), quantity.clone());
        }
    }
}

fn resource_list_from_quantity_map(map: QuantityMap) -> ResourceList {
    map.into_iter()
        .map(|(name, quantity)| (name, quantity.to_quantity()))
        .collect()
}

fn parse_quantity_str(input: &str) -> Option<ParsedQuantity> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }
    let input = input.strip_prefix('+').unwrap_or(input);
    if input.starts_with('-') {
        return None;
    }
    let (number, suffix) = split_number_suffix(input)?;
    let mut value = parse_number(number)?;
    let format = match suffix {
        "Ki" | "Mi" | "Gi" | "Ti" | "Pi" | "Ei" => QuantityFormat::Binary,
        _ => QuantityFormat::Decimal,
    };
    match suffix {
        "" => {}
        "n" => value.scale += -9,
        "u" => value.scale += -6,
        "m" => value.scale += -3,
        "k" => value.scale += 3,
        "M" => value.scale += 6,
        "G" => value.scale += 9,
        "T" => value.scale += 12,
        "P" => value.scale += 15,
        "E" => value.scale += 18,
        "Ki" => value.digits = mul_str_small(&value.digits, 1u128 << 10),
        "Mi" => value.digits = mul_str_small(&value.digits, 1u128 << 20),
        "Gi" => value.digits = mul_str_small(&value.digits, 1u128 << 30),
        "Ti" => value.digits = mul_str_small(&value.digits, 1u128 << 40),
        "Pi" => value.digits = mul_str_small(&value.digits, 1u128 << 50),
        "Ei" => value.digits = mul_str_small(&value.digits, 1u128 << 60),
        _ => {
            if let Some(exp) = suffix.strip_prefix('e').or_else(|| suffix.strip_prefix('E')) {
                let exp = exp.parse::<i32>().ok()?;
                value.scale += exp;
            } else {
                return None;
            }
        }
    }
    value.normalize();
    Some(ParsedQuantity { value, format })
}

fn split_number_suffix(input: &str) -> Option<(&str, &str)> {
    let mut idx = 0;
    for (i, ch) in input.char_indices() {
        if ch.is_ascii_digit() || ch == '.' {
            idx = i + ch.len_utf8();
            continue;
        }
        break;
    }
    if idx == 0 {
        return None;
    }
    Some(input.split_at(idx))
}

fn parse_number(input: &str) -> Option<DecimalNumber> {
    let mut int_part = String::new();
    let mut frac_part = String::new();
    let mut iter = input.char_indices().peekable();
    while let Some((_, ch)) = iter.peek() {
        if ch.is_ascii_digit() {
            int_part.push(*ch);
            iter.next();
        } else {
            break;
        }
    }
    if matches!(iter.peek().map(|(_, ch)| *ch), Some('.')) {
        iter.next();
        while let Some((_, ch)) = iter.peek() {
            if ch.is_ascii_digit() {
                frac_part.push(*ch);
                iter.next();
            } else {
                break;
            }
        }
    }
    if int_part.is_empty() && frac_part.is_empty() {
        return None;
    }
    let mut digits = String::new();
    digits.push_str(&int_part);
    digits.push_str(&frac_part);
    let scale = -(frac_part.len() as i32);
    if digits.is_empty() {
        digits.push('0');
    }
    let mut value = DecimalNumber { digits, scale };
    value.normalize();
    Some(value)
}

impl DecimalNumber {
    fn normalize(&mut self) {
        let trimmed = self.digits.trim_start_matches('0');
        if trimmed.is_empty() {
            self.digits = "0".to_string();
            self.scale = 0;
            return;
        }
        self.digits = trimmed.to_string();
        while self.digits.len() > 1 && self.digits.ends_with('0') {
            self.digits.pop();
            self.scale += 1;
        }
    }
}

fn add_decimal(a: &DecimalNumber, b: &DecimalNumber) -> DecimalNumber {
    let (lhs, rhs, scale) = align_scales(a, b);
    let digits = add_digit_strings(&lhs.digits, &rhs.digits);
    let mut value = DecimalNumber { digits, scale };
    value.normalize();
    value
}

fn cmp_decimal(a: &DecimalNumber, b: &DecimalNumber) -> Ordering {
    let (lhs, rhs, _) = align_scales(a, b);
    cmp_digit_strings(&lhs.digits, &rhs.digits)
}

fn align_scales(a: &DecimalNumber, b: &DecimalNumber) -> (DecimalNumber, DecimalNumber, i32) {
    let target = a.scale.min(b.scale);
    let mut lhs = a.clone();
    let mut rhs = b.clone();
    if lhs.scale > target {
        let shift = (lhs.scale - target) as usize;
        lhs.digits = mul_pow10(&lhs.digits, shift);
        lhs.scale = target;
    }
    if rhs.scale > target {
        let shift = (rhs.scale - target) as usize;
        rhs.digits = mul_pow10(&rhs.digits, shift);
        rhs.scale = target;
    }
    (lhs, rhs, target)
}

fn add_digit_strings(a: &str, b: &str) -> String {
    let mut result = Vec::new();
    let mut carry = 0u8;
    let mut a_iter = a.as_bytes().iter().rev();
    let mut b_iter = b.as_bytes().iter().rev();
    loop {
        let da = a_iter.next().map(|c| *c - b'0');
        let db = b_iter.next().map(|c| *c - b'0');
        if da.is_none() && db.is_none() && carry == 0 {
            break;
        }
        let sum = da.unwrap_or(0) + db.unwrap_or(0) + carry;
        carry = sum / 10;
        result.push((sum % 10) + b'0');
    }
    result.reverse();
    String::from_utf8(result).unwrap_or_else(|_| "0".to_string())
}

fn cmp_digit_strings(a: &str, b: &str) -> Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    let a = if a.is_empty() { "0" } else { a };
    let b = if b.is_empty() { "0" } else { b };
    match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        other => other,
    }
}

fn mul_pow10(digits: &str, exp: usize) -> String {
    if digits == "0" {
        return "0".to_string();
    }
    let mut out = String::with_capacity(digits.len() + exp);
    out.push_str(digits);
    out.extend(std::iter::repeat('0').take(exp));
    out
}

fn mul_str_small(digits: &str, factor: u128) -> String {
    if digits == "0" {
        return "0".to_string();
    }
    if factor == 0 {
        return "0".to_string();
    }
    let mut carry = 0u128;
    let mut out = Vec::with_capacity(digits.len() + 1);
    for ch in digits.as_bytes().iter().rev() {
        let digit = (*ch - b'0') as u128;
        let prod = digit * factor + carry;
        out.push(((prod % 10) as u8) + b'0');
        carry = prod / 10;
    }
    while carry > 0 {
        out.push(((carry % 10) as u8) + b'0');
        carry /= 10;
    }
    out.reverse();
    String::from_utf8(out).unwrap_or_else(|_| "0".to_string())
}

fn format_quantity(value: &DecimalNumber, format: QuantityFormat) -> String {
    match format {
        QuantityFormat::Decimal => format_decimal_quantity(value),
        QuantityFormat::Binary => format_binary_quantity(value).unwrap_or_else(|| {
            format_decimal_plain(value)
        }),
    }
}

fn format_decimal_quantity(value: &DecimalNumber) -> String {
    if value.digits == "0" {
        return "0".to_string();
    }
    if value.scale >= 0 {
        return mul_pow10(&value.digits, value.scale as usize);
    }

    for (exp, suffix) in [(-3, "m"), (-6, "u"), (-9, "n")] {
        if exp <= value.scale {
            let shift = (value.scale - exp) as usize;
            let digits = mul_pow10(&value.digits, shift);
            return format!("{digits}{suffix}");
        }
    }

    format_decimal_plain(value)
}

fn format_binary_quantity(value: &DecimalNumber) -> Option<String> {
    let integer = decimal_to_integer_string(value)?;
    if integer == "0" {
        return Some("0".to_string());
    }
    for (exp, suffix) in [
        (60u32, "Ei"),
        (50u32, "Pi"),
        (40u32, "Ti"),
        (30u32, "Gi"),
        (20u32, "Mi"),
        (10u32, "Ki"),
    ] {
        let divisor = 1u128 << exp;
        if is_divisible_by(&integer, divisor) {
            let (quotient, _) = div_decimal_str_by_u128(&integer, divisor);
            return Some(format!("{quotient}{suffix}"));
        }
    }
    Some(integer)
}

fn decimal_to_integer_string(value: &DecimalNumber) -> Option<String> {
    if value.digits == "0" {
        return Some("0".to_string());
    }
    if value.scale >= 0 {
        return Some(mul_pow10(&value.digits, value.scale as usize));
    }
    let scale = (-value.scale) as usize;
    if value.digits.len() <= scale {
        return None;
    }
    if !ends_with_zeros(&value.digits, scale) {
        return None;
    }
    let cut = value.digits.len() - scale;
    let trimmed = value.digits[..cut].to_string();
    if trimmed.is_empty() {
        Some("0".to_string())
    } else {
        Some(trimmed)
    }
}

fn ends_with_zeros(value: &str, count: usize) -> bool {
    if count == 0 {
        return true;
    }
    value
        .as_bytes()
        .iter()
        .rev()
        .take(count)
        .all(|ch| *ch == b'0')
}

fn format_decimal_plain(value: &DecimalNumber) -> String {
    if value.scale >= 0 {
        return mul_pow10(&value.digits, value.scale as usize);
    }
    let scale = (-value.scale) as usize;
    if value.digits.len() > scale {
        let split = value.digits.len() - scale;
        format!(
            "{}.{}",
            &value.digits[..split],
            &value.digits[split..]
        )
    } else {
        let mut out = String::from("0.");
        out.extend(std::iter::repeat('0').take(scale - value.digits.len()));
        out.push_str(&value.digits);
        out
    }
}

fn is_divisible_by(value: &str, divisor: u128) -> bool {
    let (_, rem) = div_decimal_str_by_u128(value, divisor);
    rem == 0
}

fn div_decimal_str_by_u128(value: &str, divisor: u128) -> (String, u128) {
    let mut rem = 0u128;
    let mut out = String::new();
    for ch in value.chars() {
        let digit = ch.to_digit(10).unwrap_or(0) as u128;
        rem = rem * 10 + digit;
        let q = rem / divisor;
        rem %= divisor;
        if !out.is_empty() || q != 0 {
            out.push(std::char::from_digit(q as u32, 10).unwrap_or('0'));
        }
    }
    if out.is_empty() {
        out.push('0');
    }
    (out, rem)
}

pub fn apply_defaults_pod_spec(spec: &mut PodSpec) {
    if spec.dns_policy.is_empty() {
        spec.dns_policy = DNS_POLICY_CLUSTER_FIRST.to_string();
    }
    if spec.restart_policy.is_empty() {
        spec.restart_policy = RESTART_POLICY_ALWAYS.to_string();
    }
    if spec.security_context.is_none() {
        spec.security_context = Some(PodSecurityContext::default());
    }
    if spec.termination_grace_period_seconds.is_none() {
        spec.termination_grace_period_seconds = Some(DEFAULT_TERMINATION_GRACE_PERIOD_SECONDS);
    }
    if spec.scheduler_name.is_empty() {
        spec.scheduler_name = DEFAULT_SCHEDULER_NAME.to_string();
    }
    apply_defaults_resource_list(&mut spec.overhead);
    if let Some(resources) = spec.resources.as_mut() {
        apply_defaults_resource_requirements(resources);
    }

    for container in spec.containers.iter_mut() {
        apply_defaults_container(container);
    }
    for container in spec.init_containers.iter_mut() {
        apply_defaults_container(container);
    }
    for container in spec.ephemeral_containers.iter_mut() {
        apply_defaults_ephemeral_container(container);
    }
    for volume in spec.volumes.iter_mut() {
        apply_defaults_volume(volume);
    }
}

pub fn apply_defaults_container(container: &mut Container) {
    if container.image_pull_policy.is_empty() {
        container.image_pull_policy = default_pull_policy(&container.image);
    }
    if container.termination_message_path.is_empty() {
        container.termination_message_path = DEFAULT_TERMINATION_MESSAGE_PATH.to_string();
    }
    if container.termination_message_policy.is_empty() {
        container.termination_message_policy = TERMINATION_MESSAGE_READ_FILE.to_string();
    }
    for port in container.ports.iter_mut() {
        apply_defaults_container_port(port);
    }
    for env in container.env.iter_mut() {
        apply_defaults_env_var(env);
    }
    if let Some(probe) = container.liveness_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(probe) = container.readiness_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(probe) = container.startup_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(lifecycle) = container.lifecycle.as_mut() {
        apply_defaults_lifecycle(lifecycle);
    }
    if let Some(resources) = container.resources.as_mut() {
        apply_defaults_resource_requirements(resources);
    }
}

pub fn apply_defaults_resource_requirements(requirements: &mut ResourceRequirements) {
    apply_defaults_resource_list(&mut requirements.limits);
    apply_defaults_resource_list(&mut requirements.requests);
}

pub fn apply_defaults_volume_resource_requirements(requirements: &mut VolumeResourceRequirements) {
    apply_defaults_resource_list(&mut requirements.limits);
    apply_defaults_resource_list(&mut requirements.requests);
}

pub fn apply_defaults_resource_list(list: &mut ResourceList) {
    for value in list.values_mut() {
        value.round_up_milli();
    }
}

pub fn apply_defaults_resource_list_hash(list: &mut HashMap<ResourceName, Quantity>) {
    for value in list.values_mut() {
        value.round_up_milli();
    }
}

fn default_host_network_ports(containers: &mut [Container]) {
    for container in containers.iter_mut() {
        for port in container.ports.iter_mut() {
            let set_port = match port.host_port {
                None => true,
                Some(value) => value == 0,
            };
            if set_port {
                port.host_port = Some(port.container_port);
            }
        }
    }
}

pub fn apply_defaults_ephemeral_container(container: &mut EphemeralContainer) {
    if container.image_pull_policy.is_empty() {
        container.image_pull_policy = default_pull_policy(&container.image);
    }
    if container.termination_message_path.is_empty() {
        container.termination_message_path = DEFAULT_TERMINATION_MESSAGE_PATH.to_string();
    }
    if container.termination_message_policy.is_empty() {
        container.termination_message_policy = TERMINATION_MESSAGE_READ_FILE.to_string();
    }
    for port in container.ports.iter_mut() {
        apply_defaults_container_port(port);
    }
    for env in container.env.iter_mut() {
        apply_defaults_env_var(env);
    }
    if let Some(probe) = container.liveness_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(probe) = container.readiness_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(probe) = container.startup_probe.as_mut() {
        apply_defaults_probe(probe);
    }
    if let Some(lifecycle) = container.lifecycle.as_mut() {
        apply_defaults_lifecycle(lifecycle);
    }
    if let Some(resources) = container.resources.as_mut() {
        apply_defaults_resource_requirements(resources);
    }
}

pub fn apply_defaults_container_port(port: &mut ContainerPort) {
    if port.protocol.is_empty() {
        port.protocol = PROTOCOL_TCP.to_string();
    }
}

pub fn apply_defaults_env_var(env: &mut EnvVar) {
    if let Some(value_from) = env.value_from.as_mut() {
        apply_defaults_env_var_source(value_from);
    }
}

pub fn apply_defaults_env_var_source(source: &mut EnvVarSource) {
    if let Some(field_ref) = source.field_ref.as_mut() {
        apply_defaults_object_field_selector(field_ref);
    }
    if let Some(file_key_ref) = source.file_key_ref.as_mut() {
        apply_defaults_file_key_selector(file_key_ref);
    }
}

pub fn apply_defaults_object_field_selector(selector: &mut ObjectFieldSelector) {
    if selector.api_version.is_empty() {
        selector.api_version = "v1".to_string();
    }
}

pub fn apply_defaults_file_key_selector(selector: &mut FileKeySelector) {
    if selector.optional.is_none() {
        selector.optional = Some(false);
    }
}

pub fn apply_defaults_probe(probe: &mut Probe) {
    if probe.timeout_seconds.is_none() {
        probe.timeout_seconds = Some(DEFAULT_PROBE_TIMEOUT_SECONDS);
    }
    if probe.period_seconds.is_none() {
        probe.period_seconds = Some(DEFAULT_PROBE_PERIOD_SECONDS);
    }
    if probe.success_threshold.is_none() {
        probe.success_threshold = Some(DEFAULT_PROBE_SUCCESS_THRESHOLD);
    }
    if probe.failure_threshold.is_none() {
        probe.failure_threshold = Some(DEFAULT_PROBE_FAILURE_THRESHOLD);
    }
    if let Some(http_get) = probe.probe_handler.http_get.as_mut() {
        apply_defaults_http_get_action(http_get);
    }
    if let Some(grpc) = probe.probe_handler.grpc.as_mut() {
        apply_defaults_grpc_action(grpc);
    }
}

pub fn apply_defaults_http_get_action(action: &mut HTTPGetAction) {
    if action.path.is_empty() {
        action.path = "/".to_string();
    }
    if action.scheme.is_empty() {
        action.scheme = URI_SCHEME_HTTP.to_string();
    }
}

pub fn apply_defaults_grpc_action(action: &mut GRPCAction) {
    if action.service.is_none() {
        action.service = Some(String::new());
    }
}

pub fn apply_defaults_lifecycle(lifecycle: &mut Lifecycle) {
    if let Some(handler) = lifecycle.post_start.as_mut() {
        apply_defaults_lifecycle_handler(handler);
    }
    if let Some(handler) = lifecycle.pre_stop.as_mut() {
        apply_defaults_lifecycle_handler(handler);
    }
}

pub fn apply_defaults_lifecycle_handler(handler: &mut LifecycleHandler) {
    if let Some(http_get) = handler.http_get.as_mut() {
        apply_defaults_http_get_action(http_get);
    }
}

pub fn apply_defaults_volume(volume: &mut Volume) {
    apply_defaults_volume_source(&mut volume.volume_source);
    if volume_source_is_empty(&volume.volume_source) {
        volume.volume_source.empty_dir = Some(EmptyDirVolumeSource::default());
    }
}

pub fn apply_defaults_volume_source(source: &mut VolumeSource) {
    if let Some(host_path) = source.host_path.as_mut() {
        apply_defaults_host_path_volume_source(host_path);
    }
    if let Some(iscsi) = source.iscsi.as_mut() {
        apply_defaults_iscsi_volume_source(iscsi);
    }
    if let Some(rbd) = source.rbd.as_mut() {
        apply_defaults_rbd_volume_source(rbd);
    }
    if let Some(azure_disk) = source.azure_disk.as_mut() {
        apply_defaults_azure_disk_volume_source(azure_disk);
    }
    if let Some(scale_io) = source.scale_io.as_mut() {
        apply_defaults_scale_io_volume_source(scale_io);
    }
    if let Some(secret) = source.secret.as_mut() {
        apply_defaults_secret_volume_source(secret);
    }
    if let Some(config_map) = source.config_map.as_mut() {
        apply_defaults_config_map_volume_source(config_map);
    }
    if let Some(downward_api) = source.downward_a_p_i.as_mut() {
        apply_defaults_downward_api_volume_source(downward_api);
    }
    if let Some(projected) = source.projected.as_mut() {
        apply_defaults_projected_volume_source(projected);
    }
    if let Some(ephemeral) = source.ephemeral.as_mut() {
        apply_defaults_ephemeral_volume_source(ephemeral);
    }
    if let Some(image) = source.image.as_mut() {
        apply_defaults_image_volume_source(image);
    }
}

pub fn apply_defaults_host_path_volume_source(source: &mut HostPathVolumeSource) {
    if source.host_path_type.is_none() {
        source.host_path_type = Some(HOST_PATH_UNSET.to_string());
    }
}

pub fn apply_defaults_secret_volume_source(source: &mut SecretVolumeSource) {
    if source.default_mode.is_none() {
        source.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
}

pub fn apply_defaults_config_map_volume_source(source: &mut ConfigMapVolumeSource) {
    if source.default_mode.is_none() {
        source.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
}

pub fn apply_defaults_downward_api_volume_source(source: &mut DownwardAPIVolumeSource) {
    if source.default_mode.is_none() {
        source.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
    for item in source.items.iter_mut() {
        if let Some(field_ref) = item.field_ref.as_mut() {
            apply_defaults_object_field_selector(field_ref);
        }
    }
}

pub fn apply_defaults_projected_volume_source(source: &mut ProjectedVolumeSource) {
    if source.default_mode.is_none() {
        source.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
    for projection in source.sources.iter_mut() {
        if let Some(downward) = projection.downward_a_p_i.as_mut() {
            for item in downward.items.iter_mut() {
                if let Some(field_ref) = item.field_ref.as_mut() {
                    apply_defaults_object_field_selector(field_ref);
                }
            }
        }
        if let Some(token) = projection.service_account_token.as_mut() {
            apply_defaults_service_account_token_projection(token);
        }
    }
}

pub fn apply_defaults_ephemeral_volume_source(source: &mut EphemeralVolumeSource) {
    if let Some(template) = source.volume_claim_template.as_mut() {
        apply_defaults_persistent_volume_claim_spec(&mut template.spec);
    }
}

pub fn apply_defaults_image_volume_source(source: &mut ImageVolumeSource) {
    if !feature_image_volume_enabled() {
        return;
    }
    if source.pull_policy.is_empty() {
        source.pull_policy = default_pull_policy(&source.reference);
    }
}

pub fn apply_defaults_iscsi_volume_source(source: &mut ISCSIVolumeSource) {
    if source.iscsi_interface.is_empty() {
        source.iscsi_interface = DEFAULT_ISCSI_INTERFACE.to_string();
    }
}

pub fn apply_defaults_rbd_volume_source(source: &mut RBDVolumeSource) {
    if source.pool.is_empty() {
        source.pool = DEFAULT_RBD_POOL.to_string();
    }
    if source.user.is_empty() {
        source.user = DEFAULT_RBD_USER.to_string();
    }
    if source.keyring.is_empty() {
        source.keyring = DEFAULT_RBD_KEYRING.to_string();
    }
}

pub fn apply_defaults_scale_io_volume_source(source: &mut ScaleIOVolumeSource) {
    if source.storage_mode.is_empty() {
        source.storage_mode = DEFAULT_SCALE_IO_STORAGE_MODE.to_string();
    }
    if source.fs_type.is_empty() {
        source.fs_type = DEFAULT_SCALE_IO_FSTYPE.to_string();
    }
}

pub fn apply_defaults_azure_disk_volume_source(source: &mut AzureDiskVolumeSource) {
    if source.caching_mode.is_none() {
        source.caching_mode = Some(AZURE_DATA_DISK_CACHING_READ_WRITE.to_string());
    }
    if source.fs_type.is_none() {
        source.fs_type = Some(DEFAULT_AZURE_DISK_FSTYPE.to_string());
    }
    if source.kind.is_none() {
        source.kind = Some(AZURE_DATA_DISK_KIND_SHARED.to_string());
    }
}

pub fn apply_defaults_service_account_token_projection(projection: &mut ServiceAccountTokenProjection) {
    if projection.expiration_seconds.is_none() {
        projection.expiration_seconds = Some(DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS);
    }
}

pub fn apply_defaults_secret(secret: &mut Secret) {
    if secret.secret_type.is_empty() {
        secret.secret_type = SECRET_TYPE_OPAQUE.to_string();
    }
}

pub fn apply_defaults_config_map(config_map: &mut ConfigMap) {
    if config_map.data.is_empty() {
        config_map.data = BTreeMap::new();
    }
}

pub fn apply_defaults_config_map_list(list: &mut ConfigMapList) {
    for config_map in list.items.iter_mut() {
        apply_defaults_config_map(config_map);
    }
}

pub fn apply_defaults_secret_list(list: &mut SecretList) {
    for secret in list.items.iter_mut() {
        apply_defaults_secret(secret);
    }
}

pub fn apply_defaults_persistent_volume(volume: &mut PersistentVolume) {
    if let Some(spec) = volume.spec.as_mut() {
        apply_defaults_persistent_volume_spec(spec);
    }
    if let Some(status) = volume.status.as_mut() {
        if status.phase.is_empty() {
            status.phase = PV_PHASE_PENDING.to_string();
        }
    }
}

pub fn apply_defaults_persistent_volume_list(list: &mut PersistentVolumeList) {
    for volume in list.items.iter_mut() {
        apply_defaults_persistent_volume(volume);
    }
}

pub fn apply_defaults_persistent_volume_spec(spec: &mut PersistentVolumeSpec) {
    apply_defaults_resource_list(&mut spec.capacity);
    apply_defaults_persistent_volume_source(&mut spec.persistent_volume_source);
    if spec.persistent_volume_reclaim_policy.is_empty() {
        spec.persistent_volume_reclaim_policy = PV_RECLAIM_RETAIN.to_string();
    }
    if spec.volume_mode.is_none() {
        spec.volume_mode = Some(PV_MODE_FILESYSTEM.to_string());
    }
}

pub fn apply_defaults_persistent_volume_source(source: &mut PersistentVolumeSource) {
    if let Some(host_path) = source.host_path.as_mut() {
        apply_defaults_host_path_volume_source(host_path);
    }
    if let Some(iscsi) = source.iscsi.as_mut() {
        apply_defaults_iscsi_persistent_volume_source(iscsi);
    }
    if let Some(rbd) = source.rbd.as_mut() {
        apply_defaults_rbd_persistent_volume_source(rbd);
    }
    if let Some(azure_disk) = source.azure_disk.as_mut() {
        apply_defaults_azure_disk_volume_source(azure_disk);
    }
    if let Some(scale_io) = source.scale_io.as_mut() {
        apply_defaults_scale_io_persistent_volume_source(scale_io);
    }
}

pub fn apply_defaults_iscsi_persistent_volume_source(source: &mut ISCSIPersistentVolumeSource) {
    if source.iscsi_interface.is_empty() {
        source.iscsi_interface = DEFAULT_ISCSI_INTERFACE.to_string();
    }
}

pub fn apply_defaults_rbd_persistent_volume_source(source: &mut RBDPersistentVolumeSource) {
    if source.pool.is_empty() {
        source.pool = DEFAULT_RBD_POOL.to_string();
    }
    if source.user.is_empty() {
        source.user = DEFAULT_RBD_USER.to_string();
    }
    if source.keyring.is_empty() {
        source.keyring = DEFAULT_RBD_KEYRING.to_string();
    }
}

pub fn apply_defaults_scale_io_persistent_volume_source(source: &mut ScaleIOPersistentVolumeSource) {
    if source.storage_mode.is_empty() {
        source.storage_mode = DEFAULT_SCALE_IO_STORAGE_MODE.to_string();
    }
    if source.fs_type.is_empty() {
        source.fs_type = DEFAULT_SCALE_IO_FSTYPE.to_string();
    }
}

pub fn apply_defaults_persistent_volume_claim(claim: &mut PersistentVolumeClaim) {
    if let Some(spec) = claim.spec.as_mut() {
        apply_defaults_persistent_volume_claim_spec(spec);
    }
    if let Some(status) = claim.status.as_mut() {
        apply_defaults_persistent_volume_claim_status(status);
        if status.phase.is_empty() {
            status.phase = PVC_PHASE_PENDING.to_string();
        }
    }
}

pub fn apply_defaults_persistent_volume_claim_list(list: &mut PersistentVolumeClaimList) {
    for claim in list.items.iter_mut() {
        apply_defaults_persistent_volume_claim(claim);
    }
}

pub fn apply_defaults_persistent_volume_claim_spec(spec: &mut PersistentVolumeClaimSpec) {
    if let Some(resources) = spec.resources.as_mut() {
        apply_defaults_volume_resource_requirements(resources);
    }
    if spec.volume_mode.is_none() {
        spec.volume_mode = Some(PV_MODE_FILESYSTEM.to_string());
    }
}

pub fn apply_defaults_persistent_volume_claim_status(status: &mut PersistentVolumeClaimStatus) {
    apply_defaults_resource_list(&mut status.capacity);
    apply_defaults_resource_list(&mut status.allocated_resources);
}

pub fn apply_defaults_endpoints(endpoints: &mut Endpoints) {
    for subset in endpoints.subsets.iter_mut() {
        for port in subset.ports.iter_mut() {
            if port.protocol.is_empty() {
                port.protocol = PROTOCOL_TCP.to_string();
            }
        }
    }
}

pub fn apply_defaults_endpoints_list(list: &mut EndpointsList) {
    for endpoint in list.items.iter_mut() {
        apply_defaults_endpoints(endpoint);
    }
}

pub fn apply_defaults_node(node: &mut Node) {
    if let Some(status) = node.status.as_mut() {
        apply_defaults_node_status(status);
    }
}

pub fn apply_defaults_node_status(status: &mut NodeStatus) {
    if status.allocatable.is_empty() && !status.capacity.is_empty() {
        status.allocatable = status.capacity.clone();
    }
    apply_defaults_resource_list(&mut status.capacity);
    apply_defaults_resource_list(&mut status.allocatable);
}

pub fn apply_defaults_node_list(list: &mut NodeList) {
    for node in list.items.iter_mut() {
        apply_defaults_node(node);
    }
}

pub fn apply_defaults_service(service: &mut Service) {
    if let Some(spec) = service.spec.as_mut() {
        apply_defaults_service_spec(spec);
    }
    if let Some(status) = service.status.as_mut() {
        apply_defaults_service_status(status, service.spec.as_ref());
    }
}

pub fn apply_defaults_service_list(list: &mut ServiceList) {
    for service in list.items.iter_mut() {
        apply_defaults_service(service);
    }
}

pub fn apply_defaults_service_spec(spec: &mut ServiceSpec) {
    if spec.session_affinity.is_empty() {
        spec.session_affinity = SERVICE_AFFINITY_NONE.to_string();
    }
    if spec.session_affinity == SERVICE_AFFINITY_NONE {
        spec.session_affinity_config = None;
    }
    if spec.session_affinity == SERVICE_AFFINITY_CLIENT_IP {
        let timeout = DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS;
        let config = spec
            .session_affinity_config
            .get_or_insert_with(SessionAffinityConfig::default);
        let client_ip = config.client_i_p.get_or_insert_with(ClientIPConfig::default);
        if client_ip.timeout_seconds.is_none() {
            client_ip.timeout_seconds = Some(timeout);
        }
    }
    if spec.service_type.is_empty() {
        spec.service_type = SERVICE_TYPE_CLUSTER_IP.to_string();
    }
    for port in spec.ports.iter_mut() {
        if port.protocol.is_empty() {
            port.protocol = PROTOCOL_TCP.to_string();
        }
        let set_target = match port.target_port.as_ref() {
            None => true,
            Some(IntOrString::Int(value)) => *value == 0,
            Some(IntOrString::String(value)) => value.is_empty(),
        };
        if set_target {
            port.target_port = Some(IntOrString::from(port.port));
        }
    }
    let externally_accessible = spec.service_type == SERVICE_TYPE_LOAD_BALANCER
        || spec.service_type == SERVICE_TYPE_NODE_PORT
        || (spec.service_type == SERVICE_TYPE_CLUSTER_IP && !spec.external_i_ps.is_empty());
    if externally_accessible {
        if spec.external_traffic_policy.is_empty() {
            spec.external_traffic_policy = SERVICE_EXTERNAL_TRAFFIC_POLICY_CLUSTER.to_string();
        }
    }
    if spec.internal_traffic_policy.is_none()
        && (spec.service_type == SERVICE_TYPE_CLUSTER_IP
            || spec.service_type == SERVICE_TYPE_NODE_PORT
            || spec.service_type == SERVICE_TYPE_LOAD_BALANCER)
    {
        spec.internal_traffic_policy = Some(SERVICE_INTERNAL_TRAFFIC_POLICY_CLUSTER.to_string());
    }
    if spec.service_type == SERVICE_TYPE_LOAD_BALANCER
        && spec.allocate_load_balancer_node_ports.is_none()
    {
        spec.allocate_load_balancer_node_ports = Some(true);
    }
}

fn apply_defaults_service_status(status: &mut ServiceStatus, spec: Option<&ServiceSpec>) {
    let spec = match spec {
        Some(spec) => spec,
        None => return,
    };
    if spec.service_type != SERVICE_TYPE_LOAD_BALANCER {
        return;
    }
    if let Some(load_balancer) = status.load_balancer.as_mut() {
        for ingress in load_balancer.ingress.iter_mut() {
            if !ingress.ip.is_empty() && ingress.ip_mode.is_none() {
                ingress.ip_mode = Some(LOAD_BALANCER_IP_MODE_VIP.to_string());
            }
        }
    }
}

pub fn apply_defaults_replication_controller(controller: &mut ReplicationController) {
    let spec = match controller.spec.as_mut() {
        Some(spec) => spec,
        None => return,
    };
    if spec.replicas.is_none() {
        spec.replicas = Some(1);
    }
    if let Some(template) = spec.template.as_mut() {
        if let Some(pod_spec) = template.spec.as_mut() {
            apply_defaults_pod_spec(pod_spec);
        }
        let labels = template.metadata.labels.clone();
        if !labels.is_empty() {
            if spec.selector.is_empty() {
                spec.selector = labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            }
            if controller.metadata.labels.is_empty() {
                controller.metadata.labels = labels;
            }
        }
    }
}

pub fn apply_defaults_replication_controller_list(list: &mut ReplicationControllerList) {
    for controller in list.items.iter_mut() {
        apply_defaults_replication_controller(controller);
    }
}

pub fn apply_defaults_limit_range(limit_range: &mut LimitRange) {
    if let Some(spec) = limit_range.spec.as_mut() {
        for item in spec.limits.iter_mut() {
            apply_defaults_limit_range_item(item);
        }
    }
}

pub fn apply_defaults_limit_range_list(list: &mut LimitRangeList) {
    for limit_range in list.items.iter_mut() {
        apply_defaults_limit_range(limit_range);
    }
}

pub fn apply_defaults_limit_range_item(item: &mut LimitRangeItem) {
    apply_defaults_resource_list_hash(&mut item.max);
    apply_defaults_resource_list_hash(&mut item.min);
    apply_defaults_resource_list_hash(&mut item.default);
    apply_defaults_resource_list_hash(&mut item.default_request);
    apply_defaults_resource_list_hash(&mut item.max_limit_request_ratio);
    if item.limit_type != LIMIT_TYPE_CONTAINER {
        return;
    }
    for (key, value) in item.max.clone() {
        item.default.entry(key).or_insert(value);
    }
    for (key, value) in item.default.clone() {
        item.default_request.entry(key).or_insert(value);
    }
    for (key, value) in item.min.clone() {
        item.default_request.entry(key).or_insert(value);
    }
}

pub fn apply_defaults_resource_quota(quota: &mut ResourceQuota) {
    if let Some(spec) = quota.spec.as_mut() {
        apply_defaults_resource_quota_spec(spec);
    }
    if let Some(status) = quota.status.as_mut() {
        apply_defaults_resource_quota_status(status);
    }
}

pub fn apply_defaults_resource_quota_list(list: &mut ResourceQuotaList) {
    for quota in list.items.iter_mut() {
        apply_defaults_resource_quota(quota);
    }
}

pub fn apply_defaults_resource_quota_spec(spec: &mut ResourceQuotaSpec) {
    apply_defaults_resource_list_hash(&mut spec.hard);
}

pub fn apply_defaults_resource_quota_status(status: &mut ResourceQuotaStatus) {
    apply_defaults_resource_list_hash(&mut status.hard);
    apply_defaults_resource_list_hash(&mut status.used);
}

pub fn apply_defaults_namespace(namespace: &mut Namespace) {
    let name = namespace.metadata.name.clone();
    if !name.is_empty() {
        namespace
            .metadata
            .labels
            .insert(NAMESPACE_METADATA_LABEL.to_string(), name);
    }
    if let Some(status) = namespace.status.as_mut() {
        apply_defaults_namespace_status(status);
    }
}

pub fn apply_defaults_namespace_status(status: &mut NamespaceStatus) {
    if status.phase.is_empty() {
        status.phase = NAMESPACE_PHASE_ACTIVE.to_string();
    }
}

pub fn apply_defaults_namespace_list(list: &mut NamespaceList) {
    for namespace in list.items.iter_mut() {
        apply_defaults_namespace(namespace);
    }
}

pub fn apply_defaults_pod_log_options(options: &mut PodLogOptions) {
    if !feature_pod_logs_query_split_streams_enabled() {
        return;
    }
    if options.stream.is_none() {
        options.stream = Some(LOG_STREAM_ALL.to_string());
    }
}

fn volume_source_is_empty(source: &VolumeSource) -> bool {
    source.host_path.is_none()
        && source.empty_dir.is_none()
        && source.secret.is_none()
        && source.config_map.is_none()
        && source.persistent_volume_claim.is_none()
        && source.nfs.is_none()
        && source.projected.is_none()
        && source.downward_a_p_i.is_none()
        && source.csi.is_none()
        && source.gce_persistent_disk.is_none()
        && source.aws_elastic_block_store.is_none()
        && source.git_repo.is_none()
        && source.iscsi.is_none()
        && source.glusterfs.is_none()
        && source.rbd.is_none()
        && source.flex_volume.is_none()
        && source.cinder.is_none()
        && source.cephfs.is_none()
        && source.flocker.is_none()
        && source.fc.is_none()
        && source.azure_file.is_none()
        && source.vsphere_volume.is_none()
        && source.quobyte.is_none()
        && source.azure_disk.is_none()
        && source.photon_persistent_disk.is_none()
        && source.portworx_volume.is_none()
        && source.scale_io.is_none()
        && source.storageos.is_none()
        && source.ephemeral.is_none()
        && source.image.is_none()
}

fn default_pull_policy(image: &str) -> String {
    if image_tag(image) == "latest" {
        PULL_ALWAYS.to_string()
    } else {
        PULL_IF_NOT_PRESENT.to_string()
    }
}

fn image_tag(image: &str) -> &str {
    let name = image.split('@').next().unwrap_or(image);
    let last_slash = name.rfind('/').unwrap_or(0);
    let search = &name[last_slash..];
    if let Some(colon) = search.rfind(':') {
        let tag = &search[colon + 1..];
        if !tag.is_empty() {
            return tag;
        }
    }
    "latest"
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api_core::IntOrString;
    use k8s_api_core::Quantity;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;
    use std::collections::BTreeMap;

    struct FeatureOverride<'a> {
        _lock: std::sync::MutexGuard<'a, ()>,
        prev_image_volume: bool,
        prev_pod_logs: bool,
        prev_pod_level: bool,
    }

    impl<'a> FeatureOverride<'a> {
        fn new(image_volume: bool, pod_logs: bool, pod_level: bool) -> Self {
            let lock = FEATURE_LOCK.lock().expect("feature lock");
            let prev_image_volume = feature_image_volume_enabled();
            let prev_pod_logs = feature_pod_logs_query_split_streams_enabled();
            let prev_pod_level = feature_pod_level_resources_enabled();
            set_feature_image_volume(image_volume);
            set_feature_pod_logs_query_split_streams(pod_logs);
            set_feature_pod_level_resources(pod_level);
            Self {
                _lock: lock,
                prev_image_volume,
                prev_pod_logs,
                prev_pod_level,
            }
        }
    }

    impl Drop for FeatureOverride<'_> {
        fn drop(&mut self) {
            set_feature_image_volume(self.prev_image_volume);
            set_feature_pod_logs_query_split_streams(self.prev_pod_logs);
            set_feature_pod_level_resources(self.prev_pod_level);
        }
    }

    #[test]
    fn test_default_pod_spec() {
        let mut spec = PodSpec::default();
        apply_defaults_pod_spec(&mut spec);
        assert_eq!(spec.dns_policy, DNS_POLICY_CLUSTER_FIRST);
        assert_eq!(spec.restart_policy, RESTART_POLICY_ALWAYS);
        assert_eq!(
            spec.termination_grace_period_seconds,
            Some(DEFAULT_TERMINATION_GRACE_PERIOD_SECONDS)
        );
        assert_eq!(spec.scheduler_name, DEFAULT_SCHEDULER_NAME);
        assert!(spec.security_context.is_some());
    }

    #[test]
    fn test_default_pod_spec_does_not_set_enable_service_links() {
        let mut spec = PodSpec::default();
        apply_defaults_pod_spec(&mut spec);
        assert!(spec.enable_service_links.is_none());
    }

    #[test]
    fn test_default_pod_enable_service_links() {
        let mut pod = Pod {
            spec: Some(PodSpec::default()),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.expect("spec should be set");
        assert_eq!(spec.enable_service_links, Some(DEFAULT_ENABLE_SERVICE_LINKS));
    }

    #[test]
    fn test_default_container() {
        let mut container = Container {
            name: "c".to_string(),
            image: "nginx:1.28".to_string(),
            ..Default::default()
        };
        apply_defaults_container(&mut container);
        assert_eq!(container.image_pull_policy, PULL_IF_NOT_PRESENT);
        assert_eq!(
            container.termination_message_path,
            DEFAULT_TERMINATION_MESSAGE_PATH
        );
        assert_eq!(
            container.termination_message_policy,
            TERMINATION_MESSAGE_READ_FILE
        );
    }

    #[test]
    fn test_default_container_latest_tag() {
        let mut container = Container {
            name: "c".to_string(),
            image: "nginx:latest".to_string(),
            ..Default::default()
        };
        apply_defaults_container(&mut container);
        assert_eq!(container.image_pull_policy, PULL_ALWAYS);
    }

    #[test]
    fn test_default_ephemeral_container_defaults() {
        let mut container = EphemeralContainer {
            name: "debug".to_string(),
            image: "nginx:latest".to_string(),
            ports: vec![ContainerPort {
                container_port: 8080,
                ..Default::default()
            }],
            liveness_probe: Some(Probe {
                probe_handler: ProbeHandler {
                    grpc: Some(GRPCAction {
                        port: 443,
                        service: None,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_ephemeral_container(&mut container);
        assert_eq!(container.image_pull_policy, PULL_ALWAYS);
        assert_eq!(
            container.termination_message_path,
            DEFAULT_TERMINATION_MESSAGE_PATH
        );
        assert_eq!(
            container.termination_message_policy,
            TERMINATION_MESSAGE_READ_FILE
        );
        assert_eq!(container.ports[0].protocol, PROTOCOL_TCP);
        let grpc = container
            .liveness_probe
            .as_ref()
            .expect("probe should be set")
            .probe_handler
            .grpc
            .as_ref()
            .expect("grpc should be set");
        assert_eq!(grpc.service.as_deref(), Some(""));
    }

    #[test]
    fn test_default_host_network_ports() {
        let mut pod = Pod {
            spec: Some(PodSpec {
                host_network: true,
                containers: vec![Container {
                    name: "c".to_string(),
                    ports: vec![ContainerPort {
                        container_port: 123,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.expect("spec should be set");
        let port = spec
            .containers
            .first()
            .expect("container should exist")
            .ports
            .first()
            .expect("port should exist");
        assert_eq!(port.host_port, Some(123));
    }

    #[test]
    fn test_default_host_network_ports_pod_vs_spec() {
        let port_num = 12345;
        let base_container = Container {
            name: "c".to_string(),
            image: "nginx:1.28".to_string(),
            ports: vec![ContainerPort {
                container_port: port_num,
                ..Default::default()
            }],
            ..Default::default()
        };

        let mut pod = Pod {
            spec: Some(PodSpec {
                host_network: true,
                containers: vec![base_container.clone()],
                init_containers: vec![base_container.clone()],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let pod_spec = pod.spec.as_ref().expect("spec should be set");
        assert_eq!(
            pod_spec.containers[0].ports[0].host_port,
            Some(port_num)
        );
        assert_eq!(
            pod_spec.init_containers[0].ports[0].host_port,
            Some(port_num)
        );

        let mut spec = PodSpec {
            host_network: true,
            containers: vec![base_container.clone()],
            init_containers: vec![base_container],
            ..Default::default()
        };
        apply_defaults_pod_spec(&mut spec);
        assert!(spec.containers[0].ports[0].host_port.is_none());
        assert!(spec.init_containers[0].ports[0].host_port.is_none());
    }

    #[test]
    fn test_default_probe_http_get() {
        let mut probe = Probe {
            probe_handler: ProbeHandler {
                http_get: Some(HTTPGetAction {
                    port: IntOrString::from(80),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        apply_defaults_probe(&mut probe);
        assert_eq!(probe.timeout_seconds, Some(DEFAULT_PROBE_TIMEOUT_SECONDS));
        assert_eq!(probe.period_seconds, Some(DEFAULT_PROBE_PERIOD_SECONDS));
        assert_eq!(probe.success_threshold, Some(DEFAULT_PROBE_SUCCESS_THRESHOLD));
        assert_eq!(probe.failure_threshold, Some(DEFAULT_PROBE_FAILURE_THRESHOLD));
        let http_get = probe
            .probe_handler
            .http_get
            .as_ref()
            .expect("http_get should be set");
        assert_eq!(http_get.path, "/");
        assert_eq!(http_get.scheme, URI_SCHEME_HTTP);
    }

    #[test]
    fn test_default_lifecycle_http_get() {
        let mut lifecycle = Lifecycle {
            post_start: Some(LifecycleHandler {
                http_get: Some(HTTPGetAction {
                    port: IntOrString::from(80),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_lifecycle(&mut lifecycle);
        let http_get = lifecycle
            .post_start
            .as_ref()
            .expect("post_start should be set")
            .http_get
            .as_ref()
            .expect("http_get should be set");
        assert_eq!(http_get.path, "/");
        assert_eq!(http_get.scheme, URI_SCHEME_HTTP);
    }

    #[test]
    fn test_default_env_var_source() {
        let mut env = EnvVar {
            name: "ENV".to_string(),
            value_from: Some(EnvVarSource {
                field_ref: Some(ObjectFieldSelector {
                    field_path: "metadata.name".to_string(),
                    ..Default::default()
                }),
                file_key_ref: Some(FileKeySelector {
                    volume_name: "vol".to_string(),
                    path: "path".to_string(),
                    key: "key".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_env_var(&mut env);
        let value_from = env.value_from.as_ref().expect("value_from should be set");
        let field_ref = value_from.field_ref.as_ref().expect("field_ref should be set");
        assert_eq!(field_ref.api_version, "v1");
        let file_key_ref = value_from
            .file_key_ref
            .as_ref()
            .expect("file_key_ref should be set");
        assert_eq!(file_key_ref.optional, Some(false));
    }

    #[test]
    fn test_default_projected_volume_source() {
        let mut source = ProjectedVolumeSource {
            sources: vec![VolumeProjection {
                downward_a_p_i: Some(DownwardAPIProjection {
                    items: vec![DownwardAPIVolumeFile {
                        path: "path".to_string(),
                        field_ref: Some(ObjectFieldSelector {
                            field_path: "metadata.name".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                }),
                service_account_token: Some(ServiceAccountTokenProjection {
                    path: "token".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        };
        apply_defaults_projected_volume_source(&mut source);
        assert_eq!(source.default_mode, Some(DEFAULT_VOLUME_MODE));
        let projection = source.sources.first().expect("projection should exist");
        let downward = projection
            .downward_a_p_i
            .as_ref()
            .expect("downward should be set");
        let item = downward.items.first().expect("downward item should exist");
        let field_ref = item.field_ref.as_ref().expect("field_ref should be set");
        assert_eq!(field_ref.api_version, "v1");
        let token = projection
            .service_account_token
            .as_ref()
            .expect("service account token should be set");
        assert_eq!(
            token.expiration_seconds,
            Some(DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS)
        );
    }

    #[test]
    fn test_default_volume_empty_dir() {
        let mut volume = Volume {
            name: "vol".to_string(),
            ..Default::default()
        };
        apply_defaults_volume(&mut volume);
        assert!(volume.volume_source.empty_dir.is_some());
    }

    #[test]
    fn test_default_host_path_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                host_path: Some(HostPathVolumeSource {
                    path: "/tmp".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let host_path = volume
            .volume_source
            .host_path
            .as_ref()
            .expect("host_path should be set");
        assert_eq!(
            host_path.host_path_type.as_deref(),
            Some(HOST_PATH_UNSET)
        );
    }

    #[test]
    fn test_default_secret_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                secret: Some(SecretVolumeSource {
                    secret_name: "secret".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let secret = volume
            .volume_source
            .secret
            .as_ref()
            .expect("secret should be set");
        assert_eq!(secret.default_mode, Some(DEFAULT_VOLUME_MODE));
    }

    #[test]
    fn test_default_config_map_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                config_map: Some(ConfigMapVolumeSource {
                    name: "config".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let config_map = volume
            .volume_source
            .config_map
            .as_ref()
            .expect("config_map should be set");
        assert_eq!(config_map.default_mode, Some(DEFAULT_VOLUME_MODE));
    }

    #[test]
    fn test_default_downward_api_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                downward_a_p_i: Some(DownwardAPIVolumeSource {
                    items: vec![DownwardAPIVolumeFile {
                        path: "path".to_string(),
                        field_ref: Some(ObjectFieldSelector {
                            field_path: "metadata.name".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let downward = volume
            .volume_source
            .downward_a_p_i
            .as_ref()
            .expect("downward should be set");
        assert_eq!(downward.default_mode, Some(DEFAULT_VOLUME_MODE));
        let item = downward.items.first().expect("item should be set");
        let field_ref = item.field_ref.as_ref().expect("field_ref should be set");
        assert_eq!(field_ref.api_version, "v1");
    }

    #[test]
    fn test_default_secret_type() {
        let mut secret = Secret::new("secret");
        apply_defaults_secret(&mut secret);
        assert_eq!(secret.secret_type, SECRET_TYPE_OPAQUE);
    }

    #[test]
    fn test_default_persistent_volume() {
        let mut volume = PersistentVolume {
            spec: Some(PersistentVolumeSpec::default()),
            status: Some(PersistentVolumeStatus::default()),
            ..Default::default()
        };
        apply_defaults_persistent_volume(&mut volume);
        let spec = volume.spec.expect("spec should be set");
        assert_eq!(spec.persistent_volume_reclaim_policy, PV_RECLAIM_RETAIN);
        assert_eq!(spec.volume_mode, Some(PV_MODE_FILESYSTEM.to_string()));
        let status = volume.status.expect("status should be set");
        assert_eq!(status.phase, PV_PHASE_PENDING);
    }

    #[test]
    fn test_default_persistent_volume_claim() {
        let mut claim = PersistentVolumeClaim {
            spec: Some(PersistentVolumeClaimSpec::default()),
            status: Some(PersistentVolumeClaimStatus::default()),
            ..Default::default()
        };
        apply_defaults_persistent_volume_claim(&mut claim);
        let spec = claim.spec.expect("spec should be set");
        assert_eq!(spec.volume_mode, Some(PV_MODE_FILESYSTEM.to_string()));
        let status = claim.status.expect("status should be set");
        assert_eq!(status.phase, PVC_PHASE_PENDING);
    }

    #[test]
    fn test_default_endpoints_protocol() {
        let mut endpoints = Endpoints {
            subsets: vec![EndpointSubset {
                ports: vec![EndpointPort {
                    port: 80,
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        apply_defaults_endpoints(&mut endpoints);
        let port = endpoints
            .subsets
            .first()
            .expect("subset should exist")
            .ports
            .first()
            .expect("port should exist");
        assert_eq!(port.protocol, PROTOCOL_TCP);
    }

    #[test]
    fn test_default_node_status_allocatable() {
        let mut capacity = BTreeMap::new();
        capacity.insert("cpu".to_string(), Quantity::new("1"));
        let mut status = NodeStatus {
            capacity,
            ..Default::default()
        };
        apply_defaults_node_status(&mut status);
        assert_eq!(status.allocatable.len(), 1);
        assert_eq!(status.allocatable.get("cpu").map(Quantity::as_str), Some("1"));
    }

    #[test]
    fn test_default_node_status_allocatable_preserve() {
        let mut capacity = BTreeMap::new();
        capacity.insert("cpu".to_string(), Quantity::new("1"));
        let mut allocatable = BTreeMap::new();
        allocatable.insert("cpu".to_string(), Quantity::new("500m"));
        let mut status = NodeStatus {
            capacity,
            allocatable,
            ..Default::default()
        };
        apply_defaults_node_status(&mut status);
        assert_eq!(
            status.allocatable.get("cpu").map(Quantity::as_str),
            Some("500m")
        );
    }

    #[test]
    fn test_default_node_status_allocatable_no_capacity() {
        let mut status = NodeStatus::default();
        apply_defaults_node_status(&mut status);
        assert!(status.allocatable.is_empty());
    }

    #[test]
    fn test_default_service_spec() {
        let mut spec = ServiceSpec::default();
        apply_defaults_service_spec(&mut spec);
        assert_eq!(spec.session_affinity, SERVICE_AFFINITY_NONE);
        assert_eq!(spec.service_type, SERVICE_TYPE_CLUSTER_IP);
        assert_eq!(
            spec.internal_traffic_policy,
            Some(SERVICE_INTERNAL_TRAFFIC_POLICY_CLUSTER.to_string())
        );
    }

    #[test]
    fn test_default_service_session_affinity_none_clears_config() {
        let mut spec = ServiceSpec {
            session_affinity: SERVICE_AFFINITY_NONE.to_string(),
            session_affinity_config: Some(SessionAffinityConfig {
                client_i_p: Some(ClientIPConfig {
                    timeout_seconds: Some(123),
                }),
            }),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert!(spec.session_affinity_config.is_none());
    }

    #[test]
    fn test_default_service_internal_traffic_policy_external_name() {
        let mut spec = ServiceSpec {
            service_type: SERVICE_TYPE_EXTERNAL_NAME.to_string(),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert!(spec.internal_traffic_policy.is_none());
    }

    #[test]
    fn test_default_service_port() {
        let mut spec = ServiceSpec {
            ports: vec![ServicePort {
                port: 80,
                ..Default::default()
            }],
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        let port = spec.ports.first().expect("port should exist");
        assert_eq!(port.protocol, PROTOCOL_TCP);
        assert_eq!(port.target_port, Some(IntOrString::from(80)));
    }

    #[test]
    fn test_default_service_target_port_zero_or_empty() {
        let mut spec = ServiceSpec {
            ports: vec![
                ServicePort {
                    port: 80,
                    target_port: Some(IntOrString::Int(0)),
                    ..Default::default()
                },
                ServicePort {
                    port: 443,
                    target_port: Some(IntOrString::String(String::new())),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert_eq!(spec.ports[0].target_port, Some(IntOrString::from(80)));
        assert_eq!(spec.ports[1].target_port, Some(IntOrString::from(443)));
    }

    #[test]
    fn test_default_service_node_port_external_traffic_policy() {
        let mut spec = ServiceSpec {
            service_type: SERVICE_TYPE_NODE_PORT.to_string(),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert_eq!(
            spec.external_traffic_policy,
            SERVICE_EXTERNAL_TRAFFIC_POLICY_CLUSTER
        );
        assert_eq!(
            spec.internal_traffic_policy,
            Some(SERVICE_INTERNAL_TRAFFIC_POLICY_CLUSTER.to_string())
        );
    }

    #[test]
    fn test_default_service_load_balancer_allocate_node_ports() {
        let mut spec = ServiceSpec {
            service_type: SERVICE_TYPE_LOAD_BALANCER.to_string(),
            allocate_load_balancer_node_ports: None,
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert_eq!(spec.allocate_load_balancer_node_ports, Some(true));
    }

    #[test]
    fn test_default_service_client_ip_affinity() {
        let mut spec = ServiceSpec {
            session_affinity: SERVICE_AFFINITY_CLIENT_IP.to_string(),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        let config = spec
            .session_affinity_config
            .as_ref()
            .expect("session affinity config should be set");
        let client_ip = config.client_i_p.as_ref().expect("client ip should be set");
        assert_eq!(
            client_ip.timeout_seconds,
            Some(DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS)
        );
    }

    #[test]
    fn test_default_service_client_ip_affinity_config_nil() {
        let mut spec = ServiceSpec {
            session_affinity: SERVICE_AFFINITY_CLIENT_IP.to_string(),
            session_affinity_config: Some(SessionAffinityConfig { client_i_p: None }),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        let config = spec
            .session_affinity_config
            .as_ref()
            .expect("session affinity config should be set");
        let client_ip = config.client_i_p.as_ref().expect("client ip should be set");
        assert_eq!(
            client_ip.timeout_seconds,
            Some(DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS)
        );
    }

    #[test]
    fn test_default_service_client_ip_affinity_timeout_nil() {
        let mut spec = ServiceSpec {
            session_affinity: SERVICE_AFFINITY_CLIENT_IP.to_string(),
            session_affinity_config: Some(SessionAffinityConfig {
                client_i_p: Some(ClientIPConfig { timeout_seconds: None }),
            }),
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        let config = spec
            .session_affinity_config
            .as_ref()
            .expect("session affinity config should be set");
        let client_ip = config.client_i_p.as_ref().expect("client ip should be set");
        assert_eq!(
            client_ip.timeout_seconds,
            Some(DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS)
        );
    }

    #[test]
    fn test_default_service_external_traffic_policy() {
        let mut spec = ServiceSpec {
            external_i_ps: vec!["1.2.3.4".to_string()],
            ..Default::default()
        };
        apply_defaults_service_spec(&mut spec);
        assert_eq!(
            spec.external_traffic_policy,
            SERVICE_EXTERNAL_TRAFFIC_POLICY_CLUSTER
        );
    }

    #[test]
    fn test_default_service_load_balancer_ip_mode() {
        let mut service = Service {
            spec: Some(ServiceSpec {
                service_type: SERVICE_TYPE_LOAD_BALANCER.to_string(),
                ..Default::default()
            }),
            status: Some(ServiceStatus {
                load_balancer: Some(LoadBalancerStatus {
                    ingress: vec![LoadBalancerIngress {
                        ip: "1.2.3.4".to_string(),
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_service(&mut service);
        let ingress = service
            .status
            .as_ref()
            .and_then(|status| status.load_balancer.as_ref())
            .and_then(|lb| lb.ingress.first())
            .expect("ingress should exist");
        assert_eq!(
            ingress.ip_mode.as_deref(),
            Some(LOAD_BALANCER_IP_MODE_VIP)
        );

        let mut service = Service {
            spec: Some(ServiceSpec::default()),
            status: Some(ServiceStatus {
                load_balancer: Some(LoadBalancerStatus {
                    ingress: vec![LoadBalancerIngress {
                        ip: "1.2.3.4".to_string(),
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_service(&mut service);
        let ingress = service
            .status
            .as_ref()
            .and_then(|status| status.load_balancer.as_ref())
            .and_then(|lb| lb.ingress.first())
            .expect("ingress should exist");
        assert!(ingress.ip_mode.is_none());
    }

    #[test]
    fn test_default_service_load_balancer_ip_mode_preserve() {
        let mut service = Service {
            spec: Some(ServiceSpec {
                service_type: SERVICE_TYPE_LOAD_BALANCER.to_string(),
                ..Default::default()
            }),
            status: Some(ServiceStatus {
                load_balancer: Some(LoadBalancerStatus {
                    ingress: vec![LoadBalancerIngress {
                        ip: "1.2.3.4".to_string(),
                        ip_mode: Some(LOAD_BALANCER_IP_MODE_PROXY.to_string()),
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_service(&mut service);
        let ingress = service
            .status
            .as_ref()
            .and_then(|status| status.load_balancer.as_ref())
            .and_then(|lb| lb.ingress.first())
            .expect("ingress should exist");
        assert_eq!(
            ingress.ip_mode.as_deref(),
            Some(LOAD_BALANCER_IP_MODE_PROXY)
        );
    }

    #[test]
    fn test_default_limit_range_item() {
        let mut item = LimitRangeItem {
            limit_type: LIMIT_TYPE_CONTAINER.to_string(),
            max: [("cpu".to_string(), Quantity::new("1"))].into_iter().collect(),
            min: [("memory".to_string(), Quantity::new("1Gi"))]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        apply_defaults_limit_range_item(&mut item);
        assert_eq!(item.default.get("cpu").map(Quantity::as_str), Some("1"));
        assert_eq!(
            item.default_request.get("cpu").map(Quantity::as_str),
            Some("1")
        );
        assert_eq!(
            item.default_request.get("memory").map(Quantity::as_str),
            Some("1Gi")
        );
    }

    #[test]
    fn test_default_namespace_label_and_status() {
        let mut namespace = Namespace::new("ns");
        apply_defaults_namespace(&mut namespace);
        assert_eq!(
            namespace.metadata.labels.get(NAMESPACE_METADATA_LABEL).map(String::as_str),
            Some("ns")
        );
        let mut status = NamespaceStatus::default();
        apply_defaults_namespace_status(&mut status);
        assert_eq!(status.phase, NAMESPACE_PHASE_ACTIVE);
    }

    #[test]
    fn test_default_namespace_label_overwrite() {
        let mut namespace = Namespace::new("ns");
        namespace
            .metadata
            .labels
            .insert(NAMESPACE_METADATA_LABEL.to_string(), "other".to_string());
        apply_defaults_namespace(&mut namespace);
        assert_eq!(
            namespace.metadata.labels.get(NAMESPACE_METADATA_LABEL).map(String::as_str),
            Some("ns")
        );
    }

    #[test]
    fn test_default_namespace_no_label_without_name() {
        let mut namespace = Namespace::default();
        apply_defaults_namespace(&mut namespace);
        assert!(namespace.metadata.labels.is_empty());
    }

    #[test]
    fn test_default_replication_controller_labels() {
        let mut controller = ReplicationController {
            spec: Some(ReplicationControllerSpec {
                template: Some(PodTemplateSpec {
                    metadata: ObjectMeta {
                        labels: [("app".to_string(), "demo".to_string())]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_replication_controller(&mut controller);
        let spec = controller.spec.expect("spec should be set");
        assert_eq!(
            spec.selector.get("app").map(String::as_str),
            Some("demo")
        );
        assert_eq!(
            controller.metadata.labels.get("app").map(String::as_str),
            Some("demo")
        );
    }

    #[test]
    fn test_default_replication_controller_replicas_and_template() {
        let mut controller = ReplicationController {
            spec: Some(ReplicationControllerSpec {
                template: Some(PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "c".to_string(),
                            image: "nginx:latest".to_string(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_replication_controller(&mut controller);
        let spec = controller.spec.expect("spec should be set");
        assert_eq!(spec.replicas, Some(1));
        let template = spec.template.expect("template should be set");
        let pod_spec = template.spec.expect("pod spec should be set");
        assert_eq!(pod_spec.containers[0].image_pull_policy, PULL_ALWAYS);
    }

    #[test]
    fn test_default_replication_controller_init_containers() {
        let mut controller = ReplicationController {
            spec: Some(ReplicationControllerSpec {
                template: Some(PodTemplateSpec {
                    spec: Some(PodSpec {
                        init_containers: vec![Container {
                            name: "init".to_string(),
                            image: "busybox:latest".to_string(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_replication_controller(&mut controller);
        let spec = controller.spec.as_ref().expect("spec should be set");
        let template = spec.template.as_ref().expect("template should be set");
        let pod_spec = template.spec.as_ref().expect("pod spec should be set");
        let init = pod_spec
            .init_containers
            .first()
            .expect("init container should be set");
        assert_eq!(init.image_pull_policy, PULL_ALWAYS);
        assert_eq!(
            init.termination_message_path,
            DEFAULT_TERMINATION_MESSAGE_PATH
        );
    }

    #[test]
    fn test_default_request_not_set_for_replication_controller() {
        let mut controller = ReplicationController {
            spec: Some(ReplicationControllerSpec {
                template: Some(PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "c".to_string(),
                            resources: Some(ResourceRequirements {
                                limits: [("cpu".to_string(), Quantity::new("100m"))]
                                    .into_iter()
                                    .collect(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_replication_controller(&mut controller);
        let spec = controller.spec.as_ref().expect("spec should be set");
        let template = spec.template.as_ref().expect("template should be set");
        let pod_spec = template.spec.as_ref().expect("pod spec should be set");
        let requests = &pod_spec
            .containers
            .first()
            .expect("container should be set")
            .resources
            .as_ref()
            .expect("resources should be set")
            .requests;
        assert!(requests.is_empty());
    }

    #[test]
    fn test_default_pod_resource_requests_from_limits() {
        let mut pod = Pod {
            spec: Some(PodSpec {
                containers: vec![Container {
                    name: "c".to_string(),
                    resources: Some(ResourceRequirements {
                        limits: [("cpu".to_string(), Quantity::new("1"))]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                init_containers: vec![Container {
                    name: "init".to_string(),
                    resources: Some(ResourceRequirements {
                        limits: [
                            ("cpu".to_string(), Quantity::new("500m")),
                            ("memory".to_string(), Quantity::new("1Gi")),
                        ]
                        .into_iter()
                        .collect(),
                        requests: [("cpu".to_string(), Quantity::new("250m"))]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.expect("spec should be set");
        let requests = &spec.containers[0]
            .resources
            .as_ref()
            .expect("container resources should be set")
            .requests;
        assert_eq!(requests.get("cpu").map(Quantity::as_str), Some("1"));
        let init_requests = &spec.init_containers[0]
            .resources
            .as_ref()
            .expect("init resources should be set")
            .requests;
        assert_eq!(init_requests.get("cpu").map(Quantity::as_str), Some("250m"));
        assert_eq!(
            init_requests.get("memory").map(Quantity::as_str),
            Some("1Gi")
        );
    }

    #[test]
    fn test_default_pod_requests_preserve_zero() {
        let mut pod = Pod {
            spec: Some(PodSpec {
                containers: vec![Container {
                    name: "c".to_string(),
                    resources: Some(ResourceRequirements {
                        requests: [("memory".to_string(), Quantity::new("0"))]
                            .into_iter()
                            .collect(),
                        limits: [
                            ("cpu".to_string(), Quantity::new("100m")),
                            ("memory".to_string(), Quantity::new("1Gi")),
                        ]
                        .into_iter()
                        .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                init_containers: vec![Container {
                    name: "init".to_string(),
                    resources: Some(ResourceRequirements {
                        requests: [("memory".to_string(), Quantity::new("0"))]
                            .into_iter()
                            .collect(),
                        limits: [
                            ("cpu".to_string(), Quantity::new("100m")),
                            ("memory".to_string(), Quantity::new("1Gi")),
                        ]
                        .into_iter()
                        .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.as_ref().expect("spec should be set");
        let container_requests = &spec
            .containers
            .first()
            .expect("container should be set")
            .resources
            .as_ref()
            .expect("resources should be set")
            .requests;
        assert_eq!(
            container_requests.get("cpu").map(Quantity::as_str),
            Some("100m")
        );
        assert_eq!(
            container_requests.get("memory").map(Quantity::as_str),
            Some("0")
        );
        let init_requests = &spec
            .init_containers
            .first()
            .expect("init container should be set")
            .resources
            .as_ref()
            .expect("resources should be set")
            .requests;
        assert_eq!(
            init_requests.get("cpu").map(Quantity::as_str),
            Some("100m")
        );
        assert_eq!(
            init_requests.get("memory").map(Quantity::as_str),
            Some("0")
        );
    }

    #[test]
    fn test_default_minimum_scale_pod() {
        let mut pod = Pod {
            spec: Some(PodSpec {
                containers: vec![Container {
                    name: "c".to_string(),
                    resources: Some(ResourceRequirements {
                        requests: [("memory".to_string(), Quantity::new("1n"))]
                            .into_iter()
                            .collect(),
                        limits: [("cpu".to_string(), Quantity::new("2n"))]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                init_containers: vec![Container {
                    name: "init".to_string(),
                    resources: Some(ResourceRequirements {
                        requests: [("memory".to_string(), Quantity::new("1n"))]
                            .into_iter()
                            .collect(),
                        limits: [("cpu".to_string(), Quantity::new("2n"))]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.as_ref().expect("spec should be set");
        let container = spec.containers.first().expect("container should be set");
        let init = spec
            .init_containers
            .first()
            .expect("init container should be set");
        let container_req = container
            .resources
            .as_ref()
            .expect("container resources should be set")
            .requests
            .get("memory")
            .map(Quantity::as_str);
        let init_req = init
            .resources
            .as_ref()
            .expect("init resources should be set")
            .requests
            .get("memory")
            .map(Quantity::as_str);
        assert_eq!(container_req, Some("1m"));
        assert_eq!(init_req, Some("1m"));
    }

    #[test]
    fn test_default_pod_status_resource_lists() {
        let mut pod = Pod {
            status: Some(PodStatus {
                container_statuses: vec![ContainerStatus {
                    name: "c".to_string(),
                    allocated_resources: [("cpu".to_string(), Quantity::new("100n"))]
                        .into_iter()
                        .collect(),
                    resources: Some(ResourceRequirements {
                        limits: [("cpu".to_string(), Quantity::new("1e-6"))]
                            .into_iter()
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let status = pod.status.expect("status should be set");
        let container = status
            .container_statuses
            .first()
            .expect("container status should exist");
        assert_eq!(
            container.allocated_resources.get("cpu").map(Quantity::as_str),
            Some("1m")
        );
        let limits = container
            .resources
            .as_ref()
            .expect("resources should be set")
            .limits
            .get("cpu")
            .map(Quantity::as_str);
        assert_eq!(limits, Some("1e-3"));
    }

    #[test]
    fn test_default_pod_status_result_resource_lists() {
        let mut result = PodStatusResult {
            status: Some(PodStatus {
                init_container_statuses: vec![ContainerStatus {
                    name: "init".to_string(),
                    allocated_resources: [("cpu".to_string(), Quantity::new("1500u"))]
                        .into_iter()
                        .collect(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod_status_result(&mut result);
        let status = result.status.expect("status should be set");
        let init = status
            .init_container_statuses
            .first()
            .expect("init status should exist");
        assert_eq!(
            init.allocated_resources.get("cpu").map(Quantity::as_str),
            Some("2m")
        );
    }

    #[test]
    fn test_default_pod_level_resources() {
        let mut pod = Pod {
            spec: Some(PodSpec {
                containers: vec![
                    Container {
                        name: "c1".to_string(),
                        resources: Some(ResourceRequirements {
                            limits: [
                                ("cpu".to_string(), Quantity::new("2m")),
                                ("memory".to_string(), Quantity::new("1Gi")),
                                (
                                    format!("{RESOURCE_HUGE_PAGES_PREFIX}1Gi"),
                                    Quantity::new("1Gi"),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Container {
                        name: "c2".to_string(),
                        resources: Some(ResourceRequirements {
                            limits: [
                                ("cpu".to_string(), Quantity::new("1m")),
                                ("memory".to_string(), Quantity::new("512Mi")),
                                (
                                    format!("{RESOURCE_HUGE_PAGES_PREFIX}2Mi"),
                                    Quantity::new("2Mi"),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                resources: Some(ResourceRequirements {
                    limits: [
                        ("cpu".to_string(), Quantity::new("5m")),
                        ("memory".to_string(), Quantity::new("2Gi")),
                        (
                            format!("{RESOURCE_HUGE_PAGES_PREFIX}2Mi"),
                            Quantity::new("10Mi"),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        apply_defaults_pod(&mut pod);

        let spec = pod.spec.expect("spec should be set");
        let resources = spec.resources.expect("resources should be set");

        assert_eq!(
            resources
                .requests
                .get("cpu")
                .map(Quantity::as_str),
            Some("3m")
        );
        assert_eq!(
            resources
                .requests
                .get("memory")
                .map(Quantity::as_str),
            Some("1536Mi")
        );
        assert_eq!(
            resources
                .requests
                .get(&format!("{RESOURCE_HUGE_PAGES_PREFIX}2Mi"))
                .map(Quantity::as_str),
            Some("10Mi")
        );
        assert_eq!(
            resources
                .requests
                .get(&format!("{RESOURCE_HUGE_PAGES_PREFIX}1Gi"))
                .map(Quantity::as_str),
            Some("1Gi")
        );

        assert_eq!(
            resources
                .limits
                .get(&format!("{RESOURCE_HUGE_PAGES_PREFIX}1Gi"))
                .map(Quantity::as_str),
            Some("1Gi")
        );
    }

    #[test]
    fn test_default_pod_level_resources_feature_gate_disabled() {
        let _features = FeatureOverride::new(
            feature_image_volume_enabled(),
            feature_pod_logs_query_split_streams_enabled(),
            false,
        );
        let mut pod = Pod {
            spec: Some(PodSpec {
                resources: Some(ResourceRequirements {
                    limits: [("cpu".to_string(), Quantity::new("1"))]
                        .into_iter()
                        .collect(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_pod(&mut pod);
        let spec = pod.spec.as_ref().expect("spec should be set");
        let requests = &spec
            .resources
            .as_ref()
            .expect("resources should be set")
            .requests;
        assert!(requests.is_empty());
    }

    #[test]
    fn test_default_iscsi_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                iscsi: Some(ISCSIVolumeSource {
                    target_portal: "portal".to_string(),
                    iqn: "iqn".to_string(),
                    lun: 1,
                    iscsi_interface: String::new(),
                    fs_type: String::new(),
                    read_only: false,
                    portals: Vec::new(),
                    chap_auth_discovery: false,
                    chap_auth_session: false,
                    secret_ref: None,
                    initiator_name: None,
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let iscsi = volume
            .volume_source
            .iscsi
            .as_ref()
            .expect("iscsi should be set");
        assert_eq!(iscsi.iscsi_interface, DEFAULT_ISCSI_INTERFACE);
    }

    #[test]
    fn test_default_rbd_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                rbd: Some(RBDVolumeSource {
                    monitors: vec![],
                    image: "image".to_string(),
                    fs_type: String::new(),
                    pool: String::new(),
                    user: String::new(),
                    keyring: String::new(),
                    secret_ref: None,
                    read_only: false,
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let rbd = volume
            .volume_source
            .rbd
            .as_ref()
            .expect("rbd should be set");
        assert_eq!(rbd.pool, DEFAULT_RBD_POOL);
        assert_eq!(rbd.user, DEFAULT_RBD_USER);
        assert_eq!(rbd.keyring, DEFAULT_RBD_KEYRING);
    }

    #[test]
    fn test_default_azure_disk_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                azure_disk: Some(AzureDiskVolumeSource {
                    disk_name: "disk".to_string(),
                    disk_uri: "uri".to_string(),
                    caching_mode: None,
                    fs_type: None,
                    read_only: false,
                    kind: None,
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let azure = volume
            .volume_source
            .azure_disk
            .as_ref()
            .expect("azure disk should be set");
        assert_eq!(
            azure.caching_mode.as_deref(),
            Some(AZURE_DATA_DISK_CACHING_READ_WRITE)
        );
        assert_eq!(azure.fs_type.as_deref(), Some(DEFAULT_AZURE_DISK_FSTYPE));
        assert_eq!(azure.kind.as_deref(), Some(AZURE_DATA_DISK_KIND_SHARED));
    }

    #[test]
    fn test_default_scale_io_volume_source() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                scale_io: Some(ScaleIOVolumeSource {
                    gateway: "gw".to_string(),
                    system: "sys".to_string(),
                    secret_ref: LocalObjectReference {
                        name: "secret".to_string(),
                    },
                    ssl_enabled: false,
                    protection_domain: String::new(),
                    storage_pool: String::new(),
                    storage_mode: String::new(),
                    volume_name: String::new(),
                    fs_type: String::new(),
                    read_only: false,
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let scale = volume
            .volume_source
            .scale_io
            .as_ref()
            .expect("scale io should be set");
        assert_eq!(scale.storage_mode, DEFAULT_SCALE_IO_STORAGE_MODE);
        assert_eq!(scale.fs_type, DEFAULT_SCALE_IO_FSTYPE);
    }

    #[test]
    fn test_default_ephemeral_volume_claim_template() {
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                ephemeral: Some(EphemeralVolumeSource {
                    volume_claim_template: Some(PersistentVolumeClaimTemplate {
                        spec: PersistentVolumeClaimSpec::default(),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let template = volume
            .volume_source
            .ephemeral
            .as_ref()
            .expect("ephemeral should be set")
            .volume_claim_template
            .as_ref()
            .expect("claim template should be set");
        assert_eq!(
            template.spec.volume_mode.as_deref(),
            Some(PV_MODE_FILESYSTEM)
        );
    }

    #[test]
    fn test_default_persistent_volume_source_defaults() {
        let mut spec = PersistentVolumeSpec {
            persistent_volume_source: PersistentVolumeSource {
                iscsi: Some(ISCSIPersistentVolumeSource {
                    target_portal: "portal".to_string(),
                    iqn: "iqn".to_string(),
                    lun: 1,
                    iscsi_interface: String::new(),
                    fs_type: String::new(),
                    read_only: false,
                    portals: Vec::new(),
                    chap_auth_discovery: false,
                    chap_auth_session: false,
                    secret_ref: None,
                    initiator_name: None,
                }),
                rbd: Some(RBDPersistentVolumeSource {
                    monitors: vec![],
                    image: "image".to_string(),
                    fs_type: String::new(),
                    pool: String::new(),
                    user: String::new(),
                    keyring: String::new(),
                    secret_ref: None,
                    read_only: false,
                }),
                azure_disk: Some(AzureDiskVolumeSource {
                    disk_name: "disk".to_string(),
                    disk_uri: "uri".to_string(),
                    caching_mode: None,
                    fs_type: None,
                    read_only: false,
                    kind: None,
                }),
                scale_io: Some(ScaleIOPersistentVolumeSource {
                    gateway: "gw".to_string(),
                    system: "sys".to_string(),
                    secret_ref: SecretReference {
                        name: "secret".to_string(),
                        ..Default::default()
                    },
                    ssl_enabled: false,
                    protection_domain: String::new(),
                    storage_pool: String::new(),
                    storage_mode: String::new(),
                    volume_name: String::new(),
                    fs_type: String::new(),
                    read_only: false,
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        apply_defaults_persistent_volume_spec(&mut spec);
        let iscsi = spec
            .persistent_volume_source
            .iscsi
            .as_ref()
            .expect("iscsi should be set");
        assert_eq!(iscsi.iscsi_interface, DEFAULT_ISCSI_INTERFACE);
        let rbd = spec
            .persistent_volume_source
            .rbd
            .as_ref()
            .expect("rbd should be set");
        assert_eq!(rbd.pool, DEFAULT_RBD_POOL);
        assert_eq!(rbd.user, DEFAULT_RBD_USER);
        assert_eq!(rbd.keyring, DEFAULT_RBD_KEYRING);
        let azure = spec
            .persistent_volume_source
            .azure_disk
            .as_ref()
            .expect("azure disk should be set");
        assert_eq!(
            azure.caching_mode.as_deref(),
            Some(AZURE_DATA_DISK_CACHING_READ_WRITE)
        );
        assert_eq!(azure.fs_type.as_deref(), Some(DEFAULT_AZURE_DISK_FSTYPE));
        assert_eq!(azure.kind.as_deref(), Some(AZURE_DATA_DISK_KIND_SHARED));
        let scale = spec
            .persistent_volume_source
            .scale_io
            .as_ref()
            .expect("scale io should be set");
        assert_eq!(scale.storage_mode, DEFAULT_SCALE_IO_STORAGE_MODE);
        assert_eq!(scale.fs_type, DEFAULT_SCALE_IO_FSTYPE);
    }

    #[test]
    fn test_default_pod_log_options_stream() {
        let _features = FeatureOverride::new(false, false, feature_pod_level_resources_enabled());
        let mut options = PodLogOptions::default();
        apply_defaults_pod_log_options(&mut options);
        assert!(options.stream.is_none());
    }

    #[test]
    fn test_default_pod_log_options_stream_feature_gate() {
        let _features = FeatureOverride::new(
            feature_image_volume_enabled(),
            true,
            feature_pod_level_resources_enabled(),
        );
        let mut options = PodLogOptions::default();
        apply_defaults_pod_log_options(&mut options);
        assert_eq!(options.stream.as_deref(), Some(LOG_STREAM_ALL));
    }

    #[test]
    fn test_default_image_volume_source_pull_policy() {
        let _features = FeatureOverride::new(false, false, feature_pod_level_resources_enabled());
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                image: Some(ImageVolumeSource {
                    reference: "nginx:latest".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let image = volume
            .volume_source
            .image
            .as_ref()
            .expect("image should be set");
        assert!(image.pull_policy.is_empty());
    }

    #[test]
    fn test_default_image_volume_source_pull_policy_feature_gate() {
        let _features = FeatureOverride::new(
            true,
            feature_pod_logs_query_split_streams_enabled(),
            feature_pod_level_resources_enabled(),
        );
        let mut volume = Volume {
            name: "vol".to_string(),
            volume_source: VolumeSource {
                image: Some(ImageVolumeSource {
                    reference: "nginx:latest".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        apply_defaults_volume(&mut volume);
        let image = volume
            .volume_source
            .image
            .as_ref()
            .expect("image should be set");
        assert_eq!(image.pull_policy, PULL_ALWAYS);
    }

    #[test]
    fn test_default_resource_list_rounding() {
        let mut list: ResourceList = [
            ("cpu".to_string(), Quantity::new("3.000023m")),
            ("memory".to_string(), Quantity::new("500.000050m")),
            ("storage".to_string(), Quantity::new("100n")),
        ]
        .into_iter()
        .collect();
        apply_defaults_resource_list(&mut list);
        assert_eq!(list.get("cpu").map(Quantity::as_str), Some("4m"));
        assert_eq!(list.get("memory").map(Quantity::as_str), Some("501m"));
        assert_eq!(list.get("storage").map(Quantity::as_str), Some("1m"));
    }

    #[test]
    fn test_default_resource_list_rounding_noop() {
        let mut list: ResourceList = [
            ("cpu".to_string(), Quantity::new("100m")),
            ("memory".to_string(), Quantity::new("30M")),
            ("storage".to_string(), Quantity::new("1G")),
        ]
        .into_iter()
        .collect();
        apply_defaults_resource_list(&mut list);
        assert_eq!(list.get("cpu").map(Quantity::as_str), Some("100m"));
        assert_eq!(list.get("memory").map(Quantity::as_str), Some("30M"));
        assert_eq!(list.get("storage").map(Quantity::as_str), Some("1G"));
    }

    #[test]
    fn test_default_resource_list_rounding_large_value() {
        let mut list: ResourceList = [(
            "cpu".to_string(),
            Quantity::new("999999999999999999999m"),
        )]
        .into_iter()
        .collect();
        apply_defaults_resource_list(&mut list);
        assert_eq!(
            list.get("cpu").map(Quantity::as_str),
            Some("999999999999999999999m")
        );
    }

    #[test]
    fn test_default_resource_quota_rounding() {
        let mut quota = ResourceQuota {
            spec: Some(ResourceQuotaSpec {
                hard: [("cpu".to_string(), Quantity::new("3.000023m"))]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }),
            status: Some(ResourceQuotaStatus {
                used: [("memory".to_string(), Quantity::new("500.000050m"))]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_defaults_resource_quota(&mut quota);
        let spec = quota.spec.expect("spec should be set");
        assert_eq!(spec.hard.get("cpu").map(Quantity::as_str), Some("4m"));
        let status = quota.status.expect("status should be set");
        assert_eq!(status.used.get("memory").map(Quantity::as_str), Some("501m"));
    }
}
