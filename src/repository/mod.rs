use crate::domain::event::Event;
use std::fs::File;

pub mod event_repository_impl;
mod event_repository_mapper;
mod persistence_event;

pub trait EventRepository {
    fn new(file: File) -> Self;
    fn append_event(&mut self, event: Event) -> std::io::Result<()>; 
    //fn get_event(idx: usize) -> Result<Event, ()>;
    fn get_events(&mut self, limit: usize, offset: usize) -> std::io::Result<Vec<Event>>;
}
