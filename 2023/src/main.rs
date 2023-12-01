pub mod trebuchet;

fn main() {
    day_1();
    day_2();
}

fn day_2() {
    todo!()
}

fn day_1() {
    println!("Day 1 - Trebuchet");
    println!("===================================");
    println!("Input file: ./assets/trebuchet.txt");
  
    let trebuchet_input_path = std::path::Path::new("./assets/trebuchet.txt");
    let trebuchet_result = trebuchet::calculate_trebuchet(trebuchet_input_path);

    println!("Trebuchet result: {}", trebuchet_result);
}
