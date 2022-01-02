use async_trait::async_trait;
use log::{error, trace};
use waiter_di::*;

use crate::api::{InputDeviceLedManager, INPUT_DEVICE_LED, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceLedProperties;
use crate::behaviour::relation::led_event::LED_EVENT;
use crate::behaviour::relation::send_led_event::SEND_LED_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, LedType};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
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
                    self.create_any_device_led(entity_instance.clone(), led);
                }
            }
            None => {}
        }
    }

    fn create_input_device_led(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, led: LedType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let led_name = format!("{:?}", led);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, led_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        let input_device_led = self.create_entity_instance(uuid, unique_name.clone(), led_name.clone(), led);
        let input_device_led = entity_instance_manager.create(input_device_led);
        self.try_create_led_event(input_device, input_device_led, unique_name, true);
    }

    fn create_any_device_led(&self, input_device: Arc<ReactiveEntityInstance>, led: LedType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let led_name = format!("{:?}", led);
        let unique_name = format!("any-device-{}", led_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_led = self.create_entity_instance(uuid, unique_name.clone(), led_name.clone(), led);
            let input_device_led = entity_instance_manager.create(input_device_led);
            self.try_create_led_event(input_device, input_device_led, unique_name, false);
        } else {
            self.create_led_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone(), false);
        }
    }

    fn create_entity_instance(&self, uuid: Uuid, unique_name: String, led_name: String, led: LedType) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_LED)
            .id(uuid)
            .property("name", json!(unique_name))
            .property(InputDeviceLedProperties::LED.as_ref(), json!(led_name))
            .property(InputDeviceLedProperties::LED_TYPE.as_ref(), json!(led.0))
            .property(InputDeviceLedProperties::STATE.as_ref(), json!(false))
            .get()
    }

    fn try_create_led_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_led: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
        create_send_led: bool,
    ) {
        match input_device_led {
            Ok(input_device_led) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_LED, unique_name, input_device_led.id);
                self.create_led_event(input_device.clone(), input_device_led.clone(), create_send_led);
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_LED, unique_name);
            }
        }
    }

    fn create_led_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_led: Arc<ReactiveEntityInstance>, create_send_led: bool) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let led_event = RelationInstanceBuilder::new(input_device.id, LED_EVENT, input_device_led.id).get();
        let _led_event = relation_instance_manager.create(led_event);
        if create_send_led {
            let send_led_event = RelationInstanceBuilder::new(input_device_led.id, SEND_LED_EVENT, input_device.id).get();
            let _send_led_event = relation_instance_manager.create(send_led_event);
        }
    }
}
