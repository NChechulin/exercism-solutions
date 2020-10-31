pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let mut left: usize = 0;
    let mut right: usize = array.len();

    while right - left > 1 {
        let mid = (right + left) / 2;

        if array[mid] > key {
            right = mid;
        } else {
            left = mid;
        }
    }

    if array[left] == key {
        return Some(left);
    } else {
        return None;
    }
    // unimplemented!(
    //     "Using the binary search algorithm, find the element '{}' in the array '{:?}' and return its index.",
    //     key,
    //     array
    // );
}
