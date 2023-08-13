use rand::thread_rng;
use rand::seq::SliceRandom;
mod sort;
fn main() {
    // Init the array and generate a random sequence to fill it
    let rnd_array = generate_rndarray();

    let sorted_array = sort::bogo_sort(rnd_array);

    println!("Array is {:?}", rnd_array);

    println!("Sorted array is {:?}", sorted_array);
}

fn generate_rndarray() -> [i8; 64] {
    //Create an random number generator and store it in rng
    let mut rng = thread_rng();
    //Create an array and fill it with 0s
    let mut a: [i8; 64] = [0; 64];
    for i in 0..63 {
        // Fill array with a sequence from 1 to 63
        a[i] = i as i8 + 1;
    }
    //Shuffle the array to make random values
    a.shuffle(&mut rng);

    return a;
}
