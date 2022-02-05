use crate::di::*;
use async_trait::async_trait;
use log::{error, trace};

use crate::api::{InputDeviceRelativeAxisManager, INPUT_DEVICE_RELATIVE_AXIS, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceRelativeAxisProperties;
use crate::behaviour::relation::relative_axis_event::RELATIVE_AXIS_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{Device, RelativeAxisType};
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
pub struct InputDeviceRelativeAxisManagerImpl {
    context: PluginContextContainer,
}

impl InputDeviceRelativeAxisManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceRelativeAxisManager for InputDeviceRelativeAxisManagerImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn create_input_device_relative_axes(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>) {
        let supported_relative_axes = device.supported_relative_axes();
        match supported_relative_axes {
            Some(supported_relative_axes) => {
                for relative_axis in supported_relative_axes.iter() {
                    self.create_input_device_relative_axis(device, entity_instance.clone(), relative_axis);
                    self.create_any_device_relative_axis(entity_instance.clone(), relative_axis);
                }
            }
            None => {}
        }
    }

    fn create_input_device_relative_axis(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, relative_axis: RelativeAxisType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let relative_axis_name = format!("{:?}", relative_axis);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, relative_axis_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        let input_device_relative_axis = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), relative_axis_name.clone(), relative_axis);
        let input_device_relative_axis = entity_instance_manager.create(input_device_relative_axis);
        self.try_create_relative_axis_event(input_device, input_device_relative_axis, unique_name);
    }

    fn create_any_device_relative_axis(&self, input_device: Arc<ReactiveEntityInstance>, relative_axis: RelativeAxisType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = "any-device";
        let relative_axis_name = format!("{:?}", relative_axis);
        let unique_name = format!("{}-{}", device_name, relative_axis_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_relative_axis =
                self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), relative_axis_name.clone(), relative_axis);
            let input_device_relative_axis = entity_instance_manager.create(input_device_relative_axis);
            self.try_create_relative_axis_event(input_device, input_device_relative_axis, unique_name);
        } else {
            self.create_relative_axis_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone());
        }
    }

    fn create_entity_instance(
        &self,
        uuid: Uuid,
        device_name: String,
        unique_name: String,
        relative_axis_name: String,
        relative_axis: RelativeAxisType,
    ) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_RELATIVE_AXIS)
            .id(uuid)
            .property(InputDeviceRelativeAxisProperties::NAME, json!(unique_name))
            .property(InputDeviceRelativeAxisProperties::LABEL, unique_label(device_name, relative_axis_name.clone()))
            .property(InputDeviceRelativeAxisProperties::RELATIVE_AXIS, json!(relative_axis_name))
            .property(InputDeviceRelativeAxisProperties::RELATIVE_AXIS_TYPE, json!(relative_axis.0))
            .property(InputDeviceRelativeAxisProperties::STATE, InputDeviceRelativeAxisProperties::STATE.default_value())
            .get()
    }

    fn try_create_relative_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_relative_axis: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    ) {
        match input_device_relative_axis {
            Ok(input_device_relative_axis) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_RELATIVE_AXIS, unique_name, input_device_relative_axis.id);
                self.create_relative_axis_event(input_device.clone(), input_device_relative_axis.clone());
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_RELATIVE_AXIS, unique_name);
            }
        }
    }

    fn create_relative_axis_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_relative_axis: Arc<ReactiveEntityInstance>) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let relative_axis_event = RelationInstanceBuilder::new(input_device.id, RELATIVE_AXIS_EVENT, input_device_relative_axis.id).get();
        let _relative_axis_event = relation_instance_manager.create(relative_axis_event);
    }
}

fn unique_label(device_name: String, relative_axis_name: String) -> Value {
    json!(format!(
        "/org/inexor/input/{}/relative_axis/{}",
        device_name.clone().to_lowercase().replace("-", "_").replace(" ", "_"),
        relative_axis_name.clone().to_lowercase().replace("-", "_").replace(" ", "_")
    ))
}
