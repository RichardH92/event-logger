pub mod string_padder;

#[cfg(test)]
mod string_padder_tests {
    use super::*;
    use string_padder::pad_string;
    
    #[test]
    fn test_pad_and_unpad_happy_path() {
        /*let mut string_to_pad : [char; 10] = ['p', 'a', 'd', 'm', 'e'];

        pad_string(&mut string_to_pad, 10);
        let expected_padded = ['p', 'a', 'd', 'm', 'e', '*', '*', '*', '*', '*'];
  
        let expected : String = expected_padded.iter().collect();
        let actual : String = string_to_pad.iter().collect();
        
        assert_eq!(expected, actual);*/ 
    }
}
