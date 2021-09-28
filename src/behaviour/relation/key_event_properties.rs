// use indradb::NamedProperty;
// use inexor_rgf_core_reactive::NamedProperties;
// use serde_json::{json, Value};
// use strum_macros::{AsRefStr, IntoStaticStr, ToString};
//
// #[allow(non_camel_case_types)]
// #[derive(AsRefStr, IntoStaticStr, ToString)]
// pub enum KeyEventProperties {
//     #[strum(serialize = "keycode")]
//     KEYCODE,
//     #[strum(serialize = "keydown")]
//     KEYDOWN,
// }
//
// impl KeyEventProperties {
//     pub fn default_value(&self) -> Value {
//         match self {
//             KeyEventProperties::KEYCODE => json!(32),
//             KeyEventProperties::KEYDOWN => json!(false),
//         }
//     }
//     pub fn properties() -> NamedProperties {
//         vec![
//             NamedProperty::from(KeyEventProperties::KEYCODE),
//             NamedProperty::from(KeyEventProperties::KEYDOWN),
//         ]
//     }
// }
//
// impl From<KeyEventProperties> for NamedProperty {
//     fn from(p: KeyEventProperties) -> Self {
//         NamedProperty {
//             name: p.to_string(),
//             value: p.default_value(),
//         }
//     }
// }
//
// impl From<KeyEventProperties> for String {
//     fn from(p: KeyEventProperties) -> Self {
//         p.to_string()
//     }
// }
