use crate::domain::event::Event;
use crate::domain::event_type::EventType;
use std::error;
use std::fmt;

pub mod event_log_impl;
pub mod event_type_registry_impl;
pub mod event_validator;

pub trait EventLog {
    fn clear_log(&self) -> std::io::Result<()>;
    fn get_events(&self) -> Vec<Event>;
    fn log_event(&mut self, event: Event) -> Result<(), Box<dyn error::Error>>;
}

pub trait EventTypeRegistry {
    fn new() -> Self;
    fn register_event_types(&mut self, event_types: Vec<EventType>) -> Result<(), RegisterEventTypeValidationErrors>; 
    fn get_registered_event_types(&self) -> Vec<EventType>;
    fn has_event_type_key_been_registered(&self, key: &String) -> bool;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppendEventValidationError {
    error_type: AppendEventValidationErrorType,
    value: Option<String>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppendEventValidationErrors {
    errors: Vec<AppendEventValidationError>
}

impl error::Error for AppendEventValidationErrors {}

impl fmt::Display for AppendEventValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid event")
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum AppendEventValidationErrorType {
    EventKeyExceedsMaxLength,
    ParamKeyExceedsMaxLength,
    ParamValExceedsMaxLength,
    NumParamsGreaterThanMax
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RegisterEventTypeValidationError {
    error_type: RegisterEventTypeValidationErrorType,
    event_type_key: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RegisterEventTypeValidationErrors {
    errors: Vec<RegisterEventTypeValidationError>
}

impl error::Error for RegisterEventTypeValidationErrors {}

impl fmt::Display for RegisterEventTypeValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid event type")
    }
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

    // TODO: Add validation + test that param keys must be unique

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
   
        let expected_errs = Err(RegisterEventTypeValidationErrors {
            errors: vec![RegisterEventTypeValidationError{ event_type_key: "upsert-entity".to_string(), error_type: KeyAlreadyTaken }]
        });
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
        let expected_errs = Err(RegisterEventTypeValidationErrors {
            errors: vec![RegisterEventTypeValidationError{ event_type_key: "upsert-entity".to_string(), error_type: DuplicateKey }]
        });

        assert_eq!(expected_errs, result);
    }
}
