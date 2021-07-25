use crate::domain::event::Event;
use crate::repository::persistence_event::PersistenceEvent;
use crate::domain::MAX_PARAM_KEY_SIZE;
use crate::domain::MAX_PARAM_VALUE_SIZE;
use crate::domain::MAX_EVENT_TYPE_KEY_SIZE;
use crate::domain::MAX_NUM_PARAMS;
use crate::repository::persistence_event::{EventTypeKey, Param, Params};
use std::collections::HashMap;

const PAD_CHAR : char = '*';
const PARAM_VALUE_PAIR_SIZE: usize = MAX_PARAM_KEY_SIZE + MAX_PARAM_VALUE_SIZE;
const MAX_EVENT_SIZE: usize = MAX_EVENT_TYPE_KEY_SIZE + (PARAM_VALUE_PAIR_SIZE) * MAX_NUM_PARAMS;

const DEFAULT_EVENT_TYPE_KEY : EventTypeKey = [PAD_CHAR; MAX_EVENT_TYPE_KEY_SIZE];
const DEFAULT_PARAM : Param = ([PAD_CHAR; MAX_PARAM_KEY_SIZE], [PAD_CHAR; MAX_PARAM_VALUE_SIZE]);
const DEFAULT_PARAMS : Params = [DEFAULT_PARAM; MAX_NUM_PARAMS];


pub fn domain_to_persistence_event(event: Event) -> PersistenceEvent {

    let mut event_type_key = DEFAULT_EVENT_TYPE_KEY;
    let mut idx = 0;
    for ch in event.event_type_key.chars() {
        event_type_key[idx] = ch;
        idx = idx + 1;
    }

    let mut params = DEFAULT_PARAMS;
    idx = 0;
    for p in event.params {
        let mut param : Param = params[idx];
       
        let mut j = 0;
        for ch in p.0.chars() {
            param.0[j] = ch;
            j = j + 1;
        }

        j = 0;
        for ch in p.1.chars() {
            param.1[j] = ch;
            j = j + 1;
        }

        idx = idx + 1;
    }


    PersistenceEvent {
        event_type_key: event_type_key,
        params: params
    }
}

fn strip_padding(mut padded_string: String) -> String {
    let char_vec : Vec<char> = padded_string.chars().collect();
    for i in 0..padded_string.len() {

        if char_vec[padded_string.len() - i - 1] == PAD_CHAR {
            padded_string.pop();
        } else {
            break;
        }
    }

    padded_string.clone()
}

pub fn persistence_to_domain_event(persistence_event: PersistenceEvent) -> Event {
    let mut event_type_key : String = persistence_event.event_type_key.iter().collect();
    event_type_key = strip_padding(event_type_key);

    let mut params : HashMap<String, String> = HashMap::new();

    for p in persistence_event.params {
        if p != DEFAULT_PARAM {
            let mut param_key_str : String = p.0.iter().collect();
            let mut param_val_str : String = p.1.iter().collect();

            param_key_str = strip_padding(param_key_str);
            param_val_str = strip_padding(param_val_str);

            params.insert(param_key_str, param_val_str);
        }
    }

    Event {
        event_type_key: event_type_key,
        params: params
    }
}

#[cfg(test)]
mod repo_mapper_test {
    use super::*;

    #[test]
    fn test_map_happy_path() {
        let mut params : HashMap<String, String> = HashMap::new();
        params.insert("id".to_string(), "test123".to_string());

        let event = Event {
            event_type_key: "upsert-entity".to_string(),
            params: params
        };

        let expected_event = event.clone();
        let p_event = domain_to_persistence_event(event);
        let actual_event = persistence_to_domain_event(p_event);

        assert_eq!(expected_event, actual_event);
    }
}

