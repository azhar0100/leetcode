use std::collections::HashSet;

use crate::util::double_pointer::SafeDoublePointer;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hashset_stack: HashSet<char> = HashSet::new();
    const INIT_VALUE: usize = 0;
    let mut pointer = SafeDoublePointer::new(s.len());
    let mut max_length = INIT_VALUE;
    loop {
        let current_end_pointer = pointer.double_pointer.right;
        let current_char = match s.chars().nth(current_end_pointer){
            Some(c) => c,
            None => break,
        };
        match hashset_stack.contains(&current_char) {
            true => {
                hashset_stack.clear();
                pointer.jump_to_right();
            }
            false => {
                hashset_stack.insert(current_char);
                max_length = max_length.max(pointer.len().abs() as usize);
                match pointer.advance_right() {
                    Ok(()) => (),
                    Err(_) => break,
                }
            }
        };
    }
    max_length as i32
}
