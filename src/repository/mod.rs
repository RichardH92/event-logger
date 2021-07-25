use crate::domain::event::Event;

pub trait EventRepository {
    fn new() -> Self;
    fn append_events(events: Vec<Event>) -> Result<(), ()>;
    fn get_events() -> Vec<Event>;
}
