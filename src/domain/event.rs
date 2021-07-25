use std::collections::HashMap;
use crate::domain::event_type::ParamType;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Event {
    pub event_type_key: String,
    pub params: HashMap<ParamType, String>
}
