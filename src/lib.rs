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

// I am going think how to solve the errors:
// failures:
//     a_value_larger_than_the_arrays_largest_value_is_not_included
//     a_value_smaller_than_the_arrays_smallest_value_is_not_included
//     finds_a_value_at_the_end_of_an_array
//     finds_a_value_in_an_array_of_even_length
//     finds_a_value_in_an_array_of_odd_length
//     identifies_that_a_value_is_not_included_in_the_array
//     nothing_is_found_when_the_left_and_right_bounds_cross
//     nothing_is_included_in_an_empty_array

// test result: FAILED. 4 passed; 8 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s