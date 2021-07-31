use crate::constants::{
    EventTypeKey, 
    Params
};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PersistenceEvent {
    pub event_type_key: EventTypeKey, 
    pub params: Params 
}
