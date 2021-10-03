use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum InputDeviceLedProperties {
    #[strum(serialize = "led")]
    LED,
    #[strum(serialize = "led_type")]
    LED_TYPE,
    #[strum(serialize = "state")]
    STATE,
}

impl InputDeviceLedProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceLedProperties::LED => json!(String::new()),
            InputDeviceLedProperties::LED_TYPE => json!(-1),
            InputDeviceLedProperties::STATE => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceLedProperties::LED),
            NamedProperty::from(InputDeviceLedProperties::LED_TYPE),
            NamedProperty::from(InputDeviceLedProperties::STATE),
        ]
    }
}

impl From<InputDeviceLedProperties> for NamedProperty {
    fn from(p: InputDeviceLedProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceLedProperties> for String {
    fn from(p: InputDeviceLedProperties) -> Self {
        p.to_string()
    }
}
