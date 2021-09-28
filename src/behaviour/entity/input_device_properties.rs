use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum InputDeviceProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "physical_path")]
    PHYSICAL_PATH,
    #[strum(serialize = "event")]
    EVENT,
}

impl InputDeviceProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceProperties::NAME => json!(""),
            InputDeviceProperties::PHYSICAL_PATH => json!(""),
            InputDeviceProperties::EVENT => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceProperties::NAME),
            NamedProperty::from(InputDeviceProperties::PHYSICAL_PATH),
            NamedProperty::from(InputDeviceProperties::EVENT),
        ]
    }
}

impl From<InputDeviceProperties> for NamedProperty {
    fn from(p: InputDeviceProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceProperties> for String {
    fn from(p: InputDeviceProperties) -> Self {
        p.to_string()
    }
}
