use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceLedProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "label")]
    LABEL,
    #[strum(serialize = "led")]
    LED,
    #[strum(serialize = "led_type")]
    LED_TYPE,
    #[strum(serialize = "state")]
    STATE,
    #[strum(serialize = "set_state")]
    SET_STATE,
}

impl InputDeviceLedProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceLedProperties::NAME => json!(String::new()),
            InputDeviceLedProperties::LABEL => json!(String::new()),
            InputDeviceLedProperties::LED => json!(String::new()),
            InputDeviceLedProperties::LED_TYPE => json!(-1),
            InputDeviceLedProperties::STATE => json!(false),
            InputDeviceLedProperties::SET_STATE => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceLedProperties::NAME),
            NamedProperty::from(InputDeviceLedProperties::LABEL),
            NamedProperty::from(InputDeviceLedProperties::LED),
            NamedProperty::from(InputDeviceLedProperties::LED_TYPE),
            NamedProperty::from(InputDeviceLedProperties::STATE),
            NamedProperty::from(InputDeviceLedProperties::SET_STATE),
        ]
    }
}

impl From<InputDeviceLedProperties> for NamedProperty {
    fn from(p: InputDeviceLedProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceLedProperties> for String {
    fn from(p: InputDeviceLedProperties) -> Self {
        p.to_string()
    }
}
