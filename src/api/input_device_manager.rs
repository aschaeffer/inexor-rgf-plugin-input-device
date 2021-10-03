use crate::config::InputDeviceConfig;
use async_trait::async_trait;
use evdev::Device;
use inexor_rgf_core_plugins::PluginContext;
use std::sync::Arc;
use uuid::Uuid;

pub static NAMESPACE_INPUT_DEVICE: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd530c7);

#[async_trait]
pub trait InputDeviceManager: Send + Sync {
    fn init(&self);

    fn set_context(&self, context: Arc<dyn PluginContext>);

    /// Load list of input devices from TOML
    fn load_config(&self);

    fn autodetect_input_devices(&self);

    fn load_input_devices(&self, input_devices: Vec<InputDeviceConfig>);

    /// Creates an reactive entity instance for the given input device.
    fn create_input_device(
        &self,
        device: &Device,
        autodetect_keys: bool,
        autodetect_leds: bool,
        autodetect_relative_axes: bool,
        autodetect_absolute_axes: bool,
        autodetect_switches: bool,
    );
}
