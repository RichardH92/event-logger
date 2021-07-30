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
    file_name: String
}

impl EventRepository for EventRepositoryImpl {
    fn new(file_name: String) -> Self {
        
        EventRepositoryImpl {
            file_name: file_name
        }
    }

    fn get_events(&self, limit: usize, offset: usize) -> std::io::Result<Vec<Event>> {
        let ret : Vec<Event> = Vec::new();

        let chars_to_read : usize = MAX_EVENT_SIZE * limit; 
        let mut buffer = vec![0; chars_to_read];

        let file = OpenOptions::new()
            .read(true)
            .open(self.file_name.clone())
            .unwrap();

        let mut handle = file.take(chars_to_read as u64);
        let amt_read = handle.read(&mut buffer)?;

        println!("{}", amt_read);

        buffer.resize(amt_read, 0);

        let total_str = str::from_utf8(&buffer).unwrap();
       
        println!("{}", total_str);

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

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(self.file_name.clone())
            .unwrap();

        file.write_all(bytes)?;
        file.flush()
    }
}
