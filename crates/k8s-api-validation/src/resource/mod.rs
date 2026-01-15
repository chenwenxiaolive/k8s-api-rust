//! Resource API validation
//!
//! This module provides validation for resource API types.

use crate::common::{
    validate_dns_label, validate_dns_subdomain_name, validate_label_key, validate_object_meta,
    validate_quantity,
};
use crate::{ValidationError, ValidationResult};

const VALID_ALLOCATION_MODES: &[&str] = &["ExactCount", "All"];

fn validate_required_dns_label(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, "name is required")]
    } else {
        validate_dns_label(value, field)
    }
}

fn validate_required_dns_subdomain(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, "name is required")]
    } else {
        validate_dns_subdomain_name(value, field)
    }
}

fn validate_pool_name(name: &str, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError::required(field, "pool name is required"));
        return errors;
    }

    for (i, segment) in name.split('/').enumerate() {
        if segment.is_empty() {
            errors.push(ValidationError::invalid(
                field,
                "pool name must not contain empty segments",
            ));
            continue;
        }
        errors.extend(validate_dns_subdomain_name(
            segment,
            &format!("{}[{}]", field, i),
        ));
    }

    errors
}

fn validate_allocation_mode(
    allocation_mode: &str,
    count: Option<i64>,
    mode_field: &str,
    count_field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if allocation_mode.is_empty() {
        if let Some(value) = count {
            if value <= 0 {
                errors.push(ValidationError::invalid(
                    count_field,
                    "count must be greater than zero",
                ));
            }
        }
        return errors;
    }

    match allocation_mode {
        "ExactCount" => {
            if let Some(value) = count {
                if value <= 0 {
                    errors.push(ValidationError::invalid(
                        count_field,
                        "count must be greater than zero",
                    ));
                }
            } else {
                errors.push(ValidationError::required(
                    count_field,
                    "count is required when allocationMode is ExactCount",
                ));
            }
        }
        "All" => {
            if count.is_some() {
                errors.push(ValidationError::invalid(
                    count_field,
                    "count must not be set when allocationMode is All",
                ));
            }
        }
        _ => {
            errors.push(ValidationError::not_supported(
                mode_field,
                allocation_mode,
                VALID_ALLOCATION_MODES,
            ));
        }
    }

    errors
}

fn validate_node_selection(
    node_name: Option<&str>,
    node_selector: &Option<serde_json::Value>,
    all_nodes: &Option<bool>,
    field: &str,
    require_one: bool,
) -> ValidationResult {
    let mut errors = Vec::new();

    let has_node_name = node_name.map_or(false, |value| !value.is_empty());
    let has_node_selector = node_selector.is_some();
    let has_all_nodes = all_nodes.is_some();
    let set_count = has_node_name as u8 + has_node_selector as u8 + has_all_nodes as u8;

    if require_one && set_count == 0 {
        errors.push(ValidationError::required(
            &format!("{}.nodeName", field),
            "exactly one of nodeName, nodeSelector, or allNodes is required",
        ));
    }

    if set_count > 1 {
        errors.push(ValidationError::invalid(
            &format!("{}.nodeName", field),
            "nodeName, nodeSelector, and allNodes are mutually exclusive",
        ));
    }

    if let Some(value) = node_name {
        if !value.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                value,
                &format!("{}.nodeName", field),
            ));
        }
    }

    errors
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::resource::v1beta1 as api;
    use std::collections::{BTreeMap, BTreeSet};

    pub fn validate_resource_claim(claim: &api::ResourceClaim) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&claim.metadata, "metadata", true));
        errors.extend(validate_resource_claim_spec(&claim.spec, "spec"));

        errors
    }

    pub fn validate_device_class(class: &api::DeviceClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&class.metadata, "metadata", true));
        errors.extend(validate_device_class_spec(&class.spec, "spec"));

        errors
    }

    pub fn validate_resource_claim_template(
        template: &api::ResourceClaimTemplate,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&template.metadata, "metadata", true));
        errors.extend(validate_resource_claim_template_spec(&template.spec, "spec"));

        errors
    }

    pub fn validate_resource_slice(slice: &api::ResourceSlice) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&slice.metadata, "metadata", true));
        errors.extend(validate_resource_slice_spec(&slice.spec, "spec"));

        errors
    }

    fn validate_resource_claim_spec(spec: &api::ResourceClaimSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(devices) = &spec.devices {
            errors.extend(validate_device_claim(devices, &format!("{}.devices", field)));
        }

        errors
    }

    fn gather_request_names(requests: &[api::DeviceRequest]) -> BTreeMap<String, BTreeSet<String>> {
        let mut names = BTreeMap::new();

        for request in requests {
            let mut sub_names = BTreeSet::new();
            for subrequest in request.first_available.iter() {
                sub_names.insert(subrequest.name.clone());
            }
            names.insert(request.name.clone(), sub_names);
        }

        names
    }

    fn request_name_exists(
        name: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> bool {
        let mut parts = name.split('/');
        let request_name = match parts.next() {
            Some(value) => value,
            None => return false,
        };
        let subrequest_name = parts.next();
        if parts.next().is_some() {
            return false;
        }

        let Some(subrequests) = request_names.get(request_name) else {
            return false;
        };

        match subrequest_name {
            Some(sub) => subrequests.contains(sub),
            None => true,
        }
    }

    fn validate_device_claim(claim: &api::DeviceClaim, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen_requests = BTreeSet::new();

        for (i, request) in claim.requests.iter().enumerate() {
            let request_field = format!("{}.requests[{}]", field, i);
            errors.extend(validate_device_request(request, &request_field));

            if !request.name.is_empty() && !seen_requests.insert(request.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", request_field),
                    request.name.clone(),
                ));
            }
        }

        let request_names = gather_request_names(&claim.requests);

        for (i, constraint) in claim.constraints.iter().enumerate() {
            let constraint_field = format!("{}.constraints[{}]", field, i);
            errors.extend(validate_device_constraint(
                constraint,
                &constraint_field,
                &request_names,
            ));
        }

        for (i, config) in claim.config.iter().enumerate() {
            let config_field = format!("{}.config[{}]", field, i);
            errors.extend(validate_device_claim_configuration(
                config,
                &config_field,
                &request_names,
            ));
        }

        errors
    }

    fn validate_device_request(request: &api::DeviceRequest, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &request.name,
            &format!("{}.name", field),
        ));

        let uses_first_available = !request.first_available.is_empty();

        if uses_first_available {
            if !request.device_class_name.is_empty() {
                errors.push(ValidationError::invalid(
                    &format!("{}.deviceClassName", field),
                    "deviceClassName must be empty when firstAvailable is set",
                ));
            }

            if !request.selectors.is_empty()
                || !request.allocation_mode.is_empty()
                || request.count.is_some()
                || request.admin_access.is_some()
                || !request.tolerations.is_empty()
                || request.capacity.is_some()
            {
                errors.push(ValidationError::invalid(
                    field,
                    "request fields must be set on subrequests when firstAvailable is used",
                ));
            }
        } else {
            errors.extend(validate_required_dns_subdomain(
                &request.device_class_name,
                &format!("{}.deviceClassName", field),
            ));

            for (i, selector) in request.selectors.iter().enumerate() {
                errors.extend(validate_device_selector(
                    selector,
                    &format!("{}.selectors[{}]", field, i),
                ));
            }

            errors.extend(validate_allocation_mode(
                &request.allocation_mode,
                request.count,
                &format!("{}.allocationMode", field),
                &format!("{}.count", field),
            ));
        }

        if uses_first_available {
            let mut seen = BTreeSet::new();
            for (i, subrequest) in request.first_available.iter().enumerate() {
                let sub_field = format!("{}.firstAvailable[{}]", field, i);
                errors.extend(validate_device_subrequest(subrequest, &sub_field));

                if !subrequest.name.is_empty() && !seen.insert(subrequest.name.clone()) {
                    errors.push(ValidationError::duplicate(
                        format!("{}.name", sub_field),
                        subrequest.name.clone(),
                    ));
                }
            }
        }

        errors
    }

    fn validate_device_subrequest(
        subrequest: &api::DeviceSubRequest,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &subrequest.name,
            &format!("{}.name", field),
        ));
        errors.extend(validate_required_dns_subdomain(
            &subrequest.device_class_name,
            &format!("{}.deviceClassName", field),
        ));

        for (i, selector) in subrequest.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        errors.extend(validate_allocation_mode(
            &subrequest.allocation_mode,
            subrequest.count,
            &format!("{}.allocationMode", field),
            &format!("{}.count", field),
        ));

        errors
    }

    fn validate_device_selector(selector: &api::DeviceSelector, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(cel) = &selector.cel {
            if cel.expression.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.cel.expression", field),
                    "expression is required",
                ));
            }
        } else {
            errors.push(ValidationError::required(
                &format!("{}.cel", field),
                "cel selector is required",
            ));
        }

        errors
    }

    fn validate_device_constraint(
        constraint: &api::DeviceConstraint,
        field: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        if constraint.requests.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.requests", field),
                "requests is required",
            ));
        }

        for (i, name) in constraint.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);

            let mut parts = name.split('/');
            let first = parts.next().unwrap_or("");
            let second = parts.next();
            if parts.next().is_some() {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must be request or request/subrequest",
                ));
            } else {
                errors.extend(validate_required_dns_label(first, &name_field));
                if let Some(value) = second {
                    errors.extend(validate_required_dns_label(value, &name_field));
                }
            }

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.clone(), name.clone()));
            }

            if !request_name_exists(name, request_names) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        let has_match = constraint.match_attribute.is_some();
        let has_distinct = constraint.distinct_attribute.is_some();

        if has_match && has_distinct {
            errors.push(ValidationError::invalid(
                field,
                "matchAttribute and distinctAttribute are mutually exclusive",
            ));
        } else if let Some(match_attribute) = &constraint.match_attribute {
            errors.extend(validate_label_key(
                match_attribute,
                &format!("{}.matchAttribute", field),
            ));
        } else if let Some(distinct_attribute) = &constraint.distinct_attribute {
            errors.extend(validate_label_key(
                distinct_attribute,
                &format!("{}.distinctAttribute", field),
            ));
        } else {
            errors.push(ValidationError::required(
                field,
                "matchAttribute or distinctAttribute is required",
            ));
        }

        errors
    }

    fn validate_device_claim_configuration(
        config: &api::DeviceClaimConfiguration,
        field: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        for (i, name) in config.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);

            let mut parts = name.split('/');
            let first = parts.next().unwrap_or("");
            let second = parts.next();
            if parts.next().is_some() {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must be request or request/subrequest",
                ));
            } else {
                errors.extend(validate_required_dns_label(first, &name_field));
                if let Some(value) = second {
                    errors.extend(validate_required_dns_label(value, &name_field));
                }
            }

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.clone(), name.clone()));
            }

            if !request_name_exists(name, request_names) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        if let Some(opaque) = &config.device_configuration.opaque {
            errors.extend(validate_opaque_device_configuration(
                opaque,
                &format!("{}.opaque", field),
            ));
        } else {
            errors.push(ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            ));
        }

        errors
    }

    fn validate_opaque_device_configuration(
        config: &api::OpaqueDeviceConfiguration,
        field: &str,
    ) -> ValidationResult {
        validate_required_dns_subdomain(&config.driver, &format!("{}.driver", field))
    }

    fn validate_device_class_spec(spec: &api::DeviceClassSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (i, selector) in spec.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        for (i, config) in spec.config.iter().enumerate() {
            errors.extend(validate_device_class_configuration(
                config,
                &format!("{}.config[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_device_class_configuration(
        config: &api::DeviceClassConfiguration,
        field: &str,
    ) -> ValidationResult {
        if let Some(opaque) = &config.device_configuration.opaque {
            validate_opaque_device_configuration(opaque, &format!("{}.opaque", field))
        } else {
            vec![ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            )]
        }
    }

    fn validate_resource_claim_template_spec(
        spec: &api::ResourceClaimTemplateSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(
            &spec.metadata,
            &format!("{}.metadata", field),
            false,
        ));
        errors.extend(validate_resource_claim_spec(&spec.spec, &format!("{}.spec", field)));

        errors
    }

    fn validate_resource_slice_spec(spec: &api::ResourceSliceSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_subdomain(
            &spec.driver,
            &format!("{}.driver", field),
        ));
        errors.extend(validate_resource_pool(&spec.pool, &format!("{}.pool", field)));

        errors.extend(validate_node_selection(
            Some(spec.node_name.as_str()),
            &spec.node_selector,
            &spec.all_nodes,
            field,
            true,
        ));

        let mut device_names = BTreeSet::new();
        for (i, device) in spec.devices.iter().enumerate() {
            let device_field = format!("{}.devices[{}]", field, i);
            errors.extend(validate_device(device, &device_field));

            if !device.name.is_empty() && !device_names.insert(device.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", device_field),
                    device.name.clone(),
                ));
            }
        }

        let mut counter_set_names = BTreeSet::new();
        for (i, counter_set) in spec.shared_counters.iter().enumerate() {
            let counter_field = format!("{}.sharedCounters[{}]", field, i);
            errors.extend(validate_counter_set(counter_set, &counter_field));

            if !counter_set.name.is_empty()
                && !counter_set_names.insert(counter_set.name.clone())
            {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", counter_field),
                    counter_set.name.clone(),
                ));
            }
        }

        errors
    }

    fn validate_resource_pool(pool: &api::ResourcePool, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_pool_name(&pool.name, &format!("{}.name", field)));

        if pool.generation < 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.generation", field),
                "generation must be non-negative",
            ));
        }

        if pool.resource_slice_count <= 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.resourceSliceCount", field),
                "resourceSliceCount must be greater than zero",
            ));
        }

        errors
    }

    fn validate_device(device: &api::Device, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &device.name,
            &format!("{}.name", field),
        ));

        if let Some(basic) = &device.basic {
            errors.extend(validate_basic_device(basic, &format!("{}.basic", field)));
        }

        errors
    }

    fn validate_basic_device(basic: &api::BasicDevice, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (key, value) in &basic.attributes {
            let key_field = format!("{}.attributes[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_attribute(value, &key_field));
        }

        for (key, value) in &basic.capacity {
            let key_field = format!("{}.capacity[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_capacity(value, &key_field));
        }

        let mut counter_sets = BTreeSet::new();
        for (i, counter) in basic.consumes_counters.iter().enumerate() {
            let counter_field = format!("{}.consumesCounters[{}]", field, i);
            errors.extend(validate_device_counter_consumption(counter, &counter_field));

            if !counter.counter_set.is_empty() && !counter_sets.insert(counter.counter_set.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.counterSet", counter_field),
                    counter.counter_set.clone(),
                ));
            }
        }

        errors.extend(validate_node_selection(
            basic.node_name.as_deref(),
            &basic.node_selector,
            &basic.all_nodes,
            field,
            false,
        ));

        errors
    }

    fn validate_device_counter_consumption(
        counter: &api::DeviceCounterConsumption,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &counter.counter_set,
            &format!("{}.counterSet", field),
        ));

        for (key, value) in &counter.counters {
            let key_field = format!("{}.counters[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_quantity(&value.value, &format!("{}.value", key_field)));
        }

        errors
    }

    fn validate_counter_set(counter_set: &api::CounterSet, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &counter_set.name,
            &format!("{}.name", field),
        ));

        for (key, value) in &counter_set.counters {
            let key_field = format!("{}.counters[{}]", field, key);
            errors.extend(validate_required_dns_label(key, &key_field));
            errors.extend(validate_quantity(&value.value, &format!("{}.value", key_field)));
        }

        errors
    }

    fn validate_device_attribute(attr: &api::DeviceAttribute, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut set_count = 0;

        if attr.int_value.is_some() {
            set_count += 1;
        }
        if attr.bool_value.is_some() {
            set_count += 1;
        }
        if attr.string_value.is_some() {
            set_count += 1;
        }
        if attr.version_value.is_some() {
            set_count += 1;
        }

        if set_count == 0 {
            errors.push(ValidationError::required(
                field,
                "one of intValue, boolValue, stringValue, or versionValue is required",
            ));
        } else if set_count > 1 {
            errors.push(ValidationError::invalid(
                field,
                "only one of intValue, boolValue, stringValue, or versionValue is allowed",
            ));
        }

        if let Some(value) = &attr.string_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.stringValue", field),
                    "stringValue is required",
                ));
            }
        }

        if let Some(value) = &attr.version_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.versionValue", field),
                    "versionValue is required",
                ));
            }
        }

        errors
    }

    fn validate_device_capacity(capacity: &api::DeviceCapacity, field: &str) -> ValidationResult {
        validate_quantity(&capacity.value, &format!("{}.value", field))
    }
}

pub mod v1beta2 {
    use super::*;
    use k8s_api::resource::v1beta2 as api;
    use std::collections::{BTreeMap, BTreeSet};

    pub fn validate_resource_claim(claim: &api::ResourceClaim) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&claim.metadata, "metadata", true));
        errors.extend(validate_resource_claim_spec(&claim.spec, "spec"));

        errors
    }

    pub fn validate_device_class(class: &api::DeviceClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&class.metadata, "metadata", true));
        errors.extend(validate_device_class_spec(&class.spec, "spec"));

        errors
    }

    pub fn validate_resource_claim_template(
        template: &api::ResourceClaimTemplate,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&template.metadata, "metadata", true));
        errors.extend(validate_resource_claim_template_spec(&template.spec, "spec"));

        errors
    }

    pub fn validate_resource_slice(slice: &api::ResourceSlice) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&slice.metadata, "metadata", true));
        errors.extend(validate_resource_slice_spec(&slice.spec, "spec"));

        errors
    }

    fn validate_resource_claim_spec(spec: &api::ResourceClaimSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(devices) = &spec.devices {
            errors.extend(validate_device_claim(devices, &format!("{}.devices", field)));
        }

        errors
    }

    fn gather_request_names(requests: &[api::DeviceRequest]) -> BTreeMap<String, BTreeSet<String>> {
        let mut names = BTreeMap::new();

        for request in requests {
            let mut sub_names = BTreeSet::new();
            for subrequest in request.first_available.iter() {
                sub_names.insert(subrequest.name.clone());
            }
            names.insert(request.name.clone(), sub_names);
        }

        names
    }

    fn request_name_exists(
        name: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> bool {
        let mut parts = name.split('/');
        let request_name = match parts.next() {
            Some(value) => value,
            None => return false,
        };
        let subrequest_name = parts.next();
        if parts.next().is_some() {
            return false;
        }

        let Some(subrequests) = request_names.get(request_name) else {
            return false;
        };

        match subrequest_name {
            Some(sub) => subrequests.contains(sub),
            None => true,
        }
    }

    fn validate_device_claim(claim: &api::DeviceClaim, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut request_names = BTreeMap::new();
        let mut seen_requests = BTreeSet::new();

        for (i, request) in claim.requests.iter().enumerate() {
            let request_field = format!("{}.requests[{}]", field, i);
            errors.extend(validate_device_request(request, &request_field));

            if !request.name.is_empty() && !seen_requests.insert(request.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", request_field),
                    request.name.clone(),
                ));
            }
        }

        request_names.extend(gather_request_names(&claim.requests));

        for (i, constraint) in claim.constraints.iter().enumerate() {
            let constraint_field = format!("{}.constraints[{}]", field, i);
            errors.extend(validate_device_constraint(
                constraint,
                &constraint_field,
                &request_names,
            ));
        }

        for (i, config) in claim.config.iter().enumerate() {
            let config_field = format!("{}.config[{}]", field, i);
            errors.extend(validate_device_claim_configuration(
                config,
                &config_field,
                &request_names,
            ));
        }

        errors
    }

    fn validate_device_request(request: &api::DeviceRequest, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &request.name,
            &format!("{}.name", field),
        ));

        let has_exact = request.exactly.is_some();
        let has_first_available = !request.first_available.is_empty();

        if has_exact && has_first_available {
            errors.push(ValidationError::invalid(
                field,
                "exactly and firstAvailable are mutually exclusive",
            ));
        } else if !has_exact && !has_first_available {
            errors.push(ValidationError::required(
                field,
                "either exactly or firstAvailable is required",
            ));
        }

        if let Some(exact) = &request.exactly {
            errors.extend(validate_exact_device_request(
                exact,
                &format!("{}.exactly", field),
            ));
        }

        if has_first_available {
            let mut seen = BTreeSet::new();
            for (i, subrequest) in request.first_available.iter().enumerate() {
                let sub_field = format!("{}.firstAvailable[{}]", field, i);
                errors.extend(validate_device_subrequest(subrequest, &sub_field));

                if !subrequest.name.is_empty() && !seen.insert(subrequest.name.clone()) {
                    errors.push(ValidationError::duplicate(
                        format!("{}.name", sub_field),
                        subrequest.name.clone(),
                    ));
                }
            }
        }

        errors
    }

    fn validate_exact_device_request(
        request: &api::ExactDeviceRequest,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_subdomain(
            &request.device_class_name,
            &format!("{}.deviceClassName", field),
        ));

        for (i, selector) in request.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        errors.extend(validate_allocation_mode(
            &request.allocation_mode,
            request.count,
            &format!("{}.allocationMode", field),
            &format!("{}.count", field),
        ));

        if let Some(capacity) = &request.capacity {
            errors.extend(validate_capacity_requirements(
                capacity,
                &format!("{}.capacity", field),
            ));
        }

        errors
    }

    fn validate_device_subrequest(
        subrequest: &api::DeviceSubRequest,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &subrequest.name,
            &format!("{}.name", field),
        ));
        errors.extend(validate_required_dns_subdomain(
            &subrequest.device_class_name,
            &format!("{}.deviceClassName", field),
        ));

        for (i, selector) in subrequest.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        errors.extend(validate_allocation_mode(
            &subrequest.allocation_mode,
            subrequest.count,
            &format!("{}.allocationMode", field),
            &format!("{}.count", field),
        ));

        if let Some(capacity) = &subrequest.capacity {
            errors.extend(validate_capacity_requirements(
                capacity,
                &format!("{}.capacity", field),
            ));
        }

        errors
    }

    fn validate_capacity_requirements(
        capacity: &api::CapacityRequirements,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        for (key, value) in &capacity.requests {
            let key_field = format!("{}.requests[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_quantity(value, &format!("{}.value", key_field)));
        }

        errors
    }

    fn validate_device_selector(selector: &api::DeviceSelector, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(cel) = &selector.cel {
            if cel.expression.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.cel.expression", field),
                    "expression is required",
                ));
            }
        } else {
            errors.push(ValidationError::required(
                &format!("{}.cel", field),
                "cel selector is required",
            ));
        }

        errors
    }

    fn validate_device_constraint(
        constraint: &api::DeviceConstraint,
        field: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        if constraint.requests.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.requests", field),
                "requests is required",
            ));
        }

        for (i, name) in constraint.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);

            let mut parts = name.split('/');
            let first = parts.next().unwrap_or("");
            let second = parts.next();
            if parts.next().is_some() {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must be request or request/subrequest",
                ));
            } else {
                errors.extend(validate_required_dns_label(first, &name_field));
                if let Some(value) = second {
                    errors.extend(validate_required_dns_label(value, &name_field));
                }
            }

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.clone(), name.clone()));
            }

            if !request_name_exists(name, request_names) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        let has_match = constraint.match_attribute.is_some();
        let has_distinct = constraint.distinct_attribute.is_some();

        if has_match && has_distinct {
            errors.push(ValidationError::invalid(
                field,
                "matchAttribute and distinctAttribute are mutually exclusive",
            ));
        } else if let Some(match_attribute) = &constraint.match_attribute {
            errors.extend(validate_label_key(
                match_attribute,
                &format!("{}.matchAttribute", field),
            ));
        } else if let Some(distinct_attribute) = &constraint.distinct_attribute {
            errors.extend(validate_label_key(
                distinct_attribute,
                &format!("{}.distinctAttribute", field),
            ));
        } else {
            errors.push(ValidationError::required(
                field,
                "matchAttribute or distinctAttribute is required",
            ));
        }

        errors
    }

    fn validate_device_claim_configuration(
        config: &api::DeviceClaimConfiguration,
        field: &str,
        request_names: &BTreeMap<String, BTreeSet<String>>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        for (i, name) in config.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);

            let mut parts = name.split('/');
            let first = parts.next().unwrap_or("");
            let second = parts.next();
            if parts.next().is_some() {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must be request or request/subrequest",
                ));
            } else {
                errors.extend(validate_required_dns_label(first, &name_field));
                if let Some(value) = second {
                    errors.extend(validate_required_dns_label(value, &name_field));
                }
            }

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.clone(), name.clone()));
            }

            if !request_name_exists(name, request_names) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        if let Some(opaque) = &config.device_configuration.opaque {
            errors.extend(validate_opaque_device_configuration(
                opaque,
                &format!("{}.opaque", field),
            ));
        } else {
            errors.push(ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            ));
        }

        errors
    }

    fn validate_opaque_device_configuration(
        config: &api::OpaqueDeviceConfiguration,
        field: &str,
    ) -> ValidationResult {
        validate_required_dns_subdomain(&config.driver, &format!("{}.driver", field))
    }

    fn validate_device_class_spec(spec: &api::DeviceClassSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (i, selector) in spec.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        for (i, config) in spec.config.iter().enumerate() {
            errors.extend(validate_device_class_configuration(
                config,
                &format!("{}.config[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_device_class_configuration(
        config: &api::DeviceClassConfiguration,
        field: &str,
    ) -> ValidationResult {
        if let Some(opaque) = &config.device_configuration.opaque {
            validate_opaque_device_configuration(opaque, &format!("{}.opaque", field))
        } else {
            vec![ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            )]
        }
    }

    fn validate_resource_claim_template_spec(
        spec: &api::ResourceClaimTemplateSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(
            &spec.metadata,
            &format!("{}.metadata", field),
            false,
        ));
        errors.extend(validate_resource_claim_spec(&spec.spec, &format!("{}.spec", field)));

        errors
    }

    fn validate_resource_slice_spec(spec: &api::ResourceSliceSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_subdomain(
            &spec.driver,
            &format!("{}.driver", field),
        ));
        errors.extend(validate_resource_pool(&spec.pool, &format!("{}.pool", field)));

        errors.extend(validate_node_selection(
            spec.node_name.as_deref(),
            &spec.node_selector,
            &spec.all_nodes,
            field,
            true,
        ));

        let mut device_names = BTreeSet::new();
        for (i, device) in spec.devices.iter().enumerate() {
            let device_field = format!("{}.devices[{}]", field, i);
            errors.extend(validate_device(device, &device_field));

            if !device.name.is_empty() && !device_names.insert(device.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", device_field),
                    device.name.clone(),
                ));
            }
        }

        let mut counter_set_names = BTreeSet::new();
        for (i, counter_set) in spec.shared_counters.iter().enumerate() {
            let counter_field = format!("{}.sharedCounters[{}]", field, i);
            errors.extend(validate_counter_set(counter_set, &counter_field));

            if !counter_set.name.is_empty()
                && !counter_set_names.insert(counter_set.name.clone())
            {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", counter_field),
                    counter_set.name.clone(),
                ));
            }
        }

        errors
    }

    fn validate_counter_set(counter_set: &api::CounterSet, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &counter_set.name,
            &format!("{}.name", field),
        ));

        for (key, value) in &counter_set.counters {
            let key_field = format!("{}.counters[{}]", field, key);
            errors.extend(validate_required_dns_label(key, &key_field));
            errors.extend(validate_quantity(&value.value, &format!("{}.value", key_field)));
        }

        errors
    }

    fn validate_resource_pool(pool: &api::ResourcePool, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_pool_name(&pool.name, &format!("{}.name", field)));

        if pool.generation < 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.generation", field),
                "generation must be non-negative",
            ));
        }

        if pool.resource_slice_count <= 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.resourceSliceCount", field),
                "resourceSliceCount must be greater than zero",
            ));
        }

        errors
    }

    fn validate_device(device: &api::Device, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &device.name,
            &format!("{}.name", field),
        ));

        for (key, value) in &device.attributes {
            let key_field = format!("{}.attributes[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_attribute(value, &key_field));
        }

        for (key, value) in &device.capacity {
            let key_field = format!("{}.capacity[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_capacity(value, &key_field));
        }

        let mut counter_sets = BTreeSet::new();
        for (i, counter) in device.consumes_counters.iter().enumerate() {
            let counter_field = format!("{}.consumesCounters[{}]", field, i);
            errors.extend(validate_device_counter_consumption(counter, &counter_field));

            if !counter.counter_set.is_empty() && !counter_sets.insert(counter.counter_set.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.counterSet", counter_field),
                    counter.counter_set.clone(),
                ));
            }
        }

        errors.extend(validate_node_selection(
            device.node_name.as_deref(),
            &device.node_selector,
            &device.all_nodes,
            field,
            false,
        ));

        errors
    }

    fn validate_device_counter_consumption(
        counter: &api::DeviceCounterConsumption,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &counter.counter_set,
            &format!("{}.counterSet", field),
        ));

        for (key, value) in &counter.counters {
            let key_field = format!("{}.counters[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_quantity(&value.value, &format!("{}.value", key_field)));
        }

        errors
    }

    fn validate_device_attribute(attr: &api::DeviceAttribute, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut set_count = 0;

        if attr.int_value.is_some() {
            set_count += 1;
        }
        if attr.bool_value.is_some() {
            set_count += 1;
        }
        if attr.string_value.is_some() {
            set_count += 1;
        }
        if attr.version_value.is_some() {
            set_count += 1;
        }

        if set_count == 0 {
            errors.push(ValidationError::required(
                field,
                "one of intValue, boolValue, stringValue, or versionValue is required",
            ));
        } else if set_count > 1 {
            errors.push(ValidationError::invalid(
                field,
                "only one of intValue, boolValue, stringValue, or versionValue is allowed",
            ));
        }

        if let Some(value) = &attr.string_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.stringValue", field),
                    "stringValue is required",
                ));
            }
        }

        if let Some(value) = &attr.version_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.versionValue", field),
                    "versionValue is required",
                ));
            }
        }

        errors
    }

    fn validate_device_capacity(capacity: &api::DeviceCapacity, field: &str) -> ValidationResult {
        validate_quantity(&capacity.value, &format!("{}.value", field))
    }
}

pub mod v1alpha3 {
    use super::*;
    use k8s_api::resource::v1alpha3 as api;
    use std::collections::BTreeSet;

    pub fn validate_resource_claim(claim: &api::ResourceClaim) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&claim.metadata, "metadata", true));
        errors.extend(validate_resource_claim_spec(&claim.spec, "spec"));

        errors
    }

    pub fn validate_device_class(class: &api::DeviceClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&class.metadata, "metadata", true));
        errors.extend(validate_device_class_spec(&class.spec, "spec"));

        errors
    }

    pub fn validate_resource_claim_template(
        template: &api::ResourceClaimTemplate,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&template.metadata, "metadata", true));
        errors.extend(validate_resource_claim_template_spec(&template.spec, "spec"));

        errors
    }

    pub fn validate_resource_slice(slice: &api::ResourceSlice) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&slice.metadata, "metadata", true));
        errors.extend(validate_resource_slice_spec(&slice.spec, "spec"));

        errors
    }

    fn validate_resource_claim_spec(spec: &api::ResourceClaimSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(devices) = &spec.devices {
            errors.extend(validate_device_claim(devices, &format!("{}.devices", field)));
        }

        if !spec.controller.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                &spec.controller,
                &format!("{}.controller", field),
            ));
        }

        errors
    }

    fn validate_device_claim(claim: &api::DeviceClaim, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut request_names = BTreeSet::new();

        for (i, request) in claim.requests.iter().enumerate() {
            let request_field = format!("{}.requests[{}]", field, i);
            errors.extend(validate_device_request(request, &request_field));

            if !request.name.is_empty() && !request_names.insert(request.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", request_field),
                    request.name.clone(),
                ));
            }
        }

        for (i, constraint) in claim.constraints.iter().enumerate() {
            let constraint_field = format!("{}.constraints[{}]", field, i);
            errors.extend(validate_device_constraint(
                constraint,
                &constraint_field,
                &request_names,
            ));
        }

        for (i, config) in claim.config.iter().enumerate() {
            let config_field = format!("{}.config[{}]", field, i);
            errors.extend(validate_device_claim_configuration(
                config,
                &config_field,
                &request_names,
            ));
        }

        errors
    }

    fn validate_device_request(request: &api::DeviceRequest, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &request.name,
            &format!("{}.name", field),
        ));
        errors.extend(validate_required_dns_subdomain(
            &request.device_class_name,
            &format!("{}.deviceClassName", field),
        ));

        for (i, selector) in request.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        errors.extend(validate_allocation_mode(
            &request.allocation_mode,
            request.count,
            &format!("{}.allocationMode", field),
            &format!("{}.count", field),
        ));

        errors
    }

    fn validate_device_selector(selector: &api::DeviceSelector, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(cel) = &selector.cel {
            if cel.expression.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.cel.expression", field),
                    "expression is required",
                ));
            }
        } else {
            errors.push(ValidationError::required(
                &format!("{}.cel", field),
                "cel selector is required",
            ));
        }

        errors
    }

    fn validate_device_constraint(
        constraint: &api::DeviceConstraint,
        field: &str,
        request_names: &BTreeSet<String>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        if constraint.requests.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.requests", field),
                "requests is required",
            ));
        }

        for (i, name) in constraint.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);
            errors.extend(validate_required_dns_label(name, &name_field));

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.as_str(), name.clone()));
            }

            if !request_names.contains(name) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        if let Some(match_attribute) = &constraint.match_attribute {
            errors.extend(validate_label_key(
                match_attribute,
                &format!("{}.matchAttribute", field),
            ));
        } else {
            errors.push(ValidationError::required(
                &format!("{}.matchAttribute", field),
                "matchAttribute is required",
            ));
        }

        errors
    }

    fn validate_device_claim_configuration(
        config: &api::DeviceClaimConfiguration,
        field: &str,
        request_names: &BTreeSet<String>,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        for (i, name) in config.requests.iter().enumerate() {
            let name_field = format!("{}.requests[{}]", field, i);
            errors.extend(validate_required_dns_label(name, &name_field));

            if !seen.insert(name.clone()) {
                errors.push(ValidationError::duplicate(name_field.as_str(), name.clone()));
            }

            if !request_names.contains(name) {
                errors.push(ValidationError::invalid(
                    &name_field,
                    "request must reference an existing request",
                ));
            }
        }

        if let Some(opaque) = &config.opaque {
            errors.extend(validate_opaque_device_configuration(
                opaque,
                &format!("{}.opaque", field),
            ));
        } else {
            errors.push(ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            ));
        }

        errors
    }

    fn validate_opaque_device_configuration(
        config: &api::OpaqueDeviceConfiguration,
        field: &str,
    ) -> ValidationResult {
        validate_required_dns_subdomain(&config.driver, &format!("{}.driver", field))
    }

    fn validate_device_class_spec(spec: &api::DeviceClassSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (i, selector) in spec.selectors.iter().enumerate() {
            errors.extend(validate_device_selector(
                selector,
                &format!("{}.selectors[{}]", field, i),
            ));
        }

        for (i, config) in spec.config.iter().enumerate() {
            errors.extend(validate_device_class_configuration(
                config,
                &format!("{}.config[{}]", field, i),
            ));
        }

        errors
    }

    fn validate_device_class_configuration(
        config: &api::DeviceClassConfiguration,
        field: &str,
    ) -> ValidationResult {
        if let Some(opaque) = &config.opaque {
            validate_opaque_device_configuration(opaque, &format!("{}.opaque", field))
        } else {
            vec![ValidationError::required(
                &format!("{}.opaque", field),
                "opaque configuration is required",
            )]
        }
    }

    fn validate_resource_claim_template_spec(
        spec: &api::ResourceClaimTemplateSpec,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(
            &spec.metadata,
            &format!("{}.metadata", field),
            false,
        ));
        errors.extend(validate_resource_claim_spec(&spec.spec, &format!("{}.spec", field)));

        errors
    }

    fn validate_resource_slice_spec(spec: &api::ResourceSliceSpec, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_subdomain(
            &spec.driver,
            &format!("{}.driver", field),
        ));
        errors.extend(validate_resource_pool(&spec.pool, &format!("{}.pool", field)));

        errors.extend(validate_node_selection(
            Some(spec.node_name.as_str()),
            &spec.node_selector,
            &spec.all_nodes,
            field,
            true,
        ));

        let mut device_names = BTreeSet::new();
        for (i, device) in spec.devices.iter().enumerate() {
            let device_field = format!("{}.devices[{}]", field, i);
            errors.extend(validate_device(device, &device_field));

            if !device.name.is_empty() && !device_names.insert(device.name.clone()) {
                errors.push(ValidationError::duplicate(
                    format!("{}.name", device_field),
                    device.name.clone(),
                ));
            }
        }

        errors
    }

    fn validate_resource_pool(pool: &api::ResourcePool, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_pool_name(&pool.name, &format!("{}.name", field)));

        if pool.generation < 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.generation", field),
                "generation must be non-negative",
            ));
        }

        if pool.resource_slice_count <= 0 {
            errors.push(ValidationError::invalid(
                &format!("{}.resourceSliceCount", field),
                "resourceSliceCount must be greater than zero",
            ));
        }

        errors
    }

    fn validate_device(device: &api::Device, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_required_dns_label(
            &device.name,
            &format!("{}.name", field),
        ));

        if let Some(basic) = &device.basic {
            errors.extend(validate_basic_device(basic, &format!("{}.basic", field)));
        }

        errors
    }

    fn validate_basic_device(basic: &api::BasicDevice, field: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (key, value) in &basic.attributes {
            let key_field = format!("{}.attributes[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_attribute(value, &key_field));
        }

        for (key, value) in &basic.capacity {
            let key_field = format!("{}.capacity[{}]", field, key);
            errors.extend(validate_label_key(key, &key_field));
            errors.extend(validate_device_capacity(value, &key_field));
        }

        errors
    }

    fn validate_device_attribute(attr: &api::DeviceAttribute, field: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut set_count = 0;

        if attr.int_value.is_some() {
            set_count += 1;
        }
        if attr.bool_value.is_some() {
            set_count += 1;
        }
        if attr.string_value.is_some() {
            set_count += 1;
        }
        if attr.version_value.is_some() {
            set_count += 1;
        }

        if set_count == 0 {
            errors.push(ValidationError::required(
                field,
                "one of intValue, boolValue, stringValue, or versionValue is required",
            ));
        } else if set_count > 1 {
            errors.push(ValidationError::invalid(
                field,
                "only one of intValue, boolValue, stringValue, or versionValue is allowed",
            ));
        }

        if let Some(value) = &attr.string_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.stringValue", field),
                    "stringValue is required",
                ));
            }
        }

        if let Some(value) = &attr.version_value {
            if value.is_empty() {
                errors.push(ValidationError::required(
                    &format!("{}.versionValue", field),
                    "versionValue is required",
                ));
            }
        }

        errors
    }

    fn validate_device_capacity(capacity: &api::DeviceCapacity, field: &str) -> ValidationResult {
        validate_quantity(&capacity.value, &format!("{}.value", field))
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{v1beta1 as validation_v1beta1, v1beta2 as validation_v1beta2};
    use k8s_api::resource::{v1beta1 as api_v1beta1, v1beta2 as api_v1beta2};
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_resource_claim_v1beta2_valid() {
        let claim = api_v1beta2::ResourceClaim {
            metadata: ObjectMeta::named("claim"),
            spec: api_v1beta2::ResourceClaimSpec {
                devices: Some(api_v1beta2::DeviceClaim {
                    requests: vec![api_v1beta2::DeviceRequest {
                        name: "req".to_string(),
                        exactly: Some(api_v1beta2::ExactDeviceRequest {
                            device_class_name: "example.com".to_string(),
                            allocation_mode: "ExactCount".to_string(),
                            count: Some(1),
                            ..Default::default()
                        }),
                        first_available: Vec::new(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        };

        let errors = validation_v1beta2::validate_resource_claim(&claim);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }

    #[test]
    fn test_validate_resource_claim_v1beta2_missing_class() {
        let claim = api_v1beta2::ResourceClaim {
            metadata: ObjectMeta::named("claim"),
            spec: api_v1beta2::ResourceClaimSpec {
                devices: Some(api_v1beta2::DeviceClaim {
                    requests: vec![api_v1beta2::DeviceRequest {
                        name: "req".to_string(),
                        exactly: Some(api_v1beta2::ExactDeviceRequest {
                            device_class_name: "".to_string(),
                            allocation_mode: "ExactCount".to_string(),
                            count: Some(1),
                            ..Default::default()
                        }),
                        first_available: Vec::new(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        };

        let errors = validation_v1beta2::validate_resource_claim(&claim);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.field.contains("deviceClassName")));
    }

    #[test]
    fn test_validate_resource_slice_v1beta1_node_selection_conflict() {
        let slice = api_v1beta1::ResourceSlice {
            metadata: ObjectMeta::named("slice"),
            spec: api_v1beta1::ResourceSliceSpec {
                driver: "example.com".to_string(),
                pool: api_v1beta1::ResourcePool {
                    name: "pool".to_string(),
                    generation: 1,
                    resource_slice_count: 1,
                },
                node_name: "node-1".to_string(),
                all_nodes: Some(true),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_resource_slice(&slice);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|error| error.field.contains("node")));
    }
}
