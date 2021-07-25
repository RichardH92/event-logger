const PAD_CHAR : char = '*';

pub fn pad_string(val: &mut [char], max_size: usize) {
   let pad_amt : usize = max_size - val.len();
   let start_idx : usize = val.len();

   for i in 0..pad_amt {
       val[start_idx + i] = PAD_CHAR;
   }
}

pub fn get_unpad_amt(val: &[char], max_size: usize) -> usize {

    let mut unpad_amt = 0;

    for i in 0..unpad_amt {
        let idx = max_size - i - 1;
        if val[idx] == PAD_CHAR {
            unpad_amt = unpad_amt + 1;
        } else {
            break;
        }
    }

    unpad_amt
}
