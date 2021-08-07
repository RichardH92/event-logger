use criterion::{black_box, criterion_group, criterion_main, Criterion};
use event_logger::service::event_log_impl::EventLogImpl;
use event_logger::service::EventLog;
use event_logger::domain::event::Event;
use event_logger::repository::event_repository_impl::EventRepositoryImpl;
use event_logger::repository::EventRepository;
use unchecked_unwrap::UncheckedUnwrap;
use std::collections::HashMap;

static mut event_log : Option<EventLogImpl> = None;

fn setup() {
    unsafe {
        let mut repo : EventRepositoryImpl = EventRepository::new("foo".to_string()); 
        event_log = Some(EventLog::new(repo));
    }
        //let e1 = EventType { key: "upsert-entity".to_string(), allowed_params: vec!["id".to_string()], inverse_type: None };
}

fn teardown() {
    unsafe {
        let log = event_log.as_mut().unwrap();
        log.clear_log(); 
    }
}

fn benchmark_append_event(c: &mut Criterion) {
    setup();

    unsafe {
        let log = event_log.as_mut().unwrap();
        c.bench_function("append event", |b| b.iter(|| execute_append_event(log)));
    }

    teardown();
}

fn execute_append_event(log: &mut EventLogImpl) {
    
    let mut params : HashMap<String, String> = HashMap::new();

    params.insert("id".to_string(), "test123".to_string());

    let event = Event {
        event_type_key: "upsert-entity".to_string(),
        params: params
    };

    log.log_event(event);
}

criterion_group!(benches, benchmark_append_event);
criterion_main!(benches);
