/// TERMINAL OUTPUT SYSTEM LIBRARY
/// this library is for basic and not input required functions.
/// LIB : BASIC_OUTPUT_TERMINAL_FUNCTIONS
/// VER : 0.1


use std::io;
use std::fs;
use std::process;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use hardware_query::SystemOverview;
use hardware_query::HardwarePresets;

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
    return_err_color : [u8; 3],

    start_up_type : String,
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
        "return_err_color" => print_conf.return_err_color,
        _ => [255, 255, 255]
    };

    let [r, g, b] = print_type;

    println!("{}", print_content.truecolor(r, g, b));
}

pub fn _DIRECTFUNC_change_title(title_arg : String) -> String{
    match process::Command::new("cmd").args(&["/C", "title", title_arg.as_str()]).status() {
        Ok(_) => return String::from("OK!"),
        Err(_) => return String::from("ERR!")
    }
}

pub fn _LIBFUNC_pc_health() -> (){
    let overview = SystemOverview::quick().unwrap();
    println!("{:?}", overview);
    
}

pub fn _LIBFUNC_pc_score() -> (){
    let ai_assessment = HardwarePresets::ai_assessment().unwrap();
    println!("AI Score: {}/100", ai_assessment.ai_score);
    println!("Supported Frameworks: {:?}", ai_assessment.frameworks);

    // For gaming applications  
    let gaming_assessment = HardwarePresets::gaming_assessment().unwrap();
    println!("Gaming Score: {}/100", gaming_assessment.gaming_score);
    println!("Recommended Settings: {:?}", gaming_assessment.recommended_settings);

    // For development environments
    let dev_assessment = HardwarePresets::developer_assessment().unwrap();
    println!("Build Performance: {:?}", dev_assessment.dev_score);
}

pub fn _LIBFUNC_getstartup(){
    let start_type = _event_load_configs();

    if (start_type.start_up_type == "ascii"){
        _LIBFUNC_print_ascii_to_term();
    } else if (start_type.start_up_type == "pc_health_info") {
        _LIBFUNC_pc_health();
    } else if(start_type.start_up_type == "pc_score"){
        _LIBFUNC_pc_score();
    }
}

fn _event_load_configs() -> EWUT_config{
    let config_data = fs::read_to_string("data\\conf\\EWUT.toml").unwrap();
    let rtn = toml::from_str::<EWUT_config>(&config_data).unwrap();
    rtn
}
