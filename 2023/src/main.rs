mod trebuchet;
mod cube_conundrum;
mod input_utils;
mod gear_ratios;

fn main() {
    day_1();
    day_2();
    day_3();
}

fn day_3() {
    println!();
    println!("Day 3 - Gear Ratios");
    println!("===================================");
    println!("Input file: ./assets/gear_ratios.txt");
  
    let gear_ratios_input_path = std::path::Path::new("./assets/gear_ratios.txt");
    let gear_ratios = gear_ratios::calculate_gear_ratios(gear_ratios_input_path);
    let advanced_gear_ratios = gear_ratios::calculate_advanced_gear_ratios(gear_ratios_input_path);

    println!("The Great Escape result: {} {}", gear_ratios, advanced_gear_ratios);
}

fn day_2() {
    println!();
    println!("Day 2 - Cube Conundrum");
    println!("===================================");
    println!("Input file: ./assets/cube_conundrum.txt");
  
    let cube_conundrum_input_path = std::path::Path::new("./assets/cube_conundrum.txt");
    let cube_conundrum_config = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]
    .iter()
    .cloned()
    .collect();
    let cube_conundrum_result = cube_conundrum::calculate_cube_conundrum(cube_conundrum_input_path, cube_conundrum_config);

    println!("Cube Conundrum result: {} {}", cube_conundrum_result.0, cube_conundrum_result.1);
}

fn day_1() {
    println!("Day 1 - Trebuchet");
    println!("===================================");
    println!("Input file: ./assets/trebuchet.txt");
  
    let trebuchet_input_path = std::path::Path::new("./assets/trebuchet.txt");
    let trebuchet_result = trebuchet::calculate_trebuchet(trebuchet_input_path);

    println!("Trebuchet result: {}", trebuchet_result);
}
