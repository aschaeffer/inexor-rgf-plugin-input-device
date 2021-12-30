use async_trait::async_trait;
use log::{error, trace};
use waiter_di::*;

use crate::api::{InputDeviceKeyManager, INPUT_DEVICE_KEY, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceKeyProperties;
use crate::behaviour::relation::key_event::KEY_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, Key};
use inexor_rgf_core_model::EntityInstance;
use inexor_rgf_core_plugins::entity_instance_manager::EntityInstanceCreationError;
use serde_json::json;
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
        let input_device_key = self.create_entity_instance(uuid, unique_name.clone(), key_name.clone(), key);
        let input_device_key = entity_instance_manager.create(input_device_key);
        self.try_create_key_event(input_device, input_device_key, unique_name);
    }

    fn create_any_device_key(&self, input_device: Arc<ReactiveEntityInstance>, key: Key) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let key_name = format!("{:?}", key);
        let unique_name = format!("any-device-{}", key_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_key = self.create_entity_instance(uuid, unique_name.clone(), key_name.clone(), key);
            let input_device_key = entity_instance_manager.create(input_device_key);
            self.try_create_key_event(input_device, input_device_key, unique_name);
        } else {
            self.create_key_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone());
        }
    }

    fn create_entity_instance(&self, uuid: Uuid, unique_name: String, key_name: String, key: Key) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_KEY)
            .id(uuid)
            .property("name", json!(unique_name))
            .property(InputDeviceKeyProperties::KEY.as_ref(), json!(key_name))
            .property(InputDeviceKeyProperties::KEY_CODE.as_ref(), json!(key.code()))
            .property(InputDeviceKeyProperties::KEY_DOWN.as_ref(), json!(false))
            .get()
    }

    fn try_create_key_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_key: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    ) {
        match input_device_key {
            Ok(input_device_key) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_KEY, unique_name, input_device_key.id);
                self.create_key_event(input_device.clone(), input_device_key.clone());
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_KEY, unique_name);
            }
        }
    }

    fn create_key_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_key: Arc<ReactiveEntityInstance>) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let key_event = RelationInstanceBuilder::new(input_device.id, KEY_EVENT, input_device_key.id).get();
        let _key_event = relation_instance_manager.create(key_event);
    }
}
