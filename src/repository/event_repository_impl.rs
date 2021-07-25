use crate::domain::event::Event;
use crate::repository::EventRepository;
use std::fs::OpenOptions;
use std::fs::File;
use crate::repository::event_repository_mapper::{merge_all_strings, domain_to_persistence_event};

pub struct EventRepositoryImpl {
    file: File
}

impl EventRepository for EventRepositoryImpl {
    fn new(file: File) -> EventRepositoryImpl {
        EventRepositoryImpl {
            file: file
        }
    }

    /*fn get_event(idx: usize) -> Result<Event, ()> {

    }*/

    fn get_events(&self, limit: usize, offset: usize) -> Vec<Event> {
        vec![]
    }

    fn append_event(&mut self, event: Event) -> Result<(), ()> {
     
        let persistence_event = domain_to_persistence_event(event);
        let total_char_arr = merge_all_strings(persistence_event);
        
        writeln!(self.file, total_char_arr.to_string())
    }
}
