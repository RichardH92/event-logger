use crate::service::RegisterEventTypeValidationError;
use crate::service::RegisterEventTypeValidationErrorType::KeyAlreadyTaken;
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

    fn register_event_types(&mut self, mut event_types: Vec<EventType>) -> Result<(), Vec<RegisterEventTypeValidationError>> {
        
        let errors : Vec<RegisterEventTypeValidationError> = event_types.iter()
            .filter(|event_type| self.has_event_type_key_been_registered(&event_type.key))
            .map(|event_type| RegisterEventTypeValidationError { error_type: KeyAlreadyTaken, event_type_key: event_type.key.clone() })
            .collect();


        if errors.is_empty() {
            self.registered_event_types.append(&mut event_types);
            return Ok(());
        }

        Err(errors)
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
