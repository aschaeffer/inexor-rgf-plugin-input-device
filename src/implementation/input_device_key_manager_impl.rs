use crate::di::*;
use async_trait::async_trait;
use log::{error, trace};

use crate::api::{InputDeviceKeyManager, INPUT_DEVICE_KEY, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceKeyProperties;
use crate::behaviour::relation::key_event::KEY_EVENT;
use crate::behaviour::relation::send_key_event::SEND_KEY_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, Key};
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
pub struct InputDeviceKeyManagerImpl {
    context: PluginContextContainer,
}

impl InputDeviceKeyManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceKeyManager for InputDeviceKeyManagerImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn create_input_device_keys(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>) {
        let supported_keys = device.supported_keys();
        match supported_keys {
            Some(supported_keys) => {
                for key in supported_keys.iter() {
                    self.create_input_device_key(device, entity_instance.clone(), key);
                    self.create_any_device_key(entity_instance.clone(), key);
                }
            }
            None => {}
        }
    }

    fn create_input_device_key(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, key: Key) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let key_name = format!("{:?}", key);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, key_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        let input_device_key = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), key_name.clone(), key);
        let input_device_key = entity_instance_manager.create(input_device_key);
        self.try_create_key_event(input_device, input_device_key, unique_name, true);
    }

    fn create_any_device_key(&self, input_device: Arc<ReactiveEntityInstance>, key: Key) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = "any-device";
        let key_name = format!("{:?}", key);
        let unique_name = format!("{}-{}", device_name, key_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_key = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), key_name.clone(), key);
            let input_device_key = entity_instance_manager.create(input_device_key);
            self.try_create_key_event(input_device, input_device_key, unique_name, false);
        } else {
            self.create_key_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone(), false);
        }
    }

    fn create_entity_instance(&self, uuid: Uuid, device_name: String, unique_name: String, key_name: String, key: Key) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_KEY)
            .id(uuid)
            .property(InputDeviceKeyProperties::NAME, json!(unique_name.clone()))
            .property(InputDeviceKeyProperties::LABEL, unique_label(device_name, key_name.clone()))
            .property(InputDeviceKeyProperties::KEY, json!(key_name))
            .property(InputDeviceKeyProperties::KEY_CODE, json!(key.code()))
            .property(InputDeviceKeyProperties::KEY_DOWN, json!(false))
            .get()
    }

    fn try_create_key_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_key: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
        create_send_key: bool,
    ) {
        match input_device_key {
            Ok(input_device_key) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_KEY, unique_name, input_device_key.id);
                self.create_key_event(input_device.clone(), input_device_key.clone(), create_send_key);
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_KEY, unique_name);
            }
        }
    }

    fn create_key_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_key: Arc<ReactiveEntityInstance>, create_send_key: bool) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let key_event = RelationInstanceBuilder::new(input_device.id, KEY_EVENT, input_device_key.id).get();
        let _key_event = relation_instance_manager.create(key_event);
        if create_send_key {
            let send_key_event = RelationInstanceBuilder::new(input_device_key.id, SEND_KEY_EVENT, input_device.id).get();
            let _send_key_event = relation_instance_manager.create(send_key_event);
        }
    }
}

fn unique_label(device_name: String, key_name: String) -> Value {
    json!(format!(
        "/org/inexor/input/{}/key/{}",
        device_name.clone().to_lowercase().replace("-", "_").replace(" ", "_"),
        key_name.clone().to_lowercase().replace("-", "_").replace(" ", "_")
    ))
}
