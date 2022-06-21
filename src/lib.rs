mod generator;

pub fn print_random_num() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
