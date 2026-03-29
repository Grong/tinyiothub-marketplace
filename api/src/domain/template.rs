use serde::{Deserialize, Serialize};

/// 多语言字符串
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalizedString {
    #[serde(default)]
    pub zh: Option<String>,
    #[serde(default)]
    pub en: Option<String>,
}

impl LocalizedString {
    #[allow(dead_code)]
    pub fn get(&self, lang: &str) -> &str {
        match lang {
            "zh" if self.zh.is_some() => self.zh.as_ref().unwrap(),
            "en" if self.en.is_some() => self.en.as_ref().unwrap(),
            _ => self.zh.as_ref().or(self.en.as_ref()).map(|s| s.as_str()).unwrap_or(""),
        }
    }
}

/// 属性定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub display_name: LocalizedString,
    pub description: LocalizedString,
    pub data_type: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub min_value: Option<f64>,
    #[serde(default)]
    pub max_value: Option<f64>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_read_only: bool,
    #[serde(default)]
    pub is_required: bool,
}

/// 命令定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub display_name: LocalizedString,
    pub description: LocalizedString,
    #[serde(default)]
    pub parameters: Option<String>,
    #[serde(default)]
    pub parameter_schema: Option<String>,
    #[serde(default)]
    pub is_required: bool,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    #[serde(default)]
    pub default_name_pattern: Option<String>,
    #[serde(default)]
    pub default_display_name_pattern: Option<LocalizedString>,
    #[serde(default)]
    pub default_description: Option<LocalizedString>,
    #[serde(default)]
    pub required_fields: Vec<String>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            default_name_pattern: None,
            default_display_name_pattern: None,
            default_description: None,
            required_fields: Vec::new(),
        }
    }
}

/// 设备模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub display_name: LocalizedString,
    pub description: LocalizedString,
    pub version: String,
    pub author: String,
    pub category: String,
    #[serde(default)]
    pub manufacturer: Option<String>,
    pub device_type: String,
    pub protocol_type: String,
    pub driver_name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub device_info: DeviceInfo,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub commands: Vec<Command>,
}
