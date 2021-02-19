// rust lib
use std::{
    env, path::Path,
    collections::BTreeMap
};
// svg_generator
mod parse;
use rustviz_lib::svg_frontend::{
    svg_generation, utils
};
use rustviz_lib::data::{
    VisualizationData
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // verify usage
    if args.len() != 2 {
        println!(r"Usage Error: cargo run <filename>"); 
        return;
    }

    let filename = format!("../svg_generator/examples/{}/main.rs", &args[1]);
    if !Path::new(&filename).is_file() {
        panic!("Example source file not found in {}!", &filename);
    }
    let contents = utils::read_file_to_string(filename).unwrap(); // read to single string

    /* ******************************************
            --- Parse main.rs file ---
    ****************************************** */
    let var_map = parse::extract_vars_to_map(&contents);
    let events = parse::extract_events_to_string(&contents);
    /* ******************************************
            --- Build VisualizationData ---
    ****************************************** */
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
        preprocess_external_events: Vec::new(),
        event_line_map: BTreeMap::new()
    };

    // TODO: match events to ExternalEvents and implement line numbers
    parse::add_events(&mut vd, events);

    /* ******************************************
            --- Render SVG images ---
    ****************************************** */
    let input_path = format!("../svg_generator/examples/{}/input/", &args[1]);
    let output_path = format!("../svg_generator/examples/{}/", &args[1]);
    svg_generation::render_svg(&input_path, &output_path, &mut vd);
}
