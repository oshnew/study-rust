fn main() {
    println!("## Start=== binary search");

    let arr = [
        1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210, 222,
    ];
    let target: i32 = 47;
    if let Some(result) = iterative(&arr, arr.len(), &target) {
        println!("{} found at index {}", target, result);
    } else {
        println!("{} not found.", target);
    }

    println!("## End=== binary search");
}

//https://dev-notes.eu/2020/03/Binary-Search-in-Rust/
// Binary search, iterative.
// Arguments: Array slice, array length, target integer to look up.
pub fn iterative(a: &[i32], len: usize, target_value: &i32) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
        }
    }
    None
}
