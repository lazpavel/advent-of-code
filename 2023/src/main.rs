mod challenges;
mod utils;

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
}

fn day_6() {
    println!();
    println!("Day 6 - Boat Race");
    println!("===================================");
    println!("Input file: ./assets/boat_race.txt");

    let boat_race_input_path  = std::path::Path::new("./assets/boat_race.txt");
    let boat_race_result  = challenges::boat_race::calculate(boat_race_input_path);

    println!("Boat Race result: {}", boat_race_result);
}

fn day_5() {
    println!();
    println!("Day 5 - Almanac Map");
    println!("===================================");
    println!("Input file: ./assets/almanac_map.txt");

    let almanac_map_input_path = std::path::Path::new("./assets/almanac_map.txt");
    let almanac_map_result = challenges::almanac::calculate_almanac_map(almanac_map_input_path);

    println!("Almanac Map result: {}", almanac_map_result);
}

fn day_4() {
    println!();
    println!("Day 4 - Scratchcards");
    println!("===================================");
    println!("Input file: ./assets/scratchcards.txt");

    let scratchcards_input_path = std::path::Path::new("./assets/scratchcards.txt");
    let scratchcards_result =
        challenges::scratchcards::calculate_scratchcards(scratchcards_input_path);

    println!(
        "Scratchcards result: {} {}",
        scratchcards_result.0, scratchcards_result.1
    );
}

fn day_3() {
    println!();
    println!("Day 3 - Gear Ratios");
    println!("===================================");
    println!("Input file: ./assets/gear_ratios.txt");

    let gear_ratios_input_path = std::path::Path::new("./assets/gear_ratios.txt");
    let gear_ratios = challenges::gear_ratios::calculate_gear_ratios(gear_ratios_input_path);
    let advanced_gear_ratios =
        challenges::gear_ratios::calculate_advanced_gear_ratios(gear_ratios_input_path);

    println!(
        "The Gear Ratios result: {} {}",
        gear_ratios, advanced_gear_ratios
    );
}

fn day_2() {
    println!();
    println!("Day 2 - Cube Conundrum");
    println!("===================================");
    println!("Input file: ./assets/cube_conundrum.txt");

    let cube_conundrum_input_path = std::path::Path::new("./assets/cube_conundrum.txt");
    let cube_conundrum_config = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();
    let cube_conundrum_result = challenges::cube_conundrum::calculate_cube_conundrum(
        cube_conundrum_input_path,
        cube_conundrum_config,
    );

    println!(
        "Cube Conundrum result: {} {}",
        cube_conundrum_result.0, cube_conundrum_result.1
    );
}

fn day_1() {
    println!("Day 1 - Trebuchet");
    println!("===================================");
    println!("Input file: ./assets/trebuchet.txt");

    let trebuchet_input_path = std::path::Path::new("./assets/trebuchet.txt");
    let trebuchet_result = challenges::trebuchet::calculate_trebuchet(trebuchet_input_path);

    println!("Trebuchet result: {}", trebuchet_result);
}
