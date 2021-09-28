use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum InputDeviceKeyProperties {
    #[strum(serialize = "keycode")]
    KEYCODE,
    #[strum(serialize = "keydown")]
    KEYDOWN,
}

impl InputDeviceKeyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceKeyProperties::KEYCODE => json!(-1),
            InputDeviceKeyProperties::KEYDOWN => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceKeyProperties::KEYCODE),
            NamedProperty::from(InputDeviceKeyProperties::KEYDOWN),
        ]
    }
}

impl From<InputDeviceKeyProperties> for NamedProperty {
    fn from(p: InputDeviceKeyProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceKeyProperties> for String {
    fn from(p: InputDeviceKeyProperties) -> Self {
        p.to_string()
    }
}
