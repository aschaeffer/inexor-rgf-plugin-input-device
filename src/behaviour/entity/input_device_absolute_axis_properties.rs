use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum InputDeviceAbsoluteAxisProperties {
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
            InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS => json!(String::new()),
            InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS_TYPE => json!(-1),
            InputDeviceAbsoluteAxisProperties::STATE => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS_TYPE),
            NamedProperty::from(InputDeviceAbsoluteAxisProperties::STATE),
        ]
    }
}

impl From<InputDeviceAbsoluteAxisProperties> for NamedProperty {
    fn from(p: InputDeviceAbsoluteAxisProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceAbsoluteAxisProperties> for String {
    fn from(p: InputDeviceAbsoluteAxisProperties) -> Self {
        p.to_string()
    }
}
