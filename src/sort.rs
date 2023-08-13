use std::cmp::Ordering;

pub fn sort_simple(mut a: [i8; 64]) -> [i8; 64] {
    //init variable to track how many swaps are completed
    loop {
        let mut ops = 0;
        for point in 0..a.len() {
            let length = a.len() - 1;
            let current_first;
            let current_second;

            current_first = a[point];
            if point == length {
                current_second = 0;
                break;
            }

            current_second = a[point + 1];

            match current_first.cmp(&current_second) {
                //if the first value is greater than the second value, swap em
                Ordering::Greater => {
                    let current_first_prime = current_second;
                    let current_second_prime = current_first;
                    a[point] = current_first_prime;
                    a[point + 1] = current_second_prime;
                    ops += 1;
                }
                Ordering::Less => {
                    continue;
                }
                Ordering::Equal => println!("Something has gone wrong! Check your code!"),
            }

            // testing prints
            // println!("The first number is {:?}! The second number is {:?}!", current_first, current_second);
        }

        if ops == 0 {
            break;
        }
    }

    return a;
}
