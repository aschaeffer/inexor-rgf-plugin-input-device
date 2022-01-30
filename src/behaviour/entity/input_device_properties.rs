use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "label")]
    LABEL,
    #[strum(serialize = "physical_path")]
    PHYSICAL_PATH,
    #[strum(serialize = "driver_version")]
    DRIVER_VERSION,
    #[strum(serialize = "vendor")]
    VENDOR,
    #[strum(serialize = "product")]
    PRODUCT,
    #[strum(serialize = "version")]
    VERSION,
    #[strum(serialize = "event")]
    EVENT,
    #[strum(serialize = "send_event")]
    SEND_EVENT,
}

impl InputDeviceProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceProperties::NAME => json!(String::new()),
            InputDeviceProperties::LABEL => json!(String::new()),
            InputDeviceProperties::PHYSICAL_PATH => json!(""),
            InputDeviceProperties::DRIVER_VERSION => json!("1.0.0"),
            InputDeviceProperties::VENDOR => json!(0),
            InputDeviceProperties::PRODUCT => json!(0),
            InputDeviceProperties::VERSION => json!(0),
            InputDeviceProperties::EVENT => json!({}),
            InputDeviceProperties::SEND_EVENT => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceProperties::NAME),
            NamedProperty::from(InputDeviceProperties::LABEL),
            NamedProperty::from(InputDeviceProperties::PHYSICAL_PATH),
            NamedProperty::from(InputDeviceProperties::DRIVER_VERSION),
            NamedProperty::from(InputDeviceProperties::VENDOR),
            NamedProperty::from(InputDeviceProperties::PRODUCT),
            NamedProperty::from(InputDeviceProperties::VERSION),
            NamedProperty::from(InputDeviceProperties::EVENT),
            NamedProperty::from(InputDeviceProperties::SEND_EVENT),
        ]
    }
}

impl From<InputDeviceProperties> for NamedProperty {
    fn from(p: InputDeviceProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceProperties> for String {
    fn from(p: InputDeviceProperties) -> Self {
        p.to_string()
    }
}
