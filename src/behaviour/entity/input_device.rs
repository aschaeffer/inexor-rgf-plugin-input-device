use std::convert::AsRef;
use std::num::TryFromIntError;
use std::sync::Arc;
use std::time::Duration;

use crate::reactive::BehaviourCreationError;
use async_std::task;
use log::{error, trace};
use serde_json::{json, Value};

use crate::behaviour::entity::InputDeviceProperties;
use crate::behaviour::event_payload::{
    ABSOLUTE_AXIS_EVENT_ABSOLUTE_AXIS_TYPE, INPUT_EVENT_KIND, INPUT_EVENT_KIND_ABSOLUTE_AXIS_EVENT, INPUT_EVENT_KIND_KEY_EVENT, INPUT_EVENT_KIND_LED_EVENT,
    INPUT_EVENT_KIND_RELATIVE_AXIS_EVENT, INPUT_EVENT_KIND_SWITCH_EVENT, INPUT_EVENT_VALUE, KEY_EVENT_KEY_CODE, LED_EVENT_LED_TYPE,
    RELATIVE_AXIS_EVENT_RELATIVE_AXIS_TYPE, SWITCH_EVENT_SWITCH_TYPE,
};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use evdev::{EventType, InputEvent, InputEventKind};
use futures::FutureExt;
use futures::{select, StreamExt};
use futures_timer::Delay;

pub const INPUT_DEVICE: &'static str = "input_device";

pub struct InputDevice {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,

    stopper: crossbeam::channel::Sender<()>,
}

impl InputDevice {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<InputDevice, BehaviourCreationError> {
        let physical_path = e.properties.get(InputDeviceProperties::PHYSICAL_PATH.as_ref());
        if physical_path.is_none() {
            error!("Missing physical_path");
            return Err(BehaviourCreationError.into());
        }
        let physical_path = physical_path.unwrap().as_string().unwrap();

        let name = e.as_string(InputDeviceProperties::NAME).unwrap_or("Unknown Device".into());
        trace!("Initializing behaviour for input device {} with physical path {}", name, physical_path);

        let device = evdev::enumerate().find(|d| physical_path.as_str() == d.physical_path().unwrap_or(""));
        if device.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let mut device = device.unwrap();

        let handle_id = e.properties.get(InputDeviceProperties::SEND_EVENT.as_ref()).unwrap().id.as_u128();

        let physical_path_2 = physical_path.clone();
        e.properties
            .get(InputDeviceProperties::SEND_EVENT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |send_event: &Value| {
                    // event_type (u64)
                    let event_type = send_event.get("event_type");
                    if event_type.is_none() {
                        return;
                    }
                    let event_type = event_type.unwrap();
                    if !event_type.is_number() {
                        // Invalid
                        return;
                    }
                    let event_type = to_event_type(event_type.as_u64().unwrap());
                    if event_type.is_err() {
                        // Invalid
                        return;
                    }
                    let event_type = event_type.unwrap();

                    // code (i64 -> i16)
                    let code = send_event.get("code");
                    if code.is_none() {
                        return;
                    }
                    let code = code.unwrap();
                    if !code.is_number() {
                        // Invalid
                        return;
                    }
                    let code = u16::try_from(code.as_i64().unwrap()).unwrap();

                    // value (bool)
                    let value = send_event.get("value");
                    if value.is_none() {
                        return;
                    }
                    let value = value.unwrap();
                    if !value.is_boolean() {
                        // Invalid
                        return;
                    }
                    let value = if value.as_bool().unwrap() { i32::MAX } else { 0 };

                    let device = evdev::enumerate().find(|d| physical_path_2.as_str() == d.physical_path().unwrap_or(""));
                    if device.is_none() {
                        return;
                    }

                    let _result = device.unwrap().send_events(&[InputEvent::new(event_type, code, value)]);
                },
                handle_id,
            );

        let (stopper_tx, stopper_rx) = crossbeam::channel::bounded(1);

        let entity_instance = e.clone();

        let thread_name = format!("{}-{}", e.type_name.clone(), e.id.to_string());
        let _handler = task::Builder::new().name(thread_name).spawn(async move {
            let event_stream = device.into_event_stream();
            if event_stream.is_err() {
                error!("{:?}", event_stream.err().unwrap());
                return;
            }

            let property_event = entity_instance.properties.get(InputDeviceProperties::EVENT.as_ref());
            if property_event.is_none() {
                error!("Missing property event");
                return;
            }
            let property_event = property_event.unwrap();

            let mut event_stream = event_stream.unwrap();
            loop {
                let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
                let mut event = event_stream.next().fuse();

                select! {
                    _ = delay => {
                        match stopper_rx.try_recv() {
                            // Stop thread
                            Ok(_) => break,
                            // Continue thread
                            Err(_) => continue,
                        }
                    },
                    maybe_event = event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                match event.kind() {
                                    InputEventKind::Key(key) => {
                                        property_event.set(json!({
                                            INPUT_EVENT_KIND: INPUT_EVENT_KIND_KEY_EVENT,
                                            KEY_EVENT_KEY_CODE: key.code(),
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    InputEventKind::Led(led_type) => {
                                        property_event.set(json!({
                                            INPUT_EVENT_KIND: INPUT_EVENT_KIND_LED_EVENT,
                                            LED_EVENT_LED_TYPE: led_type.0,
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    InputEventKind::RelAxis(relative_axis_type) => {
                                        property_event.set(json!({
                                            INPUT_EVENT_KIND: INPUT_EVENT_KIND_RELATIVE_AXIS_EVENT,
                                            RELATIVE_AXIS_EVENT_RELATIVE_AXIS_TYPE: relative_axis_type.0,
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    InputEventKind::AbsAxis(absolute_axis_type) => {
                                        property_event.set(json!({
                                            INPUT_EVENT_KIND: INPUT_EVENT_KIND_ABSOLUTE_AXIS_EVENT,
                                            ABSOLUTE_AXIS_EVENT_ABSOLUTE_AXIS_TYPE: absolute_axis_type.0,
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    InputEventKind::Switch(switch_type) => {
                                        property_event.set(json!({
                                            INPUT_EVENT_KIND: INPUT_EVENT_KIND_SWITCH_EVENT,
                                            SWITCH_EVENT_SWITCH_TYPE: switch_type.0,
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    _ => {}
                                }
                            }
                            Some(Err(_)) => {
                                // debug!("Error: {:?}\r", e);
                            },
                            None => break,
                        }
                    }
                };
            }
        });

        Ok(InputDevice {
            entity: e.clone(),
            stopper: stopper_tx.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for InputDevice {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", INPUT_DEVICE, self.entity.id);
        let property = self.entity.properties.get(InputDeviceProperties::SEND_EVENT.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
        trace!("Stopping thread of {} with id {}", INPUT_DEVICE, self.entity.id);
        let _ = self.stopper.send(());
    }
}

/// Automatically disconnect streams on destruction
impl Drop for InputDevice {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn to_event_type(event_type: u64) -> Result<EventType, TryFromIntError> {
    let r_event_type = u16::try_from(event_type);
    if r_event_type.is_err() {
        return Err(r_event_type.err().unwrap());
    }
    Ok(EventType(r_event_type.unwrap()))
}
