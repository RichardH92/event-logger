use crate::service::{AppendEventValidationError, AppendEventValidationErrors, AppendEventValidationErrorType};
use crate::domain::event::Event;
use crate::constants::{MAX_NUM_PARAMS, MAX_EVENT_TYPE_KEY_SIZE, MAX_PARAM_KEY_SIZE, MAX_PARAM_VALUE_SIZE};

pub fn validate_event(event: &Event) -> Result<(), AppendEventValidationErrors> {
  
    let mut errors : Vec<AppendEventValidationError> = vec![];

    if event.event_type_key.len() >= MAX_EVENT_TYPE_KEY_SIZE {
       errors.push(AppendEventValidationError {
           error_type: AppendEventValidationErrorType::EventKeyExceedsMaxLength,
           value: Some(event.event_type_key.clone())
       });
    }

    let mut i = 0;
    for (key, val) in event.params.iter() {
        if key.len() >= MAX_PARAM_KEY_SIZE {
            errors.push(AppendEventValidationError {
                error_type: AppendEventValidationErrorType::ParamKeyExceedsMaxLength,
                value: Some(key.clone())
            });
        }

        if val.len() >= MAX_PARAM_VALUE_SIZE {
            errors.push(AppendEventValidationError {
                error_type: AppendEventValidationErrorType::ParamValExceedsMaxLength,
                value: Some(key.clone())
            });
        }
    
        i = i + 1;
    }

    if i > MAX_NUM_PARAMS {
        errors.push(AppendEventValidationError {
            error_type: AppendEventValidationErrorType::NumParamsGreaterThanMax,
            value: None
        })
    }

    Ok(())
}


