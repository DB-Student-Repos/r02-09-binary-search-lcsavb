use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let left = 0;
    let right = array.len();

    // Base case: if the array is empty, return None
    if array.is_empty() {
        return None;
    }

    // Recursive case
    let middle = left + (right - left) / 2;

    match array[middle].cmp(&key) {
        Ordering::Equal => Some(middle),
        Ordering::Less => find(&array[middle + 1..], key).map(|i| i + middle + 1),
        // the map method is used because the index is relative to the subarray, 
        // not the original array; it is hard to understand.
        Ordering::Greater => {
            if middle == 0 {
                None
            } else {
                find(&array[..middle], key)
            }
        }
    }
}




pub fn find_loop(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let middle = left + (right - left) / 2;

        if array[middle] == key {
            return Some(middle);
        } else if array[middle] < key {
            left = middle + 1;
        } else {
            if middle == 0 {
                return None; 
            }
            right = middle - 1;
        }
    }

    None
}


