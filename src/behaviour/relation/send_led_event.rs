use evdev::EventType;
use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::json;

use crate::behaviour::entity::input_device_led_properties::InputDeviceLedProperties;
use crate::behaviour::entity::input_device_properties::InputDeviceProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use inexor_rgf_core_reactive::BehaviourCreationError;

pub const SEND_LED_EVENT: &'static str = "send_led_event";

pub struct SendLedEvent {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl SendLedEvent {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> Result<SendLedEvent, BehaviourCreationError> {
        let input_device_led = r.outbound.clone();
        let input_device = r.inbound.clone();
        let input_device_led_led_type = input_device_led.as_i64(InputDeviceLedProperties::LED_TYPE);
        if input_device_led_led_type.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let event_type = u64::try_from(EventType::LED.0).unwrap();
        let code = input_device_led_led_type.unwrap();
        let handle_id = input_device_led
            .properties
            .get(InputDeviceLedProperties::SET_STATE.as_ref())
            .unwrap()
            .id
            .as_u128();

        input_device_led
            .properties
            .get(InputDeviceLedProperties::SET_STATE.as_ref())
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

        Ok(SendLedEvent {
            relation: r.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for SendLedEvent {
    fn disconnect(&self) {
        debug!("Disconnecting behaviour {} from property instance {}", SEND_LED_EVENT, self.handle_id);
        let property = self.relation.inbound.properties.get(InputDeviceProperties::EVENT.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SendLedEvent {
    fn drop(&mut self) {
        self.disconnect();
    }
}
