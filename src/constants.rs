pub const MAX_EVENT_TYPE_KEY_SIZE: usize = 20;
pub const MAX_PARAM_KEY_SIZE: usize = 20;
pub const MAX_PARAM_VALUE_SIZE: usize = 120;
pub const MAX_NUM_PARAMS: usize = 5;

pub const PAD_CHAR : char = '*';
pub const PARAM_VALUE_PAIR_SIZE: usize = MAX_PARAM_KEY_SIZE + MAX_PARAM_VALUE_SIZE;
pub const MAX_EVENT_SIZE: usize = MAX_EVENT_TYPE_KEY_SIZE + (PARAM_VALUE_PAIR_SIZE) * MAX_NUM_PARAMS;

pub const DEFAULT_EVENT_TYPE_KEY : EventTypeKey = [PAD_CHAR; MAX_EVENT_TYPE_KEY_SIZE];
pub const DEFAULT_PARAM : Param = ([PAD_CHAR; MAX_PARAM_KEY_SIZE], [PAD_CHAR; MAX_PARAM_VALUE_SIZE]);
pub const DEFAULT_PARAMS : Params = [DEFAULT_PARAM; MAX_NUM_PARAMS];

pub type ParamKey = [char; MAX_PARAM_KEY_SIZE];
pub type EventTypeKey = [char; MAX_EVENT_TYPE_KEY_SIZE];
pub type ParamValue = [char; MAX_PARAM_VALUE_SIZE];
pub type AllowedParams = [ParamKey; MAX_NUM_PARAMS];
pub type Param = (ParamKey, ParamValue);
pub type Params = [Param; MAX_NUM_PARAMS];


