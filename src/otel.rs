use chrono::{DateTime, Utc};
use serde_json::{Value, json};

use crate::{
    dto::otel::{AnyValue, KeyValue, LogRecord, OtelLogsRequest, ResourceLog},
    model::{LogLevel, NewLogEntry},
};

fn extract_service(resource_log: &ResourceLog) -> Option<String> {
    let service_attribute = resource_log
        .resource
        .attributes
        .iter()
        .find(|attribute| attribute.key == "service.name");

    match service_attribute {
        Some(kv) => match &kv.value {
            AnyValue::StringValue { string_value } => Some(string_value.clone()),
            _ => None,
        },
        None => None,
    }
}

fn otel_int_to_level(value: &i64) -> LogLevel {
    match value {
        1..=4 => LogLevel::Debug,
        5..=8 => LogLevel::Info,
        9..=12 => LogLevel::Warn,
        _ => LogLevel::Error,
    }
}

fn attributes_to_json(attributes: Vec<KeyValue>) -> Value {
    let mut map = serde_json::Map::new();

    for kv in attributes {
        let value = match kv.value {
            AnyValue::StringValue { string_value } => json!(string_value),
            AnyValue::IntValue { int_value } => json!(int_value),
            AnyValue::DoubleValue { double_value } => json!(double_value),
            AnyValue::BoolValue { bool_value } => json!(bool_value),
        };

        map.insert(kv.key, value);
    }

    return serde_json::Value::Object(map);
}

fn map_log_record(record: LogRecord, service: &String) -> NewLogEntry {
    let level = otel_int_to_level(&record.severity_number);
    let message = match record.body {
        Some(AnyValue::StringValue { string_value }) => string_value,
        _ => "missing".to_string(),
    };
    let attributes = attributes_to_json(record.attributes);

    NewLogEntry {
        level,
        service: service.clone(),
        message,
        attributes,
        trace_id: record.trace_id,
        span_id: record.span_id,
        timestamp: record
            .time_unix_nano
            .parse::<i64>()
            .map(|nanos| DateTime::from_timestamp_nanos(nanos))
            .unwrap_or_else(|_| Utc::now()),
    }
}

pub fn otel_to_log_entries(request: OtelLogsRequest) -> Vec<NewLogEntry> {
    request
        .resource_logs
        .into_iter()
        .flat_map(|resource_log| {
            let service_name =
                extract_service(&resource_log).unwrap_or_else(|| "unkown".to_string());
            resource_log
                .scope_logs
                .into_iter()
                .flat_map(move |scope_log| {
                    let service_name = service_name.clone();
                    scope_log
                        .log_records
                        .into_iter()
                        .map(move |log_record| map_log_record(log_record, &service_name))
                })
        })
        .collect()
}
