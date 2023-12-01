pub mod trebuchet;

fn main() {
    println!("Hello, world!");

    let trebuchet_input_path = std::path::Path::new("./assets/trebuchet.txt");
    let trebuchet_result = trebuchet::calculate_trebuchet(trebuchet_input_path);

    println!("Trebuchet result: {}", trebuchet_result);
}
