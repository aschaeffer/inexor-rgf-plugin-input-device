use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{AbsoluteAxisType, Device};
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;

pub const INPUT_DEVICE_ABSOLUTE_AXIS: &'static str = "input_device_absolute_axis";

#[async_trait]
pub trait InputDeviceAbsoluteAxisManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_absolute_axes(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_absolute_axis(&self, device: &Device, input_device_entity_instance: Arc<ReactiveEntityInstance>, absolute_axis: AbsoluteAxisType);

    fn create_absolute_axis_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_absolute_axis: Arc<ReactiveEntityInstance>);
}
