pub const MAX_EVENT_TYPE_KEY_SIZE: usize = 20;
pub const MAX_PARAM_KEY_SIZE: usize = 20;
pub const MAX_PARAM_VALUE_SIZE: usize = 120;
pub const MAX_NUM_PARAMS: usize = 5;

pub type ParamKey = [char; MAX_PARAM_KEY_SIZE];
pub type EventTypeKey = [char; MAX_EVENT_TYPE_KEY_SIZE];
pub type ParamValue = [char; MAX_PARAM_VALUE_SIZE];
pub type AllowedParams = [ParamKey; MAX_NUM_PARAMS];
pub type Params = [(ParamKey, ParamValue); MAX_NUM_PARAMS];

pub mod event;
pub mod event_type;
