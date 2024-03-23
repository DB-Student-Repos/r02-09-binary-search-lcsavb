pub fn find(array: &[i32], key: i32) -> Option<usize> {
    
    if array.is_empty() {
        return None;
    }


    let middle_index = array.len() / 2;
       
    if key == array[middle_index] {
        return Some(middle_index);
    } else if key == array[0] {
        return Some(0);
    } else if key == array[array.len() - 1] {
        return Some(array.len() - 1);
    } else if key < array[0] || key > array[array.len() - 1] {
        return None;
    }


    let (left_array, right_array) = array.split_at(middle_index);

    let left_array_cut = left_array[left_array.len() - 1];
    let right_array_cut = right_array[0];

    if key > left_array_cut {
        return find(right_array, key)
    } else {
        return find(left_array, key)
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