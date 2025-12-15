/// TERMINAL OUTPUT SYSTEM LIBRARY
/// this library is for basic and not input required functions.
/// LIB : BASIC_OUTPUT_TERMINAL_FUNCTIONS
/// VER : 0.1


use std::io;
use std::fs;
use std::path;
use dirs;
use std::process;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use colored::Colorize;
use std::path::Path;
use serde::{Deserialize, Serialize};
use hardware_query::SystemOverview;
use hardware_query::HardwarePresets;
use std::env;

use crate::lib::ewutcom_lib;




const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";


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

    folders_color : [u8; 3],
    files_color : [u8; 3],

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
        "folders_color" => print_conf.folders_color,
        "files_color" => print_conf.files_color,
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


fn make_progress_bar(score: u8) -> String {
    let filled_len = (score as f32 / 10.0).round() as usize;
    let empty_len = 10usize.saturating_sub(filled_len);
    
    let filled = "█".repeat(filled_len);
    let empty = "·".repeat(empty_len);
    
    let color = if score < 50 { RED } else if score < 80 { YELLOW } else { GREEN };
    
    format!("{}{}{}{} {}", color, filled, RESET, empty, score)
}

pub fn _LIBFUNC_pc_score() {

    let ai_assessment = HardwarePresets::ai_assessment().unwrap();
    let gaming_assessment = HardwarePresets::gaming_assessment().unwrap();
    let dev_assessment = HardwarePresets::developer_assessment().unwrap();

    println!();
    println!("{}   HARDWARE SCORES & STATS{}", BOLD, RESET);
    println!("   -----------------------");

    // --- AI SECTION ---
    println!("{}🤖 AI Capabilities:{}", CYAN, RESET);
    println!(
        "   {:<15} {}", 
        "AI Score:", 
        make_progress_bar(ai_assessment.ai_score as u8)
    );

    let frameworks_str = ai_assessment.frameworks
            .iter()
            .map(|f| format!("{:?}", f)) 
            .collect::<Vec<String>>() 
            .join(", ");
    println!("   {:<15} {}", "Frameworks:", frameworks_str);
    println!();

    // --- GAMING SECTION ---
    println!("{}🎮 Gaming Performance:{}", CYAN, RESET);
    println!(
        "   {:<15} {}", 
        "Gaming Score:", 
        make_progress_bar(gaming_assessment.gaming_score as u8)
    );

    println!("   {:<15} {:?}", "Rec. Settings:", gaming_assessment.recommended_settings);
    println!();

    // --- DEV SECTION ---
    println!("{}🛠️  Dev Environment:{}", CYAN, RESET);
    println!("   {:<15} {:?}", "Build Perf:", dev_assessment.dev_score);
    
    println!();
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

    let confdir = env::var("USERPROFILE").expect("Path Error") + "\\AppData\\Roaming\\EWUT\\";


    let config_data = fs::read_to_string(confdir + "data\\conf\\EWUT.toml").unwrap();
    let rtn = toml::from_str::<EWUT_config>(&config_data).unwrap();
    rtn
}

