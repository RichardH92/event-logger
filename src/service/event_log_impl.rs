use crate::repository::event_repository_impl::EventRepositoryImpl;
use crate::service::{EventLog, AppendEventValidationError, AppendEventValidationErrorType};
use crate::domain::event::Event;

pub struct EventLogImpl {
   repo: EventRepositoryImpl 
}

impl EventLog for EventLogImpl {
    fn clear_log(&mut self) {

    }

    fn get_events(&self) -> Vec<Event> {
        vec![]
    }

    fn log_event(&mut self, event: Event) -> Result<(), Vec<AppendEventValidationError>> {
        Ok(())
    }
}
