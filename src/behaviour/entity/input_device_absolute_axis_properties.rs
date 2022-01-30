use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceAbsoluteAxisProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "label")]
    LABEL,
    #[strum(serialize = "absolute_axis")]
    ABSOLUTE_AXIS,
    #[strum(serialize = "absolute_axis_type")]
    ABSOLUTE_AXIS_TYPE,
    #[strum(serialize = "state")]
    STATE,
}

impl InputDeviceAbsoluteAxisProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceAbsoluteAxisProperties::NAME => json!(String::new()),
            InputDeviceAbsoluteAxisProperties::LABEL => json!(String::new()),
            InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS => json!(String::new()),
            InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS_TYPE => json!(-1),
            InputDeviceAbsoluteAxisProperties::STATE => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::NAME),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::LABEL),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS_TYPE),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::STATE),
        ]
    }
}

impl From<InputDeviceAbsoluteAxisProperties> for NamedProperty {
    fn from(p: InputDeviceAbsoluteAxisProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceAbsoluteAxisProperties> for String {
    fn from(p: InputDeviceAbsoluteAxisProperties) -> Self {
        p.to_string()
    }
}
