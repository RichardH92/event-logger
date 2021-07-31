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

    #[test]
    fn test_get_events_happy_path() {
        let mut repo : EventRepositoryImpl = EventRepository::new("foo2".to_string()); 
        
        let mut params1 : HashMap<String, String> = HashMap::new();
        params1.insert("id".to_string(), "test123".to_string());
        let mut params2 : HashMap<String, String> = HashMap::new();
        params2.insert("id".to_string(), "test123".to_string());
        let mut params3 : HashMap<String, String> = HashMap::new();
        params3.insert("id".to_string(), "test123".to_string());
        let mut params4 : HashMap<String, String> = HashMap::new();
        params4.insert("id".to_string(), "test123".to_string());
        let mut params5 : HashMap<String, String> = HashMap::new();
        params5.insert("id".to_string(), "test123".to_string());
        let mut params6 : HashMap<String, String> = HashMap::new();
        params6.insert("id".to_string(), "test123".to_string());

        let mut events = vec![
            Event {
                event_type_key: "event1".to_string(),
                params: params1
            },
            Event {
                event_type_key: "event2".to_string(),
                params: params2
            },
            Event {
                event_type_key: "event3".to_string(),
                params: params3
            },
            Event {
                event_type_key: "event4".to_string(),
                params: params4
            },
            Event {
                event_type_key: "event5".to_string(),
                params: params5
            },
            Event {
                event_type_key: "event6".to_string(),
                params: params6
            }
        ];

        for event in &events {
            match repo.append_event(event.clone()) {
                Ok(()) => (),
                Err(_e) => assert_eq!(true, false)
            };
        }

        let expected_events = vec![events[2].clone(), events[3].clone(), events[4].clone(), events[5].clone()];

        match repo.get_events(10, 2) {
            Ok(actual_events) => assert_eq!(expected_events, actual_events),
            Err(e) => { println!("{}", e); panic!() }
        };
    }

}
