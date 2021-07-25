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
    KeyAlreadyTaken,
    DuplicateKey
}

#[cfg(test)]
mod registry_tests {
    use super::*;
    use crate::service::event_type_registry_impl::EventTypeRegistryImpl;
    use crate::service::RegisterEventTypeValidationErrorType::{KeyAlreadyTaken, DuplicateKey};
    use crate::service::RegisterEventTypeValidationError;

    #[test]
    fn test_register_event_types_happy_path() {
        let mut registry : EventTypeRegistryImpl = EventTypeRegistry::new();
        let e1 = EventType { key: "upsert-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        let e2 = EventType { key: "add-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        let e3 = EventType { key: "remove-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        
        let result = registry.register_event_types(vec![e1.clone(), e2.clone(), e3.clone()]);
    
        assert_eq!(Ok(()), result);
   
        let registered_types = registry.get_registered_event_types();
        let expected_types = vec![e1, e2, e3];

        assert_eq!(expected_types, registered_types);
    }

    #[test]
    fn test_register_event_types_returns_error_when_key_already_taken() {
        let mut registry : EventTypeRegistryImpl = EventTypeRegistry::new();
        let e1 = EventType { key: "upsert-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        let e2 = EventType { key: "add-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
       
        registry.register_event_types(vec![e1.clone(), e2.clone()]);
        
        let e3 = EventType { key: "remove-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        let e4 = EventType { key: "upsert-entity".to_string(), allowed_params: vec![], inverse_type: Some("remove-entity".to_string()) };
        
        let result = registry.register_event_types(vec![e3, e4]);
   
        let expected_errs = Err(vec![RegisterEventTypeValidationError{ event_type_key: "upsert-entity".to_string(), error_type: KeyAlreadyTaken }]);
        assert_eq!(expected_errs, result);
   
        let registered_types = registry.get_registered_event_types();
        let expected_types = vec![e1, e2];
    }

    #[test]
    fn test_register_event_types_returns_error_when_registering_duplicate_keys() {
        let mut registry : EventTypeRegistryImpl = EventTypeRegistry::new();
        let e1 = EventType { key: "upsert-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
        let e2 = EventType { key: "upsert-entity".to_string(), allowed_params: vec![], inverse_type: None };
        
        let result = registry.register_event_types(vec![e1, e2]);
        let expected_errs = Err(vec![RegisterEventTypeValidationError{ event_type_key: "upsert-entity".to_string(), error_type: DuplicateKey }]);

        assert_eq!(expected_errs, result);
    }
}
