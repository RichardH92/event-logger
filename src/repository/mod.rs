use crate::domain::event::Event;

pub mod event_repository_impl;
mod event_repository_mapper;
mod persistence_event;

pub trait EventRepository {
    fn new() -> Self;
    fn append_events(events: Vec<Event>) -> Result<(), ()>;
    fn get_events() -> Vec<Event>;
}
