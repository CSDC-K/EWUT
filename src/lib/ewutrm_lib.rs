/// TERMINAL OUTPUT SYSTEM LIBRARY
/// this library is for basic and not input required functions.
/// LIB : BASIC_OUTPUT_TERMINAL_FUNCTIONS
/// VER : 0.1


use std::io;
use std::fs;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct EWUT_config {
    // theme configs
    theme_name : String,
    
    window_opacity : u8,

    term_ascii : String,
    term_ascii_color : [u8; 3],

    input_str : String,
    input_str_color : [u8; 3],

    return_ok_color : [u8; 3],
}

pub fn _LIBFUNC_print_ascii_to_term(){
    let ascii_conf = _event_load_configs();
    let [term_r, term_g, term_b] = ascii_conf.term_ascii_color;
    println!(r#"{}"#, ascii_conf.term_ascii.truecolor(term_r, term_g, term_b));
}

pub fn _LIBFUNC_print(type_of_print : &str, print_content : String) {
    let print_conf = _event_load_configs();

    let print_type = match type_of_print {
        "return_ok_color" => print_conf.return_ok_color,
        _ => [255, 255, 255]
    };

    let [r, g, b] = print_type;

    println!("{}", print_content.truecolor(r, g, b));
}

fn _event_load_configs() -> EWUT_config{
    let config_data = fs::read_to_string("data\\conf\\EWUT.toml").unwrap();
    let rtn = toml::from_str::<EWUT_config>(&config_data).unwrap();
    rtn
}
