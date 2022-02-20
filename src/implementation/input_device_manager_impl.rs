use crate::di::*;
use async_trait::async_trait;
use log::{debug, error, trace};

use crate::api::{
    InputDeviceAbsoluteAxisManager, InputDeviceKeyManager, InputDeviceLedManager, InputDeviceManager, InputDeviceRelativeAxisManager, InputDeviceSwitchManager,
    NAMESPACE_INPUT_DEVICE,
};
use crate::behaviour::entity::input_device::INPUT_DEVICE;
use crate::behaviour::entity::InputDeviceProperties;
use crate::builder::EntityInstanceBuilder;
use crate::config::{InputDeviceConfig, InputDevicesConfig};
use crate::plugins::PluginContext;
use evdev::Device;
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

const CONFIG_PATH: &str = "./config/input_devices.toml";

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[component]
pub struct InputDeviceManagerImpl {
    input_device_key_manager: Wrc<dyn InputDeviceKeyManager>,
    input_device_led_manager: Wrc<dyn InputDeviceLedManager>,
    input_device_relative_axis_manager: Wrc<dyn InputDeviceRelativeAxisManager>,
    input_device_absolute_axis_manager: Wrc<dyn InputDeviceAbsoluteAxisManager>,
    input_device_switch_manager: Wrc<dyn InputDeviceSwitchManager>,

    context: PluginContextContainer,
}

impl InputDeviceManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceManager for InputDeviceManagerImpl {
    fn init(&self) {
        self.load_config();
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
        self.input_device_key_manager.set_context(context.clone());
        self.input_device_led_manager.set_context(context.clone());
        self.input_device_relative_axis_manager.set_context(context.clone());
        self.input_device_absolute_axis_manager.set_context(context.clone());
        self.input_device_switch_manager.set_context(context.clone());
    }

    fn load_config(&self) {
        let toml_config = std::fs::read_to_string(CONFIG_PATH);
        match toml_config {
            Ok(toml_string) => {
                let input_devices_config: Result<InputDevicesConfig, _> = toml::from_str(&toml_string);
                match input_devices_config {
                    Ok(input_devices_config) => {
                        if input_devices_config.autodetect {
                            self.autodetect_input_devices();
                        } else {
                            self.load_input_devices(input_devices_config.input_device);
                        }
                    }
                    Err(_) => {
                        error!("Failed to load input devices configuration from {}: Invalid TOML:", CONFIG_PATH);
                    }
                }
            }
            Err(_) => {
                error!("Failed to load plugin configuration from {}", CONFIG_PATH);
            }
        }
    }

    fn autodetect_input_devices(&self) {
        let devices = evdev::enumerate().collect::<Vec<_>>();
        for device in devices.iter() {
            debug!("Automatically detected input device: {}", device.name().unwrap_or("Unnamed Device"));
            self.create_input_device(device, true, true, true, true, true);
        }
    }

    fn load_input_devices(&self, input_devices: Vec<InputDeviceConfig>) {
        for input_device in input_devices.iter() {
            if input_device.active {
                let device = Device::open(input_device.path.clone());
                match device {
                    Ok(device) => {
                        debug!("Loading input device {} from {}", device.name().unwrap_or("Unnamed Device"), input_device.path.clone());
                        self.create_input_device(
                            &device,
                            input_device.autodetect_keys,
                            input_device.autodetect_leds,
                            input_device.autodetect_relative_axes,
                            input_device.autodetect_absolute_axes,
                            input_device.autodetect_switches,
                        );
                    }
                    Err(_) => {
                        error!("Failed to load input device {}", input_device.path);
                    }
                }
            }
        }
    }
    fn create_input_device(
        &self,
        device: &Device,
        autodetect_keys: bool,
        autodetect_leds: bool,
        autodetect_relative_axes: bool,
        autodetect_absolute_axes: bool,
        autodetect_switches: bool,
    ) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let unique_name = format!("{}-{}", device_name, physical_path);
        let driver_version = format!("{}.{}.{}", device.driver_version().0, device.driver_version().1, device.driver_version().2);
        let vendor = device.input_id().vendor();
        let product = device.input_id().product();
        let version = device.input_id().version();
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let entity_instance = EntityInstanceBuilder::new(INPUT_DEVICE)
            .id(Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes()))
            .property(InputDeviceProperties::NAME, json!(device_name))
            .property(InputDeviceProperties::LABEL, unique_label(device_name.into()))
            .property(InputDeviceProperties::PHYSICAL_PATH, json!(physical_path))
            .property(InputDeviceProperties::DRIVER_VERSION, json!(driver_version))
            .property(InputDeviceProperties::VENDOR, json!(vendor))
            .property(InputDeviceProperties::PRODUCT, json!(product))
            .property(InputDeviceProperties::VERSION, json!(version))
            .property(InputDeviceProperties::EVENT, json!({}))
            .get();
        let reactive_entity_instance = entity_instance_manager.create(entity_instance);
        match reactive_entity_instance {
            Ok(reactive_entity_instance) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE, device_name, reactive_entity_instance.id);
                if autodetect_keys {
                    self.input_device_key_manager.create_input_device_keys(device, reactive_entity_instance.clone());
                }
                if autodetect_leds {
                    self.input_device_led_manager.create_input_device_leds(device, reactive_entity_instance.clone());
                }
                if autodetect_relative_axes {
                    self.input_device_relative_axis_manager
                        .create_input_device_relative_axes(device, reactive_entity_instance.clone());
                }
                if autodetect_absolute_axes {
                    self.input_device_absolute_axis_manager
                        .create_input_device_absolute_axes(device, reactive_entity_instance.clone());
                }
                if autodetect_switches {
                    self.input_device_switch_manager
                        .create_input_device_switches(device, reactive_entity_instance.clone());
                }
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE, device_name);
            }
        }
    }
}

fn unique_label(device_name: String) -> Value {
    json!(format!("/org/inexor/input/{}", device_name.clone().to_lowercase().replace("-", "_").replace(" ", "_")))
}
