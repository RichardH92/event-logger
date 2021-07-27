use crate::domain::event::Event;
use std::fs::File;
use std::collections::HashMap;
use std::fs::OpenOptions;

pub mod event_repository_impl;
mod event_repository_mapper;
mod persistence_event;

pub trait EventRepository {
    fn new(file_name: String) -> Self;
    fn append_event(&mut self, event: Event) -> std::io::Result<()>; 
    //fn get_event(idx: usize) -> Result<Event, ()>;
    fn get_events(&self, limit: usize, offset: usize) -> std::io::Result<Vec<Event>>;
}

#[cfg(test)]
mod event_repository_test { 
    use super::*;
    use crate::repository::event_repository_impl::EventRepositoryImpl;

    #[test]
    fn test_append_event_happy_path() {
        let mut repo : EventRepositoryImpl = EventRepository::new("foo".to_string()); 
        let mut params : HashMap<String, String> = HashMap::new();

        params.insert("id".to_string(), "test123".to_string());

        let event = Event {
            event_type_key: "upsert-entity".to_string(),
            params: params
        };

        let expected_event = event.clone();

        match repo.append_event(event) {
            Ok(()) => (),
            Err(_e) => assert_eq!(true, false)
        };

        match repo.get_events(10, 0) {
            Ok(events) => assert_eq!(vec![expected_event], events),
            Err(e) => { println!("{}", e); panic!() }
        };
    }
}
