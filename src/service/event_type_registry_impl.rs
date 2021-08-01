use crate::service::{RegisterEventTypeValidationError, RegisterEventTypeValidationErrors};
use crate::service::RegisterEventTypeValidationErrorType::{DuplicateKey, KeyAlreadyTaken};
use crate::domain::event_type::EventType;
use crate::service::EventTypeRegistry;

pub struct EventTypeRegistryImpl {
    registered_event_types: Vec<EventType>
}

impl EventTypeRegistry for EventTypeRegistryImpl {
    fn new() -> EventTypeRegistryImpl {
        EventTypeRegistryImpl {
            registered_event_types: Vec::new()
        }
    }

    fn register_event_types(&mut self, mut event_types: Vec<EventType>) -> Result<(), RegisterEventTypeValidationErrors> {

        let mut errors : Vec<RegisterEventTypeValidationError> = vec![];

        let mut key_already_taken_errors : Vec<RegisterEventTypeValidationError> = event_types.iter()
            .filter(|event_type| self.has_event_type_key_been_registered(&event_type.key))
            .map(|event_type| RegisterEventTypeValidationError { error_type: KeyAlreadyTaken, event_type_key: event_type.key.clone() })
            .collect();

        let mut duplicate_key_errors : Vec<RegisterEventTypeValidationError> = event_types.iter()
            .filter(|event_type| event_types.iter()
                    .filter(|e1| e1.key == event_type.key)
                    .count() > 1)
            .map(|event_type| RegisterEventTypeValidationError { error_type: DuplicateKey, event_type_key: event_type.key.clone() })
            .collect();

        key_already_taken_errors.dedup();
        duplicate_key_errors.dedup();

        errors.append(&mut key_already_taken_errors);
        errors.append(&mut duplicate_key_errors);

        if errors.is_empty() {
            self.registered_event_types.append(&mut event_types);
            return Ok(());
        }

        Err(RegisterEventTypeValidationErrors { errors })
    }

    fn get_registered_event_types(&self) -> Vec<EventType> {
        self.registered_event_types.clone()
    }
    
    fn has_event_type_key_been_registered(&self, key: &String) -> bool {
        for registered_type in self.registered_event_types.iter() {
            if registered_type.key == *key {
                return true;
            }
        }
    
        false
    }
}
