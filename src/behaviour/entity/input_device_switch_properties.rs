use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceSwitchProperties {
    #[strum(serialize = "switch")]
    SWITCH,
    #[strum(serialize = "switch_type")]
    SWITCH_TYPE,
    #[strum(serialize = "state")]
    STATE,
}

impl InputDeviceSwitchProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceSwitchProperties::SWITCH => json!(String::new()),
            InputDeviceSwitchProperties::SWITCH_TYPE => json!(-1),
            InputDeviceSwitchProperties::STATE => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceSwitchProperties::SWITCH),
            NamedProperty::from(InputDeviceSwitchProperties::SWITCH_TYPE),
            NamedProperty::from(InputDeviceSwitchProperties::STATE),
        ]
    }
}

impl From<InputDeviceSwitchProperties> for NamedProperty {
    fn from(p: InputDeviceSwitchProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceSwitchProperties> for String {
    fn from(p: InputDeviceSwitchProperties) -> Self {
        p.to_string()
    }
}
