use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::json;

use crate::behaviour::entity::input_device_led_properties::InputDeviceLedProperties;
use crate::behaviour::entity::input_device_properties::InputDeviceProperties;
use crate::behaviour::event_payload::{
    INPUT_EVENT_KIND, INPUT_EVENT_KIND_LED_EVENT, INPUT_EVENT_VALUE, LED_EVENT_LED_TYPE,
};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use inexor_rgf_core_model::PropertyInstanceSetter;
use inexor_rgf_core_reactive::BehaviourCreationError;

const LED_EVENT: &'static str = "led_event";

pub struct LedEvent {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl LedEvent {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> Result<LedEvent, BehaviourCreationError> {
        let input_device = r.outbound.clone();
        let input_device_led = r.inbound.clone();
        let input_device_led_led_type = input_device_led.as_i64(InputDeviceLedProperties::LED_TYPE);
        if input_device_led_led_type.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let input_device_led_led_type = input_device_led_led_type.unwrap();

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
                    debug!("LED EVENT {}", v.clone());
                    let event = v.clone();
                    if !event.is_object() {
                        return;
                    }
                    let input_event_kind = event.get(INPUT_EVENT_KIND);
                    if input_event_kind.is_none() {
                        return;
                    }

                    match input_event_kind.unwrap().as_str().unwrap() {
                        INPUT_EVENT_KIND_LED_EVENT => {
                            let event_led_type = event
                                .get(LED_EVENT_LED_TYPE)
                                .unwrap()
                                .as_i64()
                                .unwrap_or(-1);
                            if input_device_led_led_type == event_led_type {
                                let old_value = input_device_led
                                    .get(InputDeviceLedProperties::STATE)
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
                                    // LED off
                                    0 => {
                                        if old_value {
                                            input_device_led.set(
                                                InputDeviceLedProperties::STATE.to_string(),
                                                json!(false),
                                            )
                                        }
                                    }
                                    // LED on
                                    1 => {
                                        if !old_value {
                                            input_device_led.set(
                                                InputDeviceLedProperties::STATE.to_string(),
                                                json!(true),
                                            );
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

        Ok(LedEvent {
            relation: r.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for LedEvent {
    fn disconnect(&self) {
        debug!(
            "Disconnecting behaviour {} from property instance {}",
            LED_EVENT, self.handle_id
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
impl Drop for LedEvent {
    fn drop(&mut self) {
        self.disconnect();
    }
}
