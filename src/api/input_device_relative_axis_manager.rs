use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, RelativeAxisType};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub const INPUT_DEVICE_RELATIVE_AXIS: &'static str = "input_device_relative_axis";

#[async_trait]
pub trait InputDeviceRelativeAxisManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_relative_axes(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_relative_axis(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, relative_axis: RelativeAxisType);

    fn create_any_device_relative_axis(&self, input_device: Arc<ReactiveEntityInstance>, relative_axis: RelativeAxisType);

    fn create_entity_instance(
        &self,
        uuid: Uuid,
        device_name: String,
        unique_name: String,
        relative_axis_name: String,
        relative_axis: RelativeAxisType,
    ) -> EntityInstance;

    fn try_create_relative_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_relative_axis: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    );

    fn create_relative_axis_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_relative_axis: Arc<ReactiveEntityInstance>);
}
