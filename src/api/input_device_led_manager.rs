use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, LedType};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub const INPUT_DEVICE_LED: &'static str = "input_device_led";

#[async_trait]
pub trait InputDeviceLedManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_leds(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_led(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, led: LedType);

    fn create_any_device_led(&self, input_device: Arc<ReactiveEntityInstance>, led: LedType);

    fn create_entity_instance(&self, uuid: Uuid, device_name: String, unique_name: String, key_name: String, led: LedType) -> EntityInstance;

    fn try_create_led_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_led: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
        create_send_led: bool,
    );

    fn create_led_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_led: Arc<ReactiveEntityInstance>, create_send_led: bool);
}
