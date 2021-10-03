use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::json;

use crate::behaviour::entity::input_device_properties::InputDeviceProperties;
use crate::behaviour::entity::input_device_switch_properties::InputDeviceSwitchProperties;
use crate::behaviour::event_payload::{
    INPUT_EVENT_KIND, INPUT_EVENT_KIND_SWITCH_EVENT, INPUT_EVENT_VALUE, SWITCH_EVENT_SWITCH_TYPE,
};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use inexor_rgf_core_model::PropertyInstanceSetter;
use inexor_rgf_core_reactive::BehaviourCreationError;

pub const SWITCH_EVENT: &'static str = "switch_event";

pub struct SwitchEvent {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl SwitchEvent {
    pub fn new<'a>(
        r: Arc<ReactiveRelationInstance>,
    ) -> Result<SwitchEvent, BehaviourCreationError> {
        let input_device = r.outbound.clone();
        let input_device_switch = r.inbound.clone();
        let input_device_switch_switch_type =
            input_device_switch.as_i64(InputDeviceSwitchProperties::SWITCH_TYPE);
        if input_device_switch_switch_type.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let input_device_switch_switch_type = input_device_switch_switch_type.unwrap();

        let handle_id = input_device
            .properties
            .get(InputDeviceProperties::EVENT.as_ref())
            .unwrap()
            .id
            .as_u128();

        input_device
            .properties
            .get(InputDeviceProperties::EVENT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    let event = v.clone();
                    if !event.is_object() {
                        return;
                    }
                    let input_event_kind = event.get(INPUT_EVENT_KIND);
                    if input_event_kind.is_none() {
                        return;
                    }

                    match input_event_kind.unwrap().as_str().unwrap() {
                        INPUT_EVENT_KIND_SWITCH_EVENT => {
                            let event_switch_type = event
                                .get(SWITCH_EVENT_SWITCH_TYPE)
                                .unwrap()
                                .as_i64()
                                .unwrap_or(-1);
                            if input_device_switch_switch_type == event_switch_type {
                                let default = json!(0);
                                let value = event.get(INPUT_EVENT_VALUE).unwrap_or(&default);
                                input_device_switch.set(
                                    InputDeviceSwitchProperties::STATE.to_string(),
                                    value.clone(),
                                );
                            }
                        }
                        _ => {}
                    }
                },
                handle_id,
            );

        Ok(SwitchEvent {
            relation: r.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for SwitchEvent {
    fn disconnect(&self) {
        debug!(
            "Disconnecting behaviour {} from property instance {}",
            SWITCH_EVENT, self.handle_id
        );
        let property = self
            .relation
            .inbound
            .properties
            .get(InputDeviceProperties::EVENT.as_ref());
        if property.is_some() {
            property
                .unwrap()
                .stream
                .read()
                .unwrap()
                .remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SwitchEvent {
    fn drop(&mut self) {
        self.disconnect();
    }
}
