use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Event {
    pub event_type_key: String, 
    pub params: HashMap<String, String> 
}
