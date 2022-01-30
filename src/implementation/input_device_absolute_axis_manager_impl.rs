use async_trait::async_trait;
use log::{error, trace};
use waiter_di::*;

use crate::api::{InputDeviceAbsoluteAxisManager, INPUT_DEVICE_ABSOLUTE_AXIS, NAMESPACE_INPUT_DEVICE};
use crate::behaviour::entity::InputDeviceAbsoluteAxisProperties;
use crate::behaviour::relation::absolute_axis_event::ABSOLUTE_AXIS_EVENT;
use crate::builder::{EntityInstanceBuilder, RelationInstanceBuilder};
use crate::model::ReactiveEntityInstance;
use crate::plugins::PluginContext;
use evdev::{AbsoluteAxisType, Device};
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
pub struct InputDeviceAbsoluteAxisManagerImpl {
    context: PluginContextContainer,
}

impl InputDeviceAbsoluteAxisManagerImpl {}

#[async_trait]
#[provides]
impl InputDeviceAbsoluteAxisManager for InputDeviceAbsoluteAxisManagerImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn create_input_device_absolute_axes(&self, device: &Device, entity_instance: Arc<ReactiveEntityInstance>) {
        let supported_absolute_axes = device.supported_absolute_axes();
        match supported_absolute_axes {
            Some(supported_absolute_axes) => {
                for absolute_axis in supported_absolute_axes.iter() {
                    self.create_input_device_absolute_axis(device, entity_instance.clone(), absolute_axis);
                    self.create_any_device_absolute_axis(entity_instance.clone(), absolute_axis);
                }
            }
            None => {}
        }
    }

    fn create_input_device_absolute_axis(&self, device: &Device, input_device: Arc<ReactiveEntityInstance>, absolute_axis: AbsoluteAxisType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = device.name().unwrap_or("Unnamed Device");
        let physical_path = device.physical_path().unwrap_or("");
        let absolute_axis_name = format!("{:?}", absolute_axis);
        let unique_name = format!("{}-{}-{}", device_name, physical_path, absolute_axis_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        let input_device_absolute_axis = self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), absolute_axis_name.clone(), absolute_axis);
        let input_device_absolute_axis = entity_instance_manager.create(input_device_absolute_axis);
        self.try_create_absolute_axis_event(input_device, input_device_absolute_axis, unique_name);
    }

    fn create_any_device_absolute_axis(&self, input_device: Arc<ReactiveEntityInstance>, absolute_axis: AbsoluteAxisType) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let device_name = "any-device";
        let absolute_axis_name = format!("{:?}", absolute_axis);
        let unique_name = format!("{}-{}", device_name, absolute_axis_name);
        let uuid = Uuid::new_v5(&NAMESPACE_INPUT_DEVICE, unique_name.as_bytes());
        if !entity_instance_manager.has(uuid) {
            let input_device_absolute_axis =
                self.create_entity_instance(uuid, device_name.into(), unique_name.clone(), absolute_axis_name.clone(), absolute_axis);
            let input_device_absolute_axis = entity_instance_manager.create(input_device_absolute_axis);
            self.try_create_absolute_axis_event(input_device, input_device_absolute_axis, unique_name);
        } else {
            self.create_absolute_axis_event(input_device.clone(), entity_instance_manager.get(uuid).unwrap().clone());
        }
    }

    fn create_entity_instance(
        &self,
        uuid: Uuid,
        device_name: String,
        unique_name: String,
        absolute_axis_name: String,
        absolute_axis: AbsoluteAxisType,
    ) -> EntityInstance {
        EntityInstanceBuilder::new(INPUT_DEVICE_ABSOLUTE_AXIS)
            .id(uuid)
            .property(InputDeviceAbsoluteAxisProperties::NAME, json!(unique_name))
            .property(InputDeviceAbsoluteAxisProperties::LABEL, unique_label(device_name, absolute_axis_name.clone()))
            .property(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS, json!(absolute_axis_name))
            .property(InputDeviceAbsoluteAxisProperties::ABSOLUTE_AXIS_TYPE, json!(absolute_axis.0))
            .property(InputDeviceAbsoluteAxisProperties::STATE, InputDeviceAbsoluteAxisProperties::STATE.default_value())
            .get()
    }

    fn try_create_absolute_axis_event(
        &self,
        input_device: Arc<ReactiveEntityInstance>,
        input_device_absolute_axis: Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>,
        unique_name: String,
    ) {
        match input_device_absolute_axis {
            Ok(input_device_absolute_axis) => {
                trace!("Registered {} {} as {}", INPUT_DEVICE_ABSOLUTE_AXIS, unique_name, input_device_absolute_axis.id);
                self.create_absolute_axis_event(input_device.clone(), input_device_absolute_axis.clone());
            }
            Err(_) => {
                error!("Failed to create entity instance for {} {}!", INPUT_DEVICE_ABSOLUTE_AXIS, unique_name);
            }
        }
    }

    fn create_absolute_axis_event(&self, input_device: Arc<ReactiveEntityInstance>, input_device_absolute_axis: Arc<ReactiveEntityInstance>) {
        let reader = self.context.0.read().unwrap();
        let relation_instance_manager = reader.as_ref().unwrap().get_relation_instance_manager().clone();
        let absolute_axis_event = RelationInstanceBuilder::new(input_device.id, ABSOLUTE_AXIS_EVENT, input_device_absolute_axis.id).get();
        let _absolute_axis_event = relation_instance_manager.create(absolute_axis_event);
    }
}

fn unique_label(device_name: String, absolute_axis_name: String) -> Value {
    json!(format!(
        "/org/inexor/input/{}/absolute_axis/{}",
        device_name.clone().to_lowercase().replace("-", "_").replace(" ", "_"),
        absolute_axis_name.clone().to_lowercase().replace("-", "_").replace(" ", "_")
    ))
}
