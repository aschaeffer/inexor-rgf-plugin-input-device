use async_trait::async_trait;
use log::{error, trace};
use waiter_di::*;

use crate::api::{InputDeviceLedManager, INPUT_DEVICE_LED, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceLedProperties;
use crate::behaviour::relation::led_event::LED_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, LedType};
use serde_json::json;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[component]
pub struct InputDeviceLedManagerImpl {
    context: PluginContextContainer,
}

impl InputDeviceLedManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceLedManager for InputDeviceLedManagerImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn create_input_device_leds(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>) {
        let supported_leds = device.supported_leds();
        match supported_leds {
            Some(supported_leds) => {
                for led in supported_leds.iter() {
                    self.create_input_device_led(device, entity_instance.clone(), led);
                }
            }
            None => {}
        }
    }

    fn create_input_device_led(&self, device: &Device, input_device_entity_instance: Arc<ReactiveEntityInstance>, led: LedType) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let led_name = format!("{:?}", led);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, led_name);
        let reader = self.context.0.read().unwrap();
        let input_device_led = EntityInstanceBuilder::new(INPUT_DEVICE_LED)
            .id(Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes()))
            .property("name", json!(unique_name))
            .property(InputDeviceLedProperties::LED.as_ref(), json!(led_name))
            .property(InputDeviceLedProperties::LED_TYPE.as_ref(), json!(led.0))
            .property(InputDeviceLedProperties::STATE.as_ref(), json!(false))
            .get();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let input_device_led = entity_instance_manager.create(input_device_led);
        match input_device_led {
            Ok(input_device_led) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_LED, unique_name, input_device_led.id);
                self.create_led_event(input_device_entity_instance.clone(), input_device_led.clone());
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_LED, unique_name);
            }
        }
    }

    fn create_led_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_led: Arc<ReactiveEntityInstance>) {
        let led_event = RelationInstanceBuilder::new(input_device.id, LED_EVENT, input_device_led.id).get();
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let _led_event = relation_instance_manager.create(led_event);
    }
}
