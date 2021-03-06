use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum InputDeviceRelativeAxisProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "label")]
    LABEL,
    #[strum(serialize = "relative_axis")]
    RELATIVE_AXIS,
    #[strum(serialize = "relative_axis_type")]
    RELATIVE_AXIS_TYPE,
    #[strum(serialize = "state")]
    STATE,
}

impl InputDeviceRelativeAxisProperties {
    pub fn default_value(&self) -> Value {
        match self {
            InputDeviceRelativeAxisProperties::NAME => json!(String::new()),
            InputDeviceRelativeAxisProperties::LABEL => json!(String::new()),
            InputDeviceRelativeAxisProperties::RELATIVE_AXIS => json!(String::new()),
            InputDeviceRelativeAxisProperties::RELATIVE_AXIS_TYPE => json!(-1),
            InputDeviceRelativeAxisProperties::STATE => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(InputDeviceRelativeAxisProperties::NAME),
            NamedProperty::from(InputDeviceRelativeAxisProperties::LABEL),
            NamedProperty::from(InputDeviceRelativeAxisProperties::RELATIVE_AXIS),
            NamedProperty::from(InputDeviceRelativeAxisProperties::RELATIVE_AXIS_TYPE),
            NamedProperty::from(InputDeviceRelativeAxisProperties::STATE),
        ]
    }
}

impl From<InputDeviceRelativeAxisProperties> for NamedProperty {
    fn from(p: InputDeviceRelativeAxisProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<InputDeviceRelativeAxisProperties> for String {
    fn from(p: InputDeviceRelativeAxisProperties) -> Self {
        p.to_string()
    }
}
