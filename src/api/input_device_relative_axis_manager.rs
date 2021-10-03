use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, RelativeAxisType};
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;

pub const INPUT_DEVICE_RELATIVE_AXIS: &'static str = "input_device_relative_axis";

#[async_trait]
pub trait InputDeviceRelativeAxisManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_relative_axes(
        &self,
        device: &Device,
        entity_instance: Arc<ReactiveEntityInstance>,
    );

    fn create_input_device_relative_axis(
        &self,
        device: &Device,
        input_device_entity_instance: Arc<ReactiveEntityInstance>,
        relative_axis: RelativeAxisType,
    );

    fn create_relative_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_relative_axis: Arc<ReactiveEntityInstance>,
    );
}
