use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use indradb::EdgeKey;
use log::trace;

use crate::behaviour::relation::send_key_event::SendKeyEvent;
use crate::behaviour::relation::send_led_event::SendLedEvent;
use crate::behaviour::relation::switch_event::SwitchEvent;
use crate::behaviour::relation::{absolute_axis_event::AbsoluteAxisEvent, key_event::KeyEvent, led_event::LedEvent, relative_axis_event::RelativeAxisEvent};
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

const KEY_EVENT: &'static str = "key_event";

const SEND_KEY_EVENT: &'static str = "send_key_event";

const LED_EVENT: &'static str = "led_event";

const SEND_LED_EVENT: &'static str = "send_led_event";

const RELATIVE_AXIS_EVENT: &'static str = "relative_axis_event";

const ABSOLUTE_AXIS_EVENT: &'static str = "absolute_axis_event";

const SWITCH_EVENT: &'static str = "switch_event";

#[wrapper]
pub struct KeyEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<KeyEvent>>>);

#[wrapper]
pub struct SendKeyEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<SendKeyEvent>>>);

#[wrapper]
pub struct LedEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<LedEvent>>>);

#[wrapper]
pub struct SendLedEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<SendLedEvent>>>);

#[wrapper]
pub struct RelativeAxisEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<RelativeAxisEvent>>>);

#[wrapper]
pub struct AbsoluteAxisEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<AbsoluteAxisEvent>>>);

#[wrapper]
pub struct SwitchEventRelationBehaviourStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<SwitchEvent>>>);

#[provides]
fn create_key_event_relation_behaviour_storage() -> KeyEventRelationBehaviourStorage {
    KeyEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_send_key_event_relation_behaviour_storage() -> SendKeyEventRelationBehaviourStorage {
    SendKeyEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_led_event_relation_behaviour_storage() -> LedEventRelationBehaviourStorage {
    LedEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_send_led_event_relation_behaviour_storage() -> SendLedEventRelationBehaviourStorage {
    SendLedEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_relative_axis_event_relation_behaviour_storage() -> RelativeAxisEventRelationBehaviourStorage {
    RelativeAxisEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_absolute_axis_event_relation_behaviour_storage() -> AbsoluteAxisEventRelationBehaviourStorage {
    AbsoluteAxisEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_switch_event_relation_behaviour_storage() -> SwitchEventRelationBehaviourStorage {
    SwitchEventRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait InputDeviceRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_send_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_send_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_send_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_send_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_relative_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_relative_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_absolute_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_absolute_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_switch_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_switch_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

// #[derive(Clone)]
pub struct InputDeviceRelationBehaviourProviderImpl {
    key_event_relation_behaviours: KeyEventRelationBehaviourStorage,
    send_key_event_relation_behaviours: SendKeyEventRelationBehaviourStorage,
    led_event_relation_behaviours: LedEventRelationBehaviourStorage,
    send_led_event_relation_behaviours: SendLedEventRelationBehaviourStorage,
    relative_axis_event_relation_behaviours: RelativeAxisEventRelationBehaviourStorage,
    absolute_axis_event_relation_behaviours: AbsoluteAxisEventRelationBehaviourStorage,
    switch_event_relation_behaviours: SwitchEventRelationBehaviourStorage,
}

interfaces!(InputDeviceRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

#[component]
impl InputDeviceRelationBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            key_event_relation_behaviours: create_key_event_relation_behaviour_storage(),
            send_key_event_relation_behaviours: create_send_key_event_relation_behaviour_storage(),
            led_event_relation_behaviours: create_led_event_relation_behaviour_storage(),
            send_led_event_relation_behaviours: create_send_led_event_relation_behaviour_storage(),
            relative_axis_event_relation_behaviours: create_relative_axis_event_relation_behaviour_storage(),
            absolute_axis_event_relation_behaviours: create_absolute_axis_event_relation_behaviour_storage(),
            switch_event_relation_behaviours: create_switch_event_relation_behaviour_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl InputDeviceRelationBehaviourProvider for InputDeviceRelationBehaviourProviderImpl {
    fn create_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let key_event = KeyEvent::new(relation_instance);
        if key_event.is_ok() {
            self.key_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(key_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", KEY_EVENT, edge_key);
        }
    }

    fn remove_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.key_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", KEY_EVENT, edge_key);
    }

    fn create_send_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let send_key_event = SendKeyEvent::new(relation_instance);
        if send_key_event.is_ok() {
            self.send_key_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(send_key_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", SEND_KEY_EVENT, edge_key);
        }
    }

    fn remove_send_key_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.send_key_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", SEND_KEY_EVENT, edge_key);
    }

    fn create_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let led_event = LedEvent::new(relation_instance);
        if led_event.is_ok() {
            self.led_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(led_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", LED_EVENT, edge_key);
        }
    }

    fn remove_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.led_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", LED_EVENT, edge_key);
    }

    fn create_send_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let send_led_event = SendLedEvent::new(relation_instance);
        if send_led_event.is_ok() {
            self.send_led_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(send_led_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", SEND_LED_EVENT, edge_key);
        }
    }

    fn remove_send_led_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.send_led_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", SEND_LED_EVENT, edge_key);
    }

    fn create_relative_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let relative_axis_event = RelativeAxisEvent::new(relation_instance);
        if relative_axis_event.is_ok() {
            self.relative_axis_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(relative_axis_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", RELATIVE_AXIS_EVENT, edge_key);
        }
    }

    fn remove_relative_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.relative_axis_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", RELATIVE_AXIS_EVENT, edge_key);
    }

    fn create_absolute_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let absolute_axis_event = AbsoluteAxisEvent::new(relation_instance);
        if absolute_axis_event.is_ok() {
            self.absolute_axis_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(absolute_axis_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", ABSOLUTE_AXIS_EVENT, edge_key);
        }
    }

    fn remove_absolute_axis_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.absolute_axis_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", ABSOLUTE_AXIS_EVENT, edge_key);
    }

    fn create_switch_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let switch_event = SwitchEvent::new(relation_instance);
        if switch_event.is_ok() {
            self.switch_event_relation_behaviours
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), Arc::new(switch_event.unwrap()));
            trace!("Added relation behaviour {} to relation instance {:?}", SWITCH_EVENT, edge_key);
        }
    }

    fn remove_switch_event_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.switch_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
        trace!("Removed behaviour {} from relation instance {:?}", SWITCH_EVENT, edge_key);
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self.key_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.key_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", KEY_EVENT, edge_key);
        }
        if self.send_key_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.send_key_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", SEND_KEY_EVENT, edge_key);
        }
        if self.led_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.led_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", LED_EVENT, edge_key);
        }
        if self.send_led_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.send_led_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", SEND_LED_EVENT, edge_key);
        }
        if self.relative_axis_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.relative_axis_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", RELATIVE_AXIS_EVENT, edge_key);
        }
        if self.absolute_axis_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.absolute_axis_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", ABSOLUTE_AXIS_EVENT, edge_key);
        }
        if self.switch_event_relation_behaviours.0.write().unwrap().contains_key(&edge_key) {
            self.switch_event_relation_behaviours.0.write().unwrap().remove(&edge_key);
            trace!("Removed behaviour {} from relation instance {:?}", SWITCH_EVENT, edge_key);
        }
    }
}

impl RelationBehaviourProvider for InputDeviceRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            KEY_EVENT => self.create_key_event_behaviour(relation_instance),
            SEND_KEY_EVENT => self.create_send_key_event_behaviour(relation_instance),
            LED_EVENT => self.create_led_event_behaviour(relation_instance),
            SEND_LED_EVENT => self.create_send_led_event_behaviour(relation_instance),
            RELATIVE_AXIS_EVENT => self.create_relative_axis_event_behaviour(relation_instance),
            ABSOLUTE_AXIS_EVENT => self.create_absolute_axis_event_behaviour(relation_instance),
            SWITCH_EVENT => self.create_switch_event_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            KEY_EVENT => self.remove_key_event_behaviour(relation_instance),
            SEND_KEY_EVENT => self.remove_send_key_event_behaviour(relation_instance),
            LED_EVENT => self.remove_led_event_behaviour(relation_instance),
            SEND_LED_EVENT => self.remove_send_led_event_behaviour(relation_instance),
            RELATIVE_AXIS_EVENT => self.remove_relative_axis_event_behaviour(relation_instance),
            ABSOLUTE_AXIS_EVENT => self.remove_absolute_axis_event_behaviour(relation_instance),
            SWITCH_EVENT => self.remove_switch_event_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
