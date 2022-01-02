use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, Key};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub const INPUT_DEVICE_KEY: &'static str = "input_device_key";

#[async_trait]
pub trait InputDeviceKeyManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_keys(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_input_device_key(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, key: Key);

    fn create_any_device_key(&self, input_device: Arc<ReactiveEntityInstance>, key: Key);

    fn create_entity_instance(&self, uuid: Uuid, unique_name: String, key_name: String, key: Key) -> EntityInstance;

    fn try_create_key_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_key: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
        create_send_key: bool,
    );

    fn create_key_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_key: Arc<ReactiveEntityInstance>, create_send_key: bool);
}
