use crate::repository::event_repository_impl::EventRepositoryImpl;
use crate::repository::EventRepository;
use crate::service::{EventLog, AppendEventValidationErrors, AppendEventValidationError, AppendEventValidationErrorType};
use crate::service::event_validator::validate_event;
use crate::domain::event::Event;
use std::error;

pub struct EventLogImpl {
   repo: EventRepositoryImpl,
}

impl EventLog for EventLogImpl {
    fn new(repo: EventRepositoryImpl) -> Self {
        EventLogImpl {
            repo
        }
    }

    fn clear_log(&self) -> std::io::Result<()> {
        self.repo.clear_log()
    }

    fn get_events(&self, limit: usize, offset: usize) -> std::io::Result<Vec<Event>> {
        self.repo.get_events(limit, offset)
    }

    fn log_event(&mut self, event: Event) -> Result<(), Box<dyn error::Error>> {
        validate_event(&event)?;
        self.repo.append_event(event)?;
    
        Ok(())
    }
}
