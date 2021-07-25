use crate::domain::event::Event;
use crate::repository::EventRepository;
use std::fs::OpenOptions;
use std::fs::File;
use crate::repository::event_repository_mapper::{merge_all_strings, domain_to_persistence_event};
use std::io::prelude::*;

pub struct EventRepositoryImpl {
    buffer: File 
}

impl EventRepository for EventRepositoryImpl {
    fn new(file: File) -> Self {
        
        EventRepositoryImpl {
            buffer: file
        }
    }

    /*fn get_event(idx: usize) -> Result<Event, ()> {

    }*/

    fn get_events(&self, limit: usize, offset: usize) -> Vec<Event> {
        vec![]
    }

    fn append_event(&mut self, event: Event) -> std::io::Result<()> {
        let persistence_event = domain_to_persistence_event(event);
        let total_char_arr = merge_all_strings(persistence_event);
      
        let total_str : String = total_char_arr.iter().collect();
        let bytes : &[u8] = total_str.as_bytes();

        self.buffer.write_all(&bytes)
    }
}
