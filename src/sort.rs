use std::cmp::Ordering;

pub fn sort_simple(mut a: [i8; 64]) -> [i8; 64] {
    //init variable to track how many swaps are completed
    loop {
        let mut ops = 0;
        for point in 0..a.len() {
            let length = a.len() - 1;
            let current_first;
            let current_second;

            // get the first and second numbers to be sorted
            current_first = a[point];

            // check if outside the range of the array
            if point == length {
                break;
            }

            current_second = a[point + 1];

            match current_first.cmp(&current_second) {
                // if the first value is greater than the second value, swap em
                Ordering::Greater => {
                    let current_first_prime = current_second;
                    let current_second_prime = current_first;
                    a[point] = current_first_prime;
                    a[point + 1] = current_second_prime;

                    // increment the amount of operations performed
                    ops += 1;
                }
                Ordering::Less => {
                    continue;
                }
                Ordering::Equal => {
                    continue;
                }
            }
        }

        // Test if the array is sorted by looking at how many swaps it performed
        if ops == 0 {
            break;
        }
    }

    return a;
}
