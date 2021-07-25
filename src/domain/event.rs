use std::collections::HashMap;
use crate::domain::Params;
use crate::domain::EventTypeKey;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Event {
    pub event_type_key: EventTypeKey, 
    pub params: Params 
}
