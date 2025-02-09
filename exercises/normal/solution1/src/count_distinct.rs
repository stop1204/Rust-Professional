use std::collections::HashSet;


pub fn new_count_distinct(input_str: &str) -> usize {
    let mut count = 0;
    let mut set = HashSet::new();
    let mut vec = input_str.split(",").collect::<Vec<&str>>();
    for i in vec.iter() {
        set.insert(i);
    }
    set.len()
}
