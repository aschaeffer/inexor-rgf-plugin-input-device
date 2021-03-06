use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceKeyProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "label")]
    LABEL,
    #[strum(serialize = "key")]
    KEY,
    #[strum(serialize = "key_code")]
    KEY_CODE,
    #[strum(serialize = "key_down")]
    KEY_DOWN,
    #[strum(serialize = "set_key_down")]
    SET_KEY_DOWN,
}

impl InputDeviceKeyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceKeyProperties::NAME => json!(String::new()),
            InputDeviceKeyProperties::LABEL => json!(String::new()),
            InputDeviceKeyProperties::KEY => json!(String::new()),
            InputDeviceKeyProperties::KEY_CODE => json!(-1),
            InputDeviceKeyProperties::KEY_DOWN => json!(false),
            InputDeviceKeyProperties::SET_KEY_DOWN => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceKeyProperties::NAME),
            NamedProperty::from(InputDeviceKeyProperties::LABEL),
            NamedProperty::from(InputDeviceKeyProperties::KEY),
            NamedProperty::from(InputDeviceKeyProperties::KEY_CODE),
            NamedProperty::from(InputDeviceKeyProperties::KEY_DOWN),
            NamedProperty::from(InputDeviceKeyProperties::SET_KEY_DOWN),
        ]
    }
}

impl From<InputDeviceKeyProperties> for NamedProperty {
    fn from(p: InputDeviceKeyProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceKeyProperties> for String {
    fn from(p: InputDeviceKeyProperties) -> Self {
        p.to_string()
    }
}
