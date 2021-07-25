use crate::domain::event::Event;
use crate::repository::EventRepository;
use std::fs::OpenOptions;
use std::fs::File;

pub struct EventRepositoryImpl {
    // file: File
}

impl EventRepository for EventRepositoryImpl {
    fn new(/*file: mut File*/) -> EventRepositoryImpl {
        EventRepositoryImpl {
            //file: file
        }
    }

    fn get_events() -> Vec<Event> {
        vec![]
    }

    fn append_events(events: Vec<Event>) -> Result<(), ()> {
        /*if let Err(e) = writeln!(file, "A new line!") {
            eprintln!("Couldn't write to file: {}", e);
        }*/

        Ok(())
    }
}
