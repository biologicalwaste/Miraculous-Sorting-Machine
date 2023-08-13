use std::cmp::Ordering;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn sort_simple(mut a: [i8; 64]) -> [i8; 64] {
    //init variable to track how many swaps are completed
    loop {
        let mut ops = 0;
        for point in 0..63 {
            let current_first;
            let current_second;

            // get the first and second numbers to be sorted
            current_first = a[point];

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

pub fn bogo_sort(mut a: [i8; 64]) -> [i8; 64] {
    
    let mut rng = thread_rng();

    loop {
        let mut ops = 0;
        for point in 0..63 {
            let current_first;
            let current_second;

            // get the first and second numbers to be sorted
            current_first = a[point];

            current_second = a[point + 1];

            match current_first.cmp(&current_second) {
                // if the first value is greater than the second value, swap em
                Ordering::Greater => {
                    a.shuffle(&mut rng);

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

        if ops == 0 {
            break;
        }
    }

    return a;
}