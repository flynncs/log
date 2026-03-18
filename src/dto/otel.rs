use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AnyValue {
    StringValue {
        #[serde(rename = "stringValue")]
        string_value: String,
    },

    IntValue {
        #[serde(rename = "intValue")]
        int_value: i64,
    },

    BoolValue {
        #[serde(rename = "boolValue")]
        bool_value: bool,
    },

    DoubleValue {
        #[serde(rename = "doubleValue")]
        double_value: f64,
    },
}

#[derive(Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: AnyValue,
}

#[derive(Deserialize)]
pub struct Resource {
    pub attributes: Vec<KeyValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogRecord {
    pub time_unix_nano: String,
    pub severity_number: i64,
    pub body: Option<AnyValue>,
    pub attributes: Vec<KeyValue>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeLog {
    pub log_records: Vec<LogRecord>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLog {
    pub resource: Resource,
    pub scope_logs: Vec<ScopeLog>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtelLogsRequest {
    pub resource_logs: Vec<ResourceLog>,
}
