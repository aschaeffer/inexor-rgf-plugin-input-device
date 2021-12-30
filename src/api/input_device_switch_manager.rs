use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, SwitchType};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub const INPUT_DEVICE_SWITCH: &'static str = "input_device_switch";

#[async_trait]
pub trait InputDeviceSwitchManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_switches(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_switch(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, switch: SwitchType);

    fn create_any_device_switch(&self, input_device: Arc<ReactiveEntityInstance>, switch: SwitchType);

    fn create_entity_instance(&self, uuid: Uuid, unique_name: String, switch_name: String, switch: SwitchType) -> EntityInstance;

    fn try_create_switch_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_switch: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    );

    fn create_switch_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_switch: Arc<ReactiveEntityInstance>);
}
