pub fn find(array: &[i32], key: i32) -> Option<usize> {
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

