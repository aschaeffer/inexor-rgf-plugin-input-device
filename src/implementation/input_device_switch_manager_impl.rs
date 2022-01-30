use async_trait::async_trait;
use log::{error, trace};
use waiter_di::*;

use crate::api::{InputDeviceSwitchManager, INPUT_DEVICE_SWITCH, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceSwitchProperties;
use crate::behaviour::relation::switch_event::SWITCH_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, SwitchType};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[component]
pub struct InputDeviceSwitchManagerImpl {
    context: PluginContextContainer,
}

impl InputDeviceSwitchManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceSwitchManager for InputDeviceSwitchManagerImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn create_input_device_switches(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>) {
        let supported_switches = device.supported_switches();
        match supported_switches {
            Some(supported_switches) => {
                for switch in supported_switches.iter() {
                    self.create_input_device_switch(device, entity_instance.clone(), switch);
                }
            }
            None => {}
        }
    }

    fn create_input_device_switch(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, switch: SwitchType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let switch_name = format!("{:?}", switch);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, switch_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        let input_device_switch = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), switch_name.clone(), switch);
        let input_device_switch = entity_instance_manager.create(input_device_switch);
        self.try_create_switch_event(input_device, input_device_switch, unique_name);
    }

    fn create_any_device_switch(&self, input_device: Arc<ReactiveEntityInstance>, switch: SwitchType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = "any-device";
        let switch_name = format!("{:?}", switch);
        let unique_name = format!("{}-{}", device_name, switch_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_switch = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), switch_name.clone(), switch);
            let input_device_switch = entity_instance_manager.create(input_device_switch);
            self.try_create_switch_event(input_device, input_device_switch, unique_name);
        } else {
            self.create_switch_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone());
        }
    }

    fn create_entity_instance(&self, uuid: Uuid, device_name: String, unique_name: String, switch_name: String, switch: SwitchType) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_SWITCH)
            .id(uuid)
            .property(InputDeviceSwitchProperties::NAME, json!(unique_name))
            .property(InputDeviceSwitchProperties::LABEL, unique_label(device_name, switch_name.clone()))
            .property(InputDeviceSwitchProperties::SWITCH, json!(switch_name))
            .property(InputDeviceSwitchProperties::SWITCH_TYPE, json!(switch.0))
            .property(InputDeviceSwitchProperties::STATE, InputDeviceSwitchProperties::STATE.default_value())
            .get()
    }

    fn try_create_switch_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_switch: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    ) {
        match input_device_switch {
            Ok(input_device_switch) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_SWITCH, unique_name, input_device_switch.id);
                self.create_switch_event(input_device.clone(), input_device_switch.clone());
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_SWITCH, unique_name);
            }
        }
    }

    fn create_switch_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_switch: Arc<ReactiveEntityInstance>) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let switch_event = RelationInstanceBuilder::new(input_device.id, SWITCH_EVENT, input_device_switch.id).get();
        let _switch_event = relation_instance_manager.create(switch_event);
    }
}

fn unique_label(device_name: String, switch_name: String) -> Value {
    json!(format!(
        "/org/inexor/input/{}/switch/{}",
        device_name.clone().to_lowercase().replace("-", "_").replace(" ", "_"),
        switch_name.clone().to_lowercase().replace("-", "_").replace(" ", "_")
    ))
}
