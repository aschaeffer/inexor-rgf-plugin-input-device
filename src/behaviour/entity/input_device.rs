use std::convert::AsRef;
use std::sync::Arc;
use std::time::Duration;

use crate::reactive::BehaviourCreationError;
use async_std::task;
use log::{debug, error, info, trace};
use serde_json::{json, Error, Value};

use crate::behaviour::entity::InputDeviceProperties;
use crate::behaviour::event_payload::{
    INPUT_EVENT_KIND, INPUT_EVENT_KIND_KEY_EVENT, INPUT_EVENT_VALUE, KEY_EVENT_KEYCODE,
};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use evdev::{InputEvent, InputEventKind};
use futures::FutureExt;
use futures::{select, StreamExt};
use futures_timer::Delay;

const INPUT_DEVICE: &'static str = "input_device";

pub struct DeviceInput {
    pub entity: Arc<ReactiveEntityInstance>,

    stopper: crossbeam::channel::Sender<()>,
}

impl DeviceInput {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<DeviceInput, BehaviourCreationError> {
        let physical_path = e
            .properties
            .get(InputDeviceProperties::PHYSICAL_PATH.as_ref());
        if physical_path.is_none() {
            error!("Missing physical_path");
            return Err(BehaviourCreationError.into());
        }
        let physical_path = physical_path.unwrap().as_string().unwrap();

        let name = e
            .as_string(InputDeviceProperties::NAME)
            .unwrap_or("Unknown Device".into());
        trace!(
            "Initializing behaviour for input device {} with physical path {}",
            name,
            physical_path
        );

        let device =
            evdev::enumerate().find(|d| physical_path.as_str() == d.physical_path().unwrap_or(""));
        if device.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let device = device.unwrap();

        let (tx, rx) = crossbeam::channel::bounded(1);

        let entity_instance = e.clone();

        let thread_name = format!("{}-{}", e.type_name.clone(), e.id.to_string());
        let _handler = task::Builder::new().name(thread_name).spawn(async move {
            let event_stream = device.into_event_stream();
            if event_stream.is_err() {
                error!("{:?}", event_stream.err().unwrap());
                return;
            }

            let property_event = entity_instance
                .properties
                .get(InputDeviceProperties::EVENT.as_ref());
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
                        match rx.try_recv() {
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
                                            KEY_EVENT_KEYCODE: key.code(),
                                            INPUT_EVENT_VALUE: event.value()
                                        }))
                                    }
                                    _ => {}
                                }
                            }
                            Some(Err(e)) => {
                                // debug!("Error: {:?}\r", e);
                            },
                            None => break,
                        }
                    }
                };
            }
        });

        Ok(DeviceInput {
            entity: e.clone(),
            stopper: tx.clone(),
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for DeviceInput {
    fn disconnect(&self) {
        debug!("Stopping thread handling input device");
        self.stopper.send(());
    }
}

/// Automatically disconnect streams on destruction
impl Drop for DeviceInput {
    fn drop(&mut self) {
        self.disconnect();
    }
}
