mod challenges;
mod utils;
mod data_structures;

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
    day_09();
    day_10();

    day_11();
    day_12();
    // day_13();
    // day_14();
    day_15();
    // day_16();
    // day_17();
    // day_18();
    // day_19();
    day_20();
    day_21();
    day_23();
    day_24();
    day_25();
}

fn day_25() {
    println!();
    println!("Day 25 - Snow Overload");
    println!("===================================");
    println!("Input file: ./assets/snow_overload.txt");

    let input_path  = std::path::Path::new("./assets/snow_overload.txt");
    let result  = challenges::snow_overload::execute(input_path);

    println!("Snow Overload: {}", result);
}

fn day_24() {
    println!();
    println!("Day 24 - Hailstones");
    println!("===================================");
    println!("Input file: ./assets/hailstones.txt");

    let input_path  = std::path::Path::new("./assets/hailstones.txt");
    let result  = challenges::hailstones::execute(input_path);

    println!("Hailstones: {}", result);
}

fn day_23() {
    println!();
    println!("Day 24 - Long Walk");
    println!("===================================");
    println!("Input file: ./assets/long_walk.txt");

    let input_path  = std::path::Path::new("./assets/long_walk.txt");
    let result  = challenges::long_walk::execute(input_path);

    println!("Long Walk: {}", result);
}

fn day_21() {
    println!();
    println!("Day 21 - Step Counter");
    println!("===================================");
    println!("Input file: ./assets/step_counter.txt");

    let input_path  = std::path::Path::new("./assets/step_counter.txt");
    let result  = challenges::step_counter::execute(input_path);

    println!("Step Counter: {}", result);
}

fn day_20() {
    println!();
    println!("Day 20 - Aplenty");
    println!("===================================");
    println!("Input file: ./assets/aplenty.txt");

    let input_path  = std::path::Path::new("./assets/aplenty.txt");
    let result  = challenges::aplenty::execute(input_path);

    println!("Aplenty: {:?}", result);
}

fn day_15() {
    println!();
    println!("Day 15 - Lens Library");
    println!("===================================");
    println!("Input file: ./assets/lens_library.txt");

    let input_path  = std::path::Path::new("./assets/lens_library.txt");
    let result  = challenges::lens_library::execute(input_path);

    println!("Lens Library: {}", result);
}

fn day_12() {
    println!();
    println!("Day 12 - Hot Springs");
    println!("===================================");
    println!("Input file: ./assets/hot_springs.txt");

    let input_path  = std::path::Path::new("./assets/hot_springs.txt");
    let result  = challenges::hot_springs::execute(input_path);

    println!("Hot Springs: {}", result);
}

fn day_11() {
    println!();
    println!("Day 11 - Cosmic Expansion");
    println!("===================================");
    println!("Input file: ./assets/cosmic_expansion.txt");

    let input_path  = std::path::Path::new("./assets/cosmic_expansion.txt");
    let result  = challenges::cosmic_expansion::execute(input_path);

    println!("Pulse Propagation: {}", result);
}

fn day_10() {
    println!();
    println!("Day 10 - Pipe Maze");
    println!("===================================");
    println!("Input file: ./assets/pipe_maze.txt");

    let input_path  = std::path::Path::new("./assets/pipe_maze.txt");
    let result  = challenges::pipe_maze::execute(input_path);

    println!("Pipe Maze: {}", result);
}

fn day_09() {
    println!();
    println!("Day 9 - Mirage Maintenance");
    println!("===================================");
    println!("Input file: ./assets/mirage_maintenance.txt");

    let input_path  = std::path::Path::new("./assets/mirage_maintenance.txt");
    let result  = challenges::mirage::execute(input_path);

    println!("Mirage Maintenance: {}", result);
}

fn day_08() {
    println!();
    println!("Day 8 - Haunted Wasteland");
    println!("===================================");
    println!("Input file: ./assets/haunted_wasteland.txt");

    let input_path  = std::path::Path::new("./assets/haunted_wasteland.txt");
    let result  = challenges::haunted_wasteland::execute(input_path);

    println!("Haunted Wasteland: {}", result);
}

fn day_07() {
    println!();
    println!("Day 7 - Camel Cards");
    println!("===================================");
    println!("Input file: ./assets/camel_cards.txt");

    let input_path  = std::path::Path::new("./assets/camel_cards.txt");
    let result  = challenges::camel_cards::execute(input_path);

    println!("Camel Cards: {}", result);
}

fn day_06() {
    println!();
    println!("Day 6 - Boat Race");
    println!("===================================");
    println!("Input file: ./assets/boat_race.txt");

    let boat_race_input_path  = std::path::Path::new("./assets/boat_race.txt");
    let boat_race_result  = challenges::boat_race::calculate(boat_race_input_path);

    println!("Boat Race result: {}", boat_race_result);
}

fn day_05() {
    println!();
    println!("Day 5 - Almanac Map");
    println!("===================================");
    println!("Input file: ./assets/almanac_map.txt");

    let almanac_map_input_path = std::path::Path::new("./assets/almanac_map.txt");
    let almanac_map_result = challenges::almanac::calculate_almanac_map(almanac_map_input_path);

    println!("Almanac Map result: {}", almanac_map_result);
}

fn day_04() {
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

fn day_03() {
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

fn day_02() {
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

fn day_01() {
    println!("Day 1 - Trebuchet");
    println!("===================================");
    println!("Input file: ./assets/trebuchet.txt");

    let trebuchet_input_path = std::path::Path::new("./assets/trebuchet.txt");
    let trebuchet_result = challenges::trebuchet::calculate_trebuchet(trebuchet_input_path);

    println!("Trebuchet result: {}", trebuchet_result);
}
