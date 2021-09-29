use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::{debug, error, trace};
use waiter_di::*;

use crate::behaviour::entity::entity_behaviour_provider::InputDeviceEntityBehaviourProviderImpl;
use crate::behaviour::entity::InputDeviceRelativeAxisProperties;
use crate::behaviour::relation::relation_behaviour_provider::InputDeviceRelationBehaviourProviderImpl;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider,
    FlowProvider, Plugin, PluginError, RelationBehaviourProvider, RelationTypeProvider,
    WebResourceProvider,
};
use crate::provider::{
    InputDeviceEntityTypeProviderImpl, InputDeviceFlowProviderImpl,
    InputDeviceRelationTypeProviderImpl,
};
use evdev::{Device, Key, LedType, RelativeAxisType};
use serde_json::json;
use std::env;
use uuid::Uuid;

pub static NAMESPACE_INPUT_DEVICE: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd530c7);

const INPUT_DEVICE: &'static str = "input_device";

const INPUT_DEVICE_KEY: &'static str = "input_device_key";

const INPUT_DEVICE_LED: &'static str = "input_device_led";

const INPUT_DEVICE_RELATIVE_AXIS: &'static str = "input_device_relative_axis";

const KEY_EVENT: &'static str = "key_event";

const LED_EVENT: &'static str = "led_event";

const RELATIVE_AXIS_EVENT: &'static str = "relative_axis_event";

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[async_trait]
pub trait InputDevicePlugin: Plugin + Send + Sync {}

#[module]
pub struct InputDevicePluginImpl {
    entity_type_provider: Wrc<InputDeviceEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<InputDeviceEntityBehaviourProviderImpl>,
    flow_provider: Wrc<InputDeviceFlowProviderImpl>,
    relation_type_provider: Wrc<InputDeviceRelationTypeProviderImpl>,
    relation_behaviour_provider: Wrc<InputDeviceRelationBehaviourProviderImpl>,

    context: PluginContextContainer,
}

impl InputDevicePluginImpl {
    fn create_input_devices(&self) {
        let devices = evdev::enumerate().collect::<Vec<_>>();
        for device in devices.iter() {
            debug!(
                "Detected input device: {}",
                device.name().unwrap_or("Unnamed Device")
            );
            self.create_input_device(device);
        }
    }

    fn create_input_device(&self, device: &Device) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let unique_name = format!("{}-{}", device_name, physical_path);
        let driver_version = format!(
            "{}.{}.{}",
            device.driver_version().0,
            device.driver_version().1,
            device.driver_version().2
        );
        let vendor = device.input_id().vendor();
        let product = device.input_id().product();
        let version = device.input_id().version();
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        let entity_instance = EntityInstanceBuilder::new(INPUT_DEVICE)
            .id(Uuid::new_v5(
                &NAMESPACE_INPUT_DEVICE,
                unique_name.as_bytes(),
            ))
            .property("name", json!(device_name))
            .property("physical_path", json!(physical_path))
            .property("driver_version", json!(driver_version))
            .property("vendor", json!(vendor))
            .property("product", json!(product))
            .property("version", json!(version))
            .property("event", json!({}))
            .get();
        let reactive_entity_instance = entity_instance_manager.create(entity_instance);
        match reactive_entity_instance {
            Ok(reactive_entity_instance) => {
                trace!(
                    "Registered {} {} as {}",
                    INPUT_DEVICE,
                    device_name,
                    reactive_entity_instance.id
                );
                self.create_input_device_keys(device, reactive_entity_instance.clone());
                self.create_input_device_leds(device, reactive_entity_instance.clone());
                self.create_input_device_relative_axes(device, reactive_entity_instance.clone());
                // TODO: create_device_switches
            }
            Err(_) => {
                error!(
                    "Failed to create entity instance for keyboard {}!",
                    device_name
                );
            }
        }
    }

    fn create_input_device_keys(
        &self,
        device: &Device,
        entity_instance: Arc<ReactiveEntityInstance>,
    ) {
        let supported_keys = device.supported_keys();
        match supported_keys {
            Some(supported_keys) => {
                for key in supported_keys.iter() {
                    self.create_input_device_key(device, entity_instance.clone(), key);
                }
            }
            None => {}
        }
    }

    fn create_input_device_key(
        &self,
        device: &Device,
        input_device_entity_instance: Arc<ReactiveEntityInstance>,
        key: Key,
    ) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let key_name = format!("{:?}", key);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, key_name);
        let reader = self.context.0.read().unwrap();
        let input_device_key = EntityInstanceBuilder::new(INPUT_DEVICE_KEY)
            .id(Uuid::new_v5(
                &NAMESPACE_INPUT_DEVICE,
                unique_name.as_bytes(),
            ))
            .property("name", json!(unique_name))
            .property("key", json!(key_name))
            .property("key_code", json!(key.code()))
            .property("key_down", json!(false))
            .get();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        let input_device_key = entity_instance_manager.create(input_device_key);
        match input_device_key {
            Ok(input_device_key) => {
                trace!(
                    "Registered {} {} as {}",
                    INPUT_DEVICE_KEY,
                    unique_name,
                    input_device_key.id
                );
                self.create_key_event(
                    input_device_entity_instance.clone(),
                    input_device_key.clone(),
                );
            }
            Err(_) => {
                error!(
                    "Failed to create entity instance for input device key {}!",
                    unique_name
                );
            }
        }
    }

    fn create_key_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_key: Arc<ReactiveEntityInstance>,
    ) {
        let key_event =
            RelationInstanceBuilder::new(input_device.id, KEY_EVENT, input_device_key.id).get();
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_relation_instance_manager()
            .clone();
        let _key_event = relation_instance_manager.create(key_event);
    }

    fn create_input_device_leds(
        &self,
        device: &Device,
        entity_instance: Arc<ReactiveEntityInstance>,
    ) {
        let supported_leds = device.supported_leds();
        match supported_leds {
            Some(supported_leds) => {
                for led in supported_leds.iter() {
                    self.create_input_device_led(device, entity_instance.clone(), led);
                }
            }
            None => {}
        }
    }

    fn create_input_device_led(
        &self,
        device: &Device,
        input_device_entity_instance: Arc<ReactiveEntityInstance>,
        led: LedType,
    ) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let led_name = format!("{:?}", led);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, led_name);
        let reader = self.context.0.read().unwrap();
        let input_device_led = EntityInstanceBuilder::new(INPUT_DEVICE_LED)
            .id(Uuid::new_v5(
                &NAMESPACE_INPUT_DEVICE,
                unique_name.as_bytes(),
            ))
            .property("name", json!(unique_name))
            .property("led", json!(led_name))
            .property("led_type", json!(led.0))
            .property("state", json!(false))
            .get();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        let input_device_led = entity_instance_manager.create(input_device_led);
        match input_device_led {
            Ok(input_device_led) => {
                trace!(
                    "Registered {} {} as {}",
                    INPUT_DEVICE_KEY,
                    unique_name,
                    input_device_led.id
                );
                self.create_led_event(
                    input_device_entity_instance.clone(),
                    input_device_led.clone(),
                );
            }
            Err(_) => {
                error!(
                    "Failed to create entity instance for input device LED {}!",
                    unique_name
                );
            }
        }
    }

    fn create_led_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_led: Arc<ReactiveEntityInstance>,
    ) {
        let led_event =
            RelationInstanceBuilder::new(input_device.id, LED_EVENT, input_device_led.id).get();
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_relation_instance_manager()
            .clone();
        let _led_event = relation_instance_manager.create(led_event);
    }

    fn create_input_device_relative_axes(
        &self,
        device: &Device,
        entity_instance: Arc<ReactiveEntityInstance>,
    ) {
        let supported_relative_axes = device.supported_relative_axes();
        match supported_relative_axes {
            Some(supported_relative_axes) => {
                for relative_axis in supported_relative_axes.iter() {
                    self.create_input_device_relative_axis(
                        device,
                        entity_instance.clone(),
                        relative_axis,
                    );
                }
            }
            None => {}
        }
    }

    fn create_input_device_relative_axis(
        &self,
        device: &Device,
        input_device_entity_instance: Arc<ReactiveEntityInstance>,
        relative_axis: RelativeAxisType,
    ) {
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let relative_axis_name = format!("{:?}", relative_axis);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, relative_axis_name);
        let reader = self.context.0.read().unwrap();
        let input_device_relative_axis = EntityInstanceBuilder::new(INPUT_DEVICE_RELATIVE_AXIS)
            .id(Uuid::new_v5(
                &NAMESPACE_INPUT_DEVICE,
                unique_name.as_bytes(),
            ))
            .property("name", json!(unique_name))
            .property("relative_axis", json!(relative_axis_name))
            .property(
                InputDeviceRelativeAxisProperties::RELATIVE_AXIS_TYPE.as_ref(),
                json!(relative_axis.0),
            )
            .property(
                InputDeviceRelativeAxisProperties::STATE.as_ref(),
                InputDeviceRelativeAxisProperties::STATE.default_value(),
            )
            .get();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        let input_device_relative_axis = entity_instance_manager.create(input_device_relative_axis);
        match input_device_relative_axis {
            Ok(input_device_relative_axis) => {
                trace!(
                    "Registered {} {} as {}",
                    INPUT_DEVICE_RELATIVE_AXIS,
                    unique_name,
                    input_device_relative_axis.id
                );
                self.create_relative_axis_event(
                    input_device_entity_instance.clone(),
                    input_device_relative_axis.clone(),
                );
            }
            Err(_) => {
                error!(
                    "Failed to create entity instance for input device relative axis {}!",
                    unique_name
                );
            }
        }
    }

    fn create_relative_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_relative_axis: Arc<ReactiveEntityInstance>,
    ) {
        let relative_axis_event = RelationInstanceBuilder::new(
            input_device.id,
            RELATIVE_AXIS_EVENT,
            input_device_relative_axis.id,
        )
        .get();
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_relation_instance_manager()
            .clone();
        let _relative_axis_event = relation_instance_manager.create(relative_axis_event);
    }
}

impl InputDevicePluginImpl {}

interfaces!(InputDevicePluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl InputDevicePlugin for InputDevicePluginImpl {}

impl Plugin for InputDevicePluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("InputDevicePluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("InputDevicePluginModuleImpl::post_init()");
        self.create_input_devices();
        // self.create_virtual_keyboard();
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("InputDevicePluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("InputDevicePluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        Err(PluginError::NoComponentProvider)
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        let relation_type_provider = self.relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(PluginError::NoRelationTypeProvider);
        }
        Ok(relation_type_provider.unwrap())
    }

    fn get_component_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        Err(PluginError::NoComponentBehaviourProvider)
    }

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        let entity_behaviour_provider = self.entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(PluginError::NoEntityBehaviourProvider);
        }
        Ok(entity_behaviour_provider.unwrap())
    }

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        let relation_behaviour_provider = self.relation_behaviour_provider.clone();
        let relation_behaviour_provider: Result<Arc<dyn RelationBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_behaviour_provider);
        if relation_behaviour_provider.is_err() {
            return Err(PluginError::NoRelationBehaviourProvider);
        }
        Ok(relation_behaviour_provider.unwrap())
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        let flow_provider = self.flow_provider.clone();
        let flow_provider: Result<Arc<dyn FlowProvider>, _> =
            <dyn query_interface::Object>::query_arc(flow_provider);
        if flow_provider.is_err() {
            return Err(PluginError::NoFlowProvider);
        }
        Ok(flow_provider.unwrap())
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}
