use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, LedType};
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;

pub const INPUT_DEVICE_LED: &'static str = "input_device_led";

#[async_trait]
pub trait InputDeviceLedManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_leds(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_led(&self, device: &Device, input_device_entity_instance: Arc<ReactiveEntityInstance>, led: LedType);

    fn create_led_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_led: Arc<ReactiveEntityInstance>);
}
