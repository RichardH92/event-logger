use crate::domain::event::Event;
use crate::repository::EventRepository;
use std::fs::OpenOptions;
use std::fs::File;
use crate::repository::event_repository_mapper::{merge_all_strings, domain_to_persistence_event, split_into_event_strs};
use crate::repository::event_repository_mapper::{persistence_to_domain_event, str_to_persistence_event};
use std::io::prelude::*;
use crate::repository::event_repository_mapper::MAX_EVENT_SIZE;
use std::mem;
use std::str;

pub struct EventRepositoryImpl {
    file: File 
}

impl EventRepository for EventRepositoryImpl {
    fn new(file: File) -> Self {
        
        EventRepositoryImpl {
            file: file
        }
    }

    fn get_events(&mut self, limit: usize, offset: usize) -> std::io::Result<Vec<Event>> {
        let ret : Vec<Event> = Vec::new();

        let bytes_to_read : usize = MAX_EVENT_SIZE * limit * mem::size_of::<char>();
        let mut buffer = vec![0; bytes_to_read];

        self.file.read_exact(&mut buffer)?;

        let mut total_str = str::from_utf8(&buffer).unwrap();
        
        let domain_events : Vec<Event> = split_into_event_strs(total_str).iter()
            .map(|event_str| str_to_persistence_event(event_str))
            .map(|p_event| persistence_to_domain_event(p_event))
            .collect();

        Ok(domain_events)
    }

    fn append_event(&mut self, event: Event) -> std::io::Result<()> {
        let persistence_event = domain_to_persistence_event(event);
        let total_char_arr = merge_all_strings(persistence_event);
      
        let total_str : String = total_char_arr.iter().collect();
        let bytes : &[u8] = total_str.as_bytes();

        self.file.write_all(&bytes)
    }
}
