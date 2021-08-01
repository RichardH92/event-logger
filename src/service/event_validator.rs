use crate::service::{AppendEventValidationError, AppendEventValidationErrors, AppendEventValidationErrorType};
use crate::domain::event::Event;

pub fn validate_event(event: &Event) -> Result<(), AppendEventValidationErrors> {
    Ok(())
}


