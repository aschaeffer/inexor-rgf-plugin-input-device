use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::input_device::InputDevice;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

const INPUT_DEVICE: &'static str = "input_device";

#[wrapper]
pub struct InputDeviceStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<InputDevice>>>);

#[waiter_di::provides]
fn create_input_device_storage() -> InputDeviceStorage {
    InputDeviceStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait InputDeviceEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_input_device(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_input_device(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct InputDeviceEntityBehaviourProviderImpl {
    input_device: InputDeviceStorage,
}

interfaces!(InputDeviceEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl InputDeviceEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            input_device: create_input_device_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl InputDeviceEntityBehaviourProvider for InputDeviceEntityBehaviourProviderImpl {
    fn create_input_device(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let device_key = InputDevice::new(entity_instance);
        if device_key.is_ok() {
            let input_device = Arc::new(device_key.unwrap());
            self.input_device.0.write().unwrap().insert(id, input_device);
            debug!("Added behaviour {} to entity instance {}", INPUT_DEVICE, id);
        }
    }

    fn remove_input_device(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.input_device.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", INPUT_DEVICE, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.input_device.0.write().unwrap().contains_key(&id) {
            self.input_device.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", INPUT_DEVICE, id);
        }
    }
}

impl EntityBehaviourProvider for InputDeviceEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            INPUT_DEVICE => self.create_input_device(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            INPUT_DEVICE => self.remove_input_device(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
