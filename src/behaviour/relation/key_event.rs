use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::json;

use crate::behaviour::entity::input_device_key_properties::InputDeviceKeyProperties;
use crate::behaviour::entity::input_device_properties::InputDeviceProperties;
use crate::behaviour::event_payload::{
    INPUT_EVENT_KIND, INPUT_EVENT_KIND_KEY_EVENT, INPUT_EVENT_VALUE, KEY_EVENT_KEY_CODE,
};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use inexor_rgf_core_model::PropertyInstanceSetter;
use inexor_rgf_core_reactive::BehaviourCreationError;

const KEY_EVENT: &'static str = "key_event";

pub struct KeyEvent {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl KeyEvent {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> Result<KeyEvent, BehaviourCreationError> {
        let input_device = r.outbound.clone();
        let input_device_key = r.inbound.clone();
        let input_device_key_key_code = input_device_key.as_i64(InputDeviceKeyProperties::KEY_CODE);
        if input_device_key_key_code.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let input_device_key_key_code = input_device_key_key_code.unwrap();

        let handle_id = input_device
            .properties
            .get(InputDeviceProperties::EVENT.as_ref())
            .unwrap()
            .id
            .as_u128();

        input_device
            .properties
            .get(InputDeviceProperties::EVENT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    let event = v.clone();
                    // debug!("{}", event);
                    if !event.is_object() {
                        return;
                    }
                    let input_event_kind = event.get(INPUT_EVENT_KIND);
                    if input_event_kind.is_none() {
                        return;
                    }

                    match input_event_kind.unwrap().as_str().unwrap() {
                        INPUT_EVENT_KIND_KEY_EVENT => {
                            let event_key_code = event
                                .get(KEY_EVENT_KEY_CODE)
                                .unwrap()
                                .as_i64()
                                .unwrap_or(-1);
                            if input_device_key_key_code == event_key_code {
                                let old_value = input_device_key
                                    .get(InputDeviceKeyProperties::KEY_DOWN)
                                    .unwrap()
                                    .as_bool()
                                    .unwrap();
                                let default = json!(-1);
                                let value = event
                                    .get(INPUT_EVENT_VALUE)
                                    .unwrap_or(&default)
                                    .as_i64()
                                    .unwrap();
                                match value {
                                    // Key Up
                                    0 => {
                                        if old_value {
                                            input_device_key.set(
                                                InputDeviceKeyProperties::KEY_DOWN.to_string(),
                                                json!(false),
                                            )
                                        }
                                    }
                                    // Key Down
                                    1 => {
                                        if !old_value {
                                            input_device_key.set(
                                                InputDeviceKeyProperties::KEY_DOWN.to_string(),
                                                json!(true),
                                            );
                                        }
                                    }
                                    // Key Hold
                                    2 => {
                                        if !old_value {
                                            input_device_key.set(
                                                InputDeviceKeyProperties::KEY_DOWN.to_string(),
                                                json!(true),
                                            )
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                },
                handle_id,
            );

        Ok(KeyEvent {
            relation: r.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for KeyEvent {
    fn disconnect(&self) {
        debug!(
            "Disconnecting behaviour {} from property instance {}",
            KEY_EVENT, self.handle_id
        );
        let property = self
            .relation
            .inbound
            .properties
            .get(InputDeviceProperties::EVENT.as_ref());
        if property.is_some() {
            property
                .unwrap()
                .stream
                .read()
                .unwrap()
                .remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for KeyEvent {
    fn drop(&mut self) {
        self.disconnect();
    }
}
