use crate::domain::event::Event;
use crate::domain::event_type::EventType;

pub mod event_log_impl;
pub mod event_type_registry_impl;

pub trait EventLog {
    fn clear_log(&mut self);
    fn get_events(&self) -> Vec<Event>;
    fn log_events(&mut self, events: Vec<Event>);
}

pub trait EventTypeRegistry {
    fn new() -> Self;
    fn register_event_types(&mut self, event_types: Vec<EventType>) -> Result<(), Vec<RegisterEventTypeValidationError>>; 
    fn get_registered_event_types(&self) -> Vec<EventType>;
    fn has_event_type_key_been_registered(&self, key: &String) -> bool;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RegisterEventTypeValidationError {
    error_type: RegisterEventTypeValidationErrorType,
    event_type_key: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RegisterEventTypeValidationErrorType {
    KeyAlreadyTaken
}

#[cfg(test)]
mod registry_tests {
    use super::*;
    use crate::service::event_type_registry_impl::EventTypeRegistryImpl;

    #[test]
    fn test_register_event_types_happy_path() {
        let mut registry : EventTypeRegistryImpl = EventTypeRegistry::new();
        let event_type = EventType { key: "upsert-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };

        let result = registry.register_event_types(vec![event_type]);
    
        assert_eq!(Ok(()), result);
    }
}
