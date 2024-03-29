use crate::domain::event::Event;
use crate::repository::persistence_event::PersistenceEvent;
use std::collections::HashMap;
use crate::constants::{
    PAD_CHAR, 
    MAX_PARAM_VALUE_SIZE, 
    MAX_PARAM_KEY_SIZE, 
    MAX_EVENT_SIZE, 
    DEFAULT_EVENT_TYPE_KEY, 
    DEFAULT_PARAM, 
    DEFAULT_PARAMS,
    MAX_NUM_PARAMS,
    MAX_EVENT_TYPE_KEY_SIZE
};


pub fn str_to_persistence_event(event_str: &str) -> PersistenceEvent {

    let event_str_chars : Vec<char> = event_str.chars().collect();

    let mut event_type_key = DEFAULT_EVENT_TYPE_KEY;
    for j in 0..MAX_EVENT_TYPE_KEY_SIZE {
        event_type_key[j] = event_str_chars[j];
    }

    let mut params = DEFAULT_PARAMS;
    for k in 0..MAX_NUM_PARAMS {
        let param_start_idx = MAX_EVENT_TYPE_KEY_SIZE + (MAX_PARAM_KEY_SIZE + MAX_PARAM_VALUE_SIZE) * k;

        for j in 0..MAX_PARAM_KEY_SIZE {
            params[k].0[j] = event_str_chars[param_start_idx + j];
        }

        for j in 0..MAX_PARAM_VALUE_SIZE {
            params[k].1[j] = event_str_chars[param_start_idx + MAX_PARAM_KEY_SIZE + j];
        }
    }


    PersistenceEvent {
        event_type_key: event_type_key,
        params: params
    }   
} 

pub fn split_into_event_strs(total_str: &str) -> Vec<&str> {

    let mut subs = Vec::with_capacity(total_str.len() / MAX_EVENT_SIZE);
    let mut iter = total_str.chars();
    let mut pos = 0;

    while pos < total_str.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(MAX_EVENT_SIZE) {
            len += ch.len_utf8();
        }
        subs.push(&total_str[pos..pos + len]);
        pos += len;
    }
    
    subs
}

pub fn merge_all_strings(persistence_event: PersistenceEvent) -> [char; MAX_EVENT_SIZE] {
    let mut ret = [PAD_CHAR; MAX_EVENT_SIZE];
    let mut i = 0;

    for j in 0..MAX_EVENT_TYPE_KEY_SIZE {
        ret[i] = persistence_event.event_type_key[j];
        i = i + 1;
    }

    for k in 0..MAX_NUM_PARAMS {
        for j in 0..MAX_PARAM_KEY_SIZE {
            ret[i] = persistence_event.params[k].0[j];
            i = i + 1;
        }

        for j in 0..MAX_PARAM_VALUE_SIZE {
            ret[i] = persistence_event.params[k].1[j];
            i = i + 1;
        }
    }

    ret
}

pub fn domain_to_persistence_event(event: Event) -> PersistenceEvent {

    let mut event_type_key = DEFAULT_EVENT_TYPE_KEY;
    let mut idx = 0;
    for ch in event.event_type_key.chars() {
        event_type_key[idx] = ch;
        idx = idx + 1;
    }

    let mut params = DEFAULT_PARAMS;
    idx = 0;
    for (key, val) in event.params {
       
        let mut j = 0;
        for ch in key.chars() {
            params[idx].0[j] = ch;
            j = j + 1;
        }

        j = 0;
        for ch in val.chars() {
            params[idx].1[j] = ch;
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
    for _i in 0..padded_string.len() {

        if char_vec[padded_string.len() - 1] == PAD_CHAR {
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
    fn test_split_into_event_strs() {
        let default_param_str = "********************************************************************************************************************************************";
        
        let event_type_key_str_1 = "upsert-entity*******"; 
        let param_key_str_1 = "id******************"; 
        let param_val_str_1 = "test123*****************************************************************************************************************";
        let event_str_1 = event_type_key_str_1.to_string() + param_key_str_1 + param_val_str_1 + default_param_str + default_param_str + default_param_str + default_param_str;

        let event_type_key_str_2 = "remove-entity*******"; 
        let param_key_str_2 = "key*****************"; 
        let param_val_str_2 = "test12345***************************************************************************************************************";
        let event_str_2 = event_type_key_str_2.to_string() + param_key_str_2 + param_val_str_2 + default_param_str + default_param_str + default_param_str + default_param_str;

        let total_str = event_str_1.clone() + &event_str_2;

        let split_strs : Vec<&str> = split_into_event_strs(&total_str);

        assert_eq!(event_str_1, split_strs[0]);
    }

    #[test]
    fn test_str_to_persistence_happy_path() {
        let default_param_str = "********************************************************************************************************************************************";
        let event_type_key_str = "upsert-entity*******"; 
        let param_key_str = "id******************"; 
        let param_val_str = "test123*****************************************************************************************************************";
        let event_str = event_type_key_str.to_string() + param_key_str + param_val_str + default_param_str + default_param_str + default_param_str + default_param_str;

        let mut expected_p_params = DEFAULT_PARAMS;
        expected_p_params[0].0[0] = 'i';
        expected_p_params[0].0[1] = 'd';
        
        expected_p_params[0].1[0] = 't';
        expected_p_params[0].1[1] = 'e';
        expected_p_params[0].1[2] = 's';
        expected_p_params[0].1[3] = 't';
        expected_p_params[0].1[4] = '1';
        expected_p_params[0].1[5] = '2';
        expected_p_params[0].1[6] = '3';

        let mut expected_p_event_type = DEFAULT_EVENT_TYPE_KEY;
        expected_p_event_type[0] = 'u';
        expected_p_event_type[1] = 'p';
        expected_p_event_type[2] = 's';
        expected_p_event_type[3] = 'e';
        expected_p_event_type[4] = 'r';
        expected_p_event_type[5] = 't';
        expected_p_event_type[6] = '-';
        expected_p_event_type[7] = 'e';
        expected_p_event_type[8] = 'n';
        expected_p_event_type[9] = 't';
        expected_p_event_type[10] = 'i';
        expected_p_event_type[11] = 't';
        expected_p_event_type[12] = 'y';

        let expected_p_event = PersistenceEvent {
            event_type_key: expected_p_event_type,
            params: expected_p_params
        };

        let actual_p_event = str_to_persistence_event(&event_str);

        assert_eq!(expected_p_event, actual_p_event);
       
    }
    
    #[test]
    fn test_domain_to_persistence_happy_path() {
        let mut params : HashMap<String, String> = HashMap::new();
        params.insert("id".to_string(), "test123".to_string());

        let event = Event {
            event_type_key: "upsert-entity".to_string(),
            params: params
        };

        let mut expected_p_params = DEFAULT_PARAMS;
        expected_p_params[0].0[0] = 'i';
        expected_p_params[0].0[1] = 'd';
        
        expected_p_params[0].1[0] = 't';
        expected_p_params[0].1[1] = 'e';
        expected_p_params[0].1[2] = 's';
        expected_p_params[0].1[3] = 't';
        expected_p_params[0].1[4] = '1';
        expected_p_params[0].1[5] = '2';
        expected_p_params[0].1[6] = '3';

        let mut expected_p_event_type = DEFAULT_EVENT_TYPE_KEY;
        expected_p_event_type[0] = 'u';
        expected_p_event_type[1] = 'p';
        expected_p_event_type[2] = 's';
        expected_p_event_type[3] = 'e';
        expected_p_event_type[4] = 'r';
        expected_p_event_type[5] = 't';
        expected_p_event_type[6] = '-';
        expected_p_event_type[7] = 'e';
        expected_p_event_type[8] = 'n';
        expected_p_event_type[9] = 't';
        expected_p_event_type[10] = 'i';
        expected_p_event_type[11] = 't';
        expected_p_event_type[12] = 'y';

        let expected_p_event = PersistenceEvent {
            event_type_key: expected_p_event_type,
            params: expected_p_params
        };

        let actual_p_event = domain_to_persistence_event(event);

        assert_eq!(expected_p_event, actual_p_event);
    }

    #[test]
    fn test_persistence_to_domain_happy_path() {
        let mut params : HashMap<String, String> = HashMap::new();
        params.insert("id".to_string(), "test123".to_string());

        let mut p_params = DEFAULT_PARAMS;
        p_params[0].0[0] = 'i';
        p_params[0].0[1] = 'd';
        
        p_params[0].1[0] = 't';
        p_params[0].1[1] = 'e';
        p_params[0].1[2] = 's';
        p_params[0].1[3] = 't';
        p_params[0].1[4] = '1';
        p_params[0].1[5] = '2';
        p_params[0].1[6] = '3';

        let mut p_event_type = DEFAULT_EVENT_TYPE_KEY;
        p_event_type[0] = 'u';
        p_event_type[1] = 'p';
        p_event_type[2] = 's';
        p_event_type[3] = 'e';
        p_event_type[4] = 'r';
        p_event_type[5] = 't';
        p_event_type[6] = '-';
        p_event_type[7] = 'e';
        p_event_type[8] = 'n';
        p_event_type[9] = 't';
        p_event_type[10] = 'i';
        p_event_type[11] = 't';
        p_event_type[12] = 'y';

        let p_event = PersistenceEvent {
            event_type_key: p_event_type,
            params: p_params
        };

        let mut params : HashMap<String, String> = HashMap::new();
        params.insert("id".to_string(), "test123".to_string());

        let expected_event = Event {
            event_type_key: "upsert-entity".to_string(),
            params: params
        };

        let actual_event = persistence_to_domain_event(p_event);

        assert_eq!(expected_event, actual_event);
    }
}

