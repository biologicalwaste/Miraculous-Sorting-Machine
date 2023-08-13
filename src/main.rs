use std::array;
fn main() {
    // Init the array and generate a random sequence to fill it
    let rnd_array = generate_rndarray();

    //Testing; print the whole array.
    for i in 0..63 {
        let current_display = rnd_array[i];
        println!("{current_display}")
    }
}

fn generate_rndarray() -> [i8; 64] {
    let mut a: [i8; 64] = [0; 64];
    for i in 0..63 {
        // let mut a: [i8; 64] = [0; 64];
    }

    return a;
}
