use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use k8s_api_core::schema::GroupVersionKind;
use once_cell::sync::Lazy;
use prost_reflect::prost::Message;
use prost_reflect::{
    DescriptorPool, DynamicMessage, Kind, MapKey, MessageDescriptor, ReflectMessage, Value,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map as JsonMap, Number as JsonNumber, Value as JsonValue};
use std::collections::HashMap;
use thiserror::Error;

const DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/k8s_api_descriptor_set.bin"));

static DESCRIPTORS: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(DESCRIPTOR_SET.as_ref()).expect("decode Kubernetes descriptors")
});

const INT_OR_STRING: &str = "k8s.io.apimachinery.pkg.util.intstr.IntOrString";
const QUANTITY: &str = "k8s.io.apimachinery.pkg.api.resource.Quantity";
const QUANTITY_VALUE: &str = "k8s.io.apimachinery.pkg.api.resource.QuantityValue";
const TIME: &str = "k8s.io.apimachinery.pkg.apis.meta.v1.Time";
const MICRO_TIME: &str = "k8s.io.apimachinery.pkg.apis.meta.v1.MicroTime";
const FIELDS_V1: &str = "k8s.io.apimachinery.pkg.apis.meta.v1.FieldsV1";
const RAW_EXTENSION: &str = "k8s.io.apimachinery.pkg.runtime.RawExtension";

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("protobuf decode error: {0}")]
    ProtobufDecode(#[from] prost_reflect::prost::DecodeError),
    #[error("protobuf encode error: {0}")]
    ProtobufEncode(#[from] prost_reflect::prost::EncodeError),
    #[error("unknown protobuf message: {0}")]
    UnknownMessage(String),
    #[error("invalid api version: {0}")]
    InvalidApiVersion(String),
    #[error("invalid json: {0}")]
    InvalidJson(String),
    #[error("invalid enum value {value} for {enum_name}")]
    InvalidEnumValue { enum_name: String, value: String },
}

pub fn encode_json<T: Serialize>(value: &T) -> Result<Vec<u8>, CodecError> {
    Ok(serde_json::to_vec(value)?)
}

pub fn decode_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CodecError> {
    Ok(serde_json::from_slice(bytes)?)
}

pub fn encode_protobuf<T: Serialize>(message_name: &str, value: &T) -> Result<Vec<u8>, CodecError> {
    let desc = lookup_message(message_name)?;
    let json = serde_json::to_value(value)?;
    let message = json_to_dynamic_message(desc, &json)?;
    let mut buf = Vec::new();
    message.encode(&mut buf)?;
    Ok(buf)
}

pub fn decode_protobuf<T: DeserializeOwned>(
    message_name: &str,
    bytes: &[u8],
) -> Result<T, CodecError> {
    let desc = lookup_message(message_name)?;
    let message = DynamicMessage::decode(desc, bytes)?;
    let json = dynamic_message_to_json(&message)?;
    Ok(serde_json::from_value(json)?)
}

pub fn proto_message_name(gvk: &GroupVersionKind) -> Result<String, CodecError> {
    if gvk.version.is_empty() || gvk.kind.is_empty() {
        return Err(CodecError::InvalidApiVersion(gvk.api_version()));
    }

    let group_key = if gvk.group.is_empty() {
        "core"
    } else {
        gvk.group.split('.').next().unwrap_or(&gvk.group)
    };

    let prefix = match group_key {
        "apiextensions" => "k8s.io.apiextensions-apiserver.pkg.apis.apiextensions",
        "apiregistration" => "k8s.io.kube-aggregator.pkg.apis.apiregistration",
        "meta" => "k8s.io.apimachinery.pkg.apis.meta",
        _ => "k8s.io.api",
    };

    Ok(format!(
        "{prefix}.{group_key}.{}.{}",
        gvk.version, gvk.kind
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchType {
    Json,
    Merge,
    Strategic,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Patch {
    patch_type: PatchType,
    value: JsonValue,
}

impl Patch {
    pub fn json(value: JsonValue) -> Self {
        Self {
            patch_type: PatchType::Json,
            value,
        }
    }

    pub fn merge(value: JsonValue) -> Self {
        Self {
            patch_type: PatchType::Merge,
            value,
        }
    }

    pub fn strategic(value: JsonValue) -> Self {
        Self {
            patch_type: PatchType::Strategic,
            value,
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self.patch_type {
            PatchType::Json => "application/json-patch+json",
            PatchType::Merge => "application/merge-patch+json",
            PatchType::Strategic => "application/strategic-merge-patch+json",
        }
    }

    pub fn patch_type(&self) -> PatchType {
        self.patch_type
    }

    pub fn value(&self) -> &JsonValue {
        &self.value
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(serde_json::to_vec(&self.value)?)
    }
}

fn lookup_message(name: &str) -> Result<MessageDescriptor, CodecError> {
    DESCRIPTORS
        .get_message_by_name(name)
        .ok_or_else(|| CodecError::UnknownMessage(name.to_string()))
}

fn json_to_dynamic_message(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    match desc.full_name() {
        INT_OR_STRING => json_to_int_or_string(desc, value),
        QUANTITY | QUANTITY_VALUE => json_to_quantity(desc, value),
        TIME => json_to_time(desc, value),
        MICRO_TIME => json_to_micro_time(desc, value),
        FIELDS_V1 => json_to_fields_v1(desc, value),
        RAW_EXTENSION => json_to_raw_extension(desc, value),
        _ => json_to_message_fields(desc, value),
    }
}

fn json_to_message_fields(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    let obj = value.as_object().ok_or_else(|| {
        CodecError::InvalidJson(format!("expected object for {}", desc.full_name()))
    })?;

    let mut message = DynamicMessage::new(desc.clone());
    for field in desc.fields() {
        let key = field.json_name();
        let field_value = match obj.get(key) {
            Some(value) if !value.is_null() => value,
            _ => continue,
        };
        let value = json_to_field_value(&field, field_value)?;
        message
            .try_set_field(&field, value)
            .map_err(|err| CodecError::InvalidJson(format!("{:?}", err)))?;
    }

    Ok(message)
}

fn json_to_field_value(
    field: &prost_reflect::FieldDescriptor,
    value: &JsonValue,
) -> Result<Value, CodecError> {
    if field.is_list() {
        let items = value.as_array().ok_or_else(|| {
            CodecError::InvalidJson(format!(
                "expected list for {}",
                field.full_name()
            ))
        })?;
        let mut list = Vec::with_capacity(items.len());
        for item in items {
            list.push(json_to_singular_value(field, item)?);
        }
        return Ok(Value::List(list));
    }

    if field.is_map() {
        let obj = value.as_object().ok_or_else(|| {
            CodecError::InvalidJson(format!(
                "expected map object for {}",
                field.full_name()
            ))
        })?;
        let entry_desc = match field.kind() {
            Kind::Message(desc) => desc,
            _ => {
                return Err(CodecError::InvalidJson(format!(
                    "invalid map kind for {}",
                    field.full_name()
                )))
            }
        };
        let key_desc = entry_desc
            .get_field_by_name("key")
            .ok_or_else(|| CodecError::InvalidJson("map key not found".to_string()))?;
        let value_desc = entry_desc
            .get_field_by_name("value")
            .ok_or_else(|| CodecError::InvalidJson("map value not found".to_string()))?;

        let mut map = HashMap::new();
        for (key, val) in obj {
            let map_key = json_to_map_key(&key_desc, key)?;
            let map_value = json_to_field_value(&value_desc, val)?;
            map.insert(map_key, map_value);
        }
        return Ok(Value::Map(map));
    }

    json_to_singular_value(field, value)
}

fn json_to_singular_value(
    field: &prost_reflect::FieldDescriptor,
    value: &JsonValue,
) -> Result<Value, CodecError> {
    match field.kind() {
        Kind::Bool => value
            .as_bool()
            .map(Value::Bool)
            .ok_or_else(|| CodecError::InvalidJson(format!("expected bool for {}", field.full_name()))),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => {
            let num = value.as_i64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected i32 for {}", field.full_name()))
            })?;
            let int = i32::try_from(num).map_err(|_| {
                CodecError::InvalidJson(format!("i32 out of range for {}", field.full_name()))
            })?;
            Ok(Value::I32(int))
        }
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => {
            let num = value.as_i64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected i64 for {}", field.full_name()))
            })?;
            Ok(Value::I64(num))
        }
        Kind::Uint32 | Kind::Fixed32 => {
            let num = value.as_u64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected u32 for {}", field.full_name()))
            })?;
            let int = u32::try_from(num).map_err(|_| {
                CodecError::InvalidJson(format!("u32 out of range for {}", field.full_name()))
            })?;
            Ok(Value::U32(int))
        }
        Kind::Uint64 | Kind::Fixed64 => {
            let num = value.as_u64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected u64 for {}", field.full_name()))
            })?;
            Ok(Value::U64(num))
        }
        Kind::Float => {
            let num = value.as_f64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected f32 for {}", field.full_name()))
            })?;
            Ok(Value::F32(num as f32))
        }
        Kind::Double => {
            let num = value.as_f64().ok_or_else(|| {
                CodecError::InvalidJson(format!("expected f64 for {}", field.full_name()))
            })?;
            Ok(Value::F64(num))
        }
        Kind::String => value
            .as_str()
            .map(|s| Value::String(s.to_string()))
            .ok_or_else(|| {
                CodecError::InvalidJson(format!("expected string for {}", field.full_name()))
            }),
        Kind::Bytes => json_to_bytes(value)
            .map(Value::Bytes)
            .map_err(|err| CodecError::InvalidJson(format!("bytes error: {}", err))),
        Kind::Enum(enum_desc) => match value {
            JsonValue::String(name) => enum_desc
                .get_value_by_name(name)
                .map(|v| Value::EnumNumber(v.number()))
                .ok_or_else(|| CodecError::InvalidEnumValue {
                    enum_name: enum_desc.full_name().to_string(),
                    value: name.clone(),
                }),
            JsonValue::Number(num) => num
                .as_i64()
                .map(|val| Value::EnumNumber(val as i32))
                .ok_or_else(|| {
                    CodecError::InvalidJson(format!("enum number error for {}", field.full_name()))
                }),
            _ => Err(CodecError::InvalidJson(format!(
                "expected enum for {}",
                field.full_name()
            ))),
        },
        Kind::Message(desc) => Ok(Value::Message(json_to_dynamic_message(desc, value)?)),
    }
}

fn json_to_map_key(
    key_desc: &prost_reflect::FieldDescriptor,
    value: &str,
) -> Result<MapKey, CodecError> {
    match key_desc.kind() {
        Kind::Bool => value
            .parse::<bool>()
            .map(MapKey::Bool)
            .map_err(|_| CodecError::InvalidJson("invalid bool map key".to_string())),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => value
            .parse::<i32>()
            .map(MapKey::I32)
            .map_err(|_| CodecError::InvalidJson("invalid i32 map key".to_string())),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => value
            .parse::<i64>()
            .map(MapKey::I64)
            .map_err(|_| CodecError::InvalidJson("invalid i64 map key".to_string())),
        Kind::Uint32 | Kind::Fixed32 => value
            .parse::<u32>()
            .map(MapKey::U32)
            .map_err(|_| CodecError::InvalidJson("invalid u32 map key".to_string())),
        Kind::Uint64 | Kind::Fixed64 => value
            .parse::<u64>()
            .map(MapKey::U64)
            .map_err(|_| CodecError::InvalidJson("invalid u64 map key".to_string())),
        Kind::String => Ok(MapKey::String(value.to_string())),
        _ => Err(CodecError::InvalidJson("unsupported map key type".to_string())),
    }
}

fn json_to_bytes(value: &JsonValue) -> Result<prost_reflect::bytes::Bytes, String> {
    match value {
        JsonValue::String(s) => match general_purpose::STANDARD.decode(s) {
            Ok(bytes) => Ok(bytes.into()),
            Err(_) => Ok(s.as_bytes().to_vec().into()),
        },
        JsonValue::Array(items) => {
            let mut bytes = Vec::with_capacity(items.len());
            for item in items {
                let num = item
                    .as_u64()
                    .ok_or_else(|| "expected byte array".to_string())?;
                let byte = u8::try_from(num).map_err(|_| "byte out of range".to_string())?;
                bytes.push(byte);
            }
            Ok(bytes.into())
        }
        _ => Err("expected bytes".to_string()),
    }
}

fn json_to_int_or_string(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    if value.is_object() {
        return json_to_message_fields(desc, value);
    }

    let type_field = find_field(&desc, &["type"])?;
    let int_field = find_field(&desc, &["intVal"])?;
    let str_field = find_field(&desc, &["strVal"])?;
    let mut message = DynamicMessage::new(desc);

    match value {
        JsonValue::Number(num) => {
            let int = num.as_i64().ok_or_else(|| {
                CodecError::InvalidJson("invalid int or string number".to_string())
            })?;
            let int = i32::try_from(int).map_err(|_| {
                CodecError::InvalidJson("int or string out of range".to_string())
            })?;
            message.set_field(&type_field, Value::I64(0));
            message.set_field(&int_field, Value::I32(int));
        }
        JsonValue::String(text) => {
            message.set_field(&type_field, Value::I64(1));
            message.set_field(&str_field, Value::String(text.clone()));
        }
        _ => {
            return Err(CodecError::InvalidJson(
                "expected int or string".to_string(),
            ))
        }
    }

    Ok(message)
}

fn json_to_quantity(desc: MessageDescriptor, value: &JsonValue) -> Result<DynamicMessage, CodecError> {
    if value.is_object() {
        return json_to_message_fields(desc, value);
    }

    let string_field = find_field(&desc, &["string"])?;
    let mut message = DynamicMessage::new(desc);
    let text = match value {
        JsonValue::String(text) => text.clone(),
        JsonValue::Number(num) => num.to_string(),
        _ => {
            return Err(CodecError::InvalidJson(
                "expected string for quantity".to_string(),
            ))
        }
    };
    message.set_field(&string_field, Value::String(text));
    Ok(message)
}

fn json_to_time(desc: MessageDescriptor, value: &JsonValue) -> Result<DynamicMessage, CodecError> {
    if value.is_object() {
        return json_to_message_fields(desc, value);
    }
    let (seconds, nanos) = parse_time_value(value)?;
    time_message_from_parts(desc, seconds, nanos)
}

fn json_to_micro_time(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    if value.is_object() {
        return json_to_message_fields(desc, value);
    }
    let (seconds, nanos) = parse_time_value(value)?;
    time_message_from_parts(desc, seconds, nanos)
}

fn time_message_from_parts(
    desc: MessageDescriptor,
    seconds: i64,
    nanos: i32,
) -> Result<DynamicMessage, CodecError> {
    let seconds_field = find_field(&desc, &["seconds"])?;
    let nanos_field = find_field(&desc, &["nanos"])?;
    let mut message = DynamicMessage::new(desc);
    message.set_field(&seconds_field, Value::I64(seconds));
    message.set_field(&nanos_field, Value::I32(nanos));
    Ok(message)
}

fn parse_time_value(value: &JsonValue) -> Result<(i64, i32), CodecError> {
    match value {
        JsonValue::Number(num) => num
            .as_i64()
            .map(|seconds| (seconds, 0))
            .ok_or_else(|| CodecError::InvalidJson("invalid time number".to_string())),
        JsonValue::String(text) => {
            let dt = DateTime::parse_from_rfc3339(text)
                .map_err(|err| CodecError::InvalidJson(err.to_string()))?
                .with_timezone(&Utc);
            Ok((dt.timestamp(), dt.timestamp_subsec_nanos() as i32))
        }
        _ => Err(CodecError::InvalidJson(
            "expected time as number or string".to_string(),
        )),
    }
}

fn json_to_fields_v1(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    let raw_field = find_field(&desc, &["Raw", "raw"])?;
    let mut message = DynamicMessage::new(desc);
    let bytes = match value {
        JsonValue::String(text) => general_purpose::STANDARD
            .decode(text)
            .unwrap_or_else(|_| text.as_bytes().to_vec()),
        _ => serde_json::to_vec(value)?,
    };
    message.set_field(&raw_field, Value::Bytes(bytes.into()));
    Ok(message)
}

fn json_to_raw_extension(
    desc: MessageDescriptor,
    value: &JsonValue,
) -> Result<DynamicMessage, CodecError> {
    let raw_field = find_field(&desc, &["raw", "Raw"])?;
    let mut message = DynamicMessage::new(desc);
    let bytes = match value {
        JsonValue::String(text) => general_purpose::STANDARD
            .decode(text)
            .unwrap_or_else(|_| text.as_bytes().to_vec()),
        _ => serde_json::to_vec(value)?,
    };
    message.set_field(&raw_field, Value::Bytes(bytes.into()));
    Ok(message)
}

fn dynamic_message_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let desc = message.descriptor();
    match desc.full_name() {
        INT_OR_STRING => int_or_string_to_json(message),
        QUANTITY | QUANTITY_VALUE => quantity_to_json(message),
        TIME => time_to_json(message),
        MICRO_TIME => micro_time_to_json(message),
        FIELDS_V1 => fields_v1_to_json(message),
        RAW_EXTENSION => raw_extension_to_json(message),
        _ => message_fields_to_json(message),
    }
}

fn message_fields_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let desc = message.descriptor();
    let mut obj = JsonMap::new();
    for field in desc.fields() {
        if !message.has_field(&field) {
            continue;
        }
        let value = message.get_field(&field);
        let json = value_to_json(&field, value.as_ref())?;
        obj.insert(field.json_name().to_string(), json);
    }
    Ok(JsonValue::Object(obj))
}

fn value_to_json(
    field: &prost_reflect::FieldDescriptor,
    value: &Value,
) -> Result<JsonValue, CodecError> {
    if field.is_list() {
        let list = match value {
            Value::List(list) => list,
            _ => {
                return Err(CodecError::InvalidJson(format!(
                    "expected list for {}",
                    field.full_name()
                )))
            }
        };
        let mut items = Vec::with_capacity(list.len());
        for item in list {
            items.push(value_to_singular_json(field, item)?);
        }
        return Ok(JsonValue::Array(items));
    }

    if field.is_map() {
        let map = match value {
            Value::Map(map) => map,
            _ => {
                return Err(CodecError::InvalidJson(format!(
                    "expected map for {}",
                    field.full_name()
                )))
            }
        };
        let entry_desc = match field.kind() {
            Kind::Message(desc) => desc,
            _ => {
                return Err(CodecError::InvalidJson(format!(
                    "invalid map kind for {}",
                    field.full_name()
                )))
            }
        };
        let value_desc = entry_desc
            .get_field_by_name("value")
            .ok_or_else(|| CodecError::InvalidJson("map value not found".to_string()))?;

        let mut obj = JsonMap::new();
        for (key, val) in map {
            obj.insert(map_key_to_string(key), value_to_json(&value_desc, val)?);
        }
        return Ok(JsonValue::Object(obj));
    }

    value_to_singular_json(field, value)
}

fn value_to_singular_json(
    field: &prost_reflect::FieldDescriptor,
    value: &Value,
) -> Result<JsonValue, CodecError> {
    match (field.kind(), value) {
        (Kind::Bool, Value::Bool(v)) => Ok(JsonValue::Bool(*v)),
        (Kind::Int32 | Kind::Sint32 | Kind::Sfixed32, Value::I32(v)) => {
            Ok(JsonValue::Number((*v).into()))
        }
        (Kind::Int64 | Kind::Sint64 | Kind::Sfixed64, Value::I64(v)) => {
            Ok(JsonValue::Number((*v).into()))
        }
        (Kind::Uint32 | Kind::Fixed32, Value::U32(v)) => Ok(JsonValue::Number((*v).into())),
        (Kind::Uint64 | Kind::Fixed64, Value::U64(v)) => Ok(JsonValue::Number((*v).into())),
        (Kind::Float, Value::F32(v)) => JsonNumber::from_f64(*v as f64)
            .map(JsonValue::Number)
            .ok_or_else(|| CodecError::InvalidJson("invalid f32".to_string())),
        (Kind::Double, Value::F64(v)) => JsonNumber::from_f64(*v)
            .map(JsonValue::Number)
            .ok_or_else(|| CodecError::InvalidJson("invalid f64".to_string())),
        (Kind::String, Value::String(v)) => Ok(JsonValue::String(v.clone())),
        (Kind::Bytes, Value::Bytes(v)) => Ok(bytes_to_json(v)),
        (Kind::Enum(enum_desc), Value::EnumNumber(v)) => {
            if let Some(name) = enum_desc.get_value(*v) {
                Ok(JsonValue::String(name.name().to_string()))
            } else {
                Ok(JsonValue::Number((*v).into()))
            }
        }
        (Kind::Message(_), Value::Message(msg)) => dynamic_message_to_json(msg),
        _ => Err(CodecError::InvalidJson(format!(
            "unexpected value for {}",
            field.full_name()
        ))),
    }
}

fn bytes_to_json(bytes: &prost_reflect::bytes::Bytes) -> JsonValue {
    let items = bytes
        .iter()
        .map(|b| JsonValue::Number((*b).into()))
        .collect::<Vec<_>>();
    JsonValue::Array(items)
}

fn map_key_to_string(key: &MapKey) -> String {
    match key {
        MapKey::Bool(v) => v.to_string(),
        MapKey::I32(v) => v.to_string(),
        MapKey::I64(v) => v.to_string(),
        MapKey::U32(v) => v.to_string(),
        MapKey::U64(v) => v.to_string(),
        MapKey::String(v) => v.clone(),
    }
}

fn int_or_string_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    if message.has_field_by_name("strVal") {
        if let Some(Value::String(text)) = message.get_field_by_name("strVal").as_deref() {
            return Ok(JsonValue::String(text.clone()));
        }
    }

    if message.has_field_by_name("intVal") {
        if let Some(Value::I32(num)) = message.get_field_by_name("intVal").as_deref() {
            return Ok(JsonValue::Number((*num).into()));
        }
    }

    Ok(JsonValue::Null)
}

fn quantity_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    if let Some(Value::String(text)) = message.get_field_by_name("string").as_deref() {
        return Ok(JsonValue::String(text.clone()));
    }
    Ok(JsonValue::Null)
}

fn time_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let (seconds, nanos) = time_parts_from_message(message)?;
    if nanos == 0 {
        return Ok(JsonValue::Number(seconds.into()));
    }
    let total = seconds as f64 + (nanos as f64 / 1_000_000_000_f64);
    JsonNumber::from_f64(total)
        .map(JsonValue::Number)
        .ok_or_else(|| CodecError::InvalidJson("invalid time float".to_string()))
}

fn micro_time_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let (seconds, nanos) = time_parts_from_message(message)?;
    let dt = Utc
        .timestamp_opt(seconds, nanos as u32)
        .single()
        .ok_or_else(|| CodecError::InvalidJson("invalid timestamp".to_string()))?;
    Ok(JsonValue::String(
        dt.to_rfc3339_opts(SecondsFormat::Nanos, true),
    ))
}

fn fields_v1_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let raw = get_bytes_field(message, &["Raw", "raw"])?;
    Ok(serde_json::from_slice(&raw)?)
}

fn raw_extension_to_json(message: &DynamicMessage) -> Result<JsonValue, CodecError> {
    let raw = get_bytes_field(message, &["raw", "Raw"])?;
    Ok(serde_json::from_slice(&raw)?)
}

fn time_parts_from_message(message: &DynamicMessage) -> Result<(i64, i32), CodecError> {
    let seconds = match message.get_field_by_name("seconds").as_deref() {
        Some(Value::I64(val)) => *val,
        _ => 0,
    };
    let nanos = match message.get_field_by_name("nanos").as_deref() {
        Some(Value::I32(val)) => *val,
        _ => 0,
    };
    Ok((seconds, nanos))
}

fn get_bytes_field(message: &DynamicMessage, names: &[&str]) -> Result<Vec<u8>, CodecError> {
    for name in names {
        if let Some(Value::Bytes(bytes)) = message.get_field_by_name(name).as_deref() {
            return Ok(bytes.to_vec());
        }
    }
    Err(CodecError::InvalidJson("bytes field not found".to_string()))
}

fn find_field(
    desc: &MessageDescriptor,
    names: &[&str],
) -> Result<prost_reflect::FieldDescriptor, CodecError> {
    for name in names {
        if let Some(field) = desc.get_field_by_name(name) {
            return Ok(field);
        }
    }
    Err(CodecError::InvalidJson("field not found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::core::v1::Namespace;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_patch_content_type() {
        let patch = Patch::strategic(JsonValue::Object(JsonMap::new()));
        assert_eq!(
            patch.content_type(),
            "application/strategic-merge-patch+json"
        );
    }

    #[test]
    fn test_protobuf_roundtrip_namespace() {
        let namespace = Namespace {
            metadata: ObjectMeta::named("codec-test"),
            ..Default::default()
        };
        let gvk = GroupVersionKind::new("", "v1", "Namespace");
        let message_name = proto_message_name(&gvk).unwrap();
        let bytes = encode_protobuf(&message_name, &namespace).unwrap();
        let decoded: Namespace = decode_protobuf(&message_name, &bytes).unwrap();
        assert_eq!(decoded.metadata.name, "codec-test");
    }
}
