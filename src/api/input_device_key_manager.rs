use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use evdev::{Device, Key};
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;

pub const INPUT_DEVICE_KEY: &'static str = "input_device_key";

#[async_trait]
pub trait InputDeviceKeyManager: Send + Sync {
    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn create_input_device_keys(
        &self,
        device: &Device,
        entity_instance: Arc<ReactiveEntityInstance>,
    );

    fn create_input_device_key(
        &self,
        device: &Device,
        input_device_entity_instance: Arc<ReactiveEntityInstance>,
        key: Key,
    );

    fn create_key_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_key: Arc<ReactiveEntityInstance>,
    );
}
