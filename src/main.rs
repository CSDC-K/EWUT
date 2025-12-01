use std::io;
use std::fs;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, ResetColor},
    terminal::{Clear, ClearType},
};

use crate::lib::ewutrm_lib;

mod lib{
    pub mod ewutrm_lib;
}



#[derive(Debug, Deserialize, Serialize)]
struct EWUT_config {
    // theme configs
    theme_name : String,
    
    window_opacity : u8,

    term_ascii : String,
    term_ascii_color : [u8; 3],

    input_str : String,
    input_str_color : [u8; 3],
}


fn main(){
    // load ThemeConfigs
    let ewut_config = _event_load_configs();
    let [inp_r, inp_g, inp_b] = ewut_config.input_str_color;
    let [term_r, term_g, term_b] = ewut_config.term_ascii_color;
    execute!(stdout(), SetBackgroundColor(Color::Blue)).unwrap();
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    let mut input_command : String = String::new();


    //
    //println!(r#"{}"#, ewut_config.term_ascii.truecolor(term_r, term_g, term_b));

    ewutrm_lib::_LIBFUNC_print_ascii_to_term();
    print!("{}", ewut_config.input_str.truecolor(inp_r, inp_g, inp_b));
    stdout().flush().unwrap(); // flush memory for print
    stdin().read_line(&mut input_command).expect("Error!");
}


fn _event_load_configs() -> EWUT_config{
    let config_data = fs::read_to_string("data\\conf\\EWUT.toml").unwrap();
    let rtn = toml::from_str::<EWUT_config>(&config_data).unwrap();
    rtn
}
