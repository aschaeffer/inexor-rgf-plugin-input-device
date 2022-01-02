use evdev::EventType;
use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::json;

use crate::behaviour::entity::input_device_key_properties::InputDeviceKeyProperties;
use crate::behaviour::entity::input_device_properties::InputDeviceProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use inexor_rgf_core_reactive::BehaviourCreationError;

pub const SEND_KEY_EVENT: &'static str = "send_key_event";

pub struct SendKeyEvent {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl SendKeyEvent {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> Result<SendKeyEvent, BehaviourCreationError> {
        let input_device_key = r.outbound.clone();
        let input_device = r.inbound.clone();
        let input_device_key_key_code = input_device_key.as_i64(InputDeviceKeyProperties::KEY_CODE);
        if input_device_key_key_code.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let event_type = u64::try_from(EventType::KEY.0).unwrap();
        let code = input_device_key_key_code.unwrap();
        let handle_id = input_device_key
            .properties
            .get(InputDeviceKeyProperties::SET_KEY_DOWN.as_ref())
            .unwrap()
            .id
            .as_u128();

        input_device_key
            .properties
            .get(InputDeviceKeyProperties::SET_KEY_DOWN.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    if !v.is_boolean() {
                        // Invalid input
                        return;
                    }
                    let send_event = input_device.properties.get(InputDeviceProperties::SEND_EVENT.as_ref()).unwrap();
                    let event = json!({
                        "event_type": event_type, // u64
                        "code": code, // i64
                        "value": v // bool
                    });
                    send_event.set(event);
                },
                handle_id,
            );

        Ok(SendKeyEvent {
            relation: r.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for SendKeyEvent {
    fn disconnect(&self) {
        debug!("Disconnecting behaviour {} from property instance {}", SEND_KEY_EVENT, self.handle_id);
        let property = self.relation.inbound.properties.get(InputDeviceProperties::EVENT.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SendKeyEvent {
    fn drop(&mut self) {
        self.disconnect();
    }
}
