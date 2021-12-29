use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, SwitchType};
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;

pub const INPUT_DEVICE_SWITCH: &'static str = "input_device_switch";

#[async_trait]
pub trait InputDeviceSwitchManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_switches(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_switch(&self, device: &Device, input_device_entity_instance: Arc<ReactiveEntityInstance>, switch: SwitchType);

    fn create_switch_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_switch: Arc<ReactiveEntityInstance>);
}
