use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{AbsoluteAxisType, Device};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub const INPUT_DEVICE_ABSOLUTE_AXIS: &'static str = "input_device_absolute_axis";

#[async_trait]
pub trait InputDeviceAbsoluteAxisManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_absolute_axes(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_absolute_axis(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, absolute_axis: AbsoluteAxisType);

    fn create_any_device_absolute_axis(&self, input_device: Arc<ReactiveEntityInstance>, absolute_axis: AbsoluteAxisType);

    fn create_entity_instance(
        &self,
        uuid: Uuid,
        device_name: String,
        unique_name: String,
        absolute_axis_name: String,
        absolute_axis: AbsoluteAxisType,
    ) -> EntityInstance;

    fn try_create_absolute_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_absolute_axis: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    );

    fn create_absolute_axis_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_absolute_axis: Arc<ReactiveEntityInstance>);
}
