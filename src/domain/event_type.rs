
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct EventType {
    pub key: String, 
    pub allowed_params: Vec<String>,
    pub inverse_type: Option<String> 
}
