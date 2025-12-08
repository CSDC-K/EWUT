use core::str;
use std::io;
use std::fs;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use colored::Colorize;
use serde::{Deserialize, Serialize};


use crate::lib::ewutrm_lib;
use crate::lib::ewutsearch_lib;
use crate::lib::ewutcom_lib;

mod lib{
    pub mod ewutrm_lib;
    pub mod ewutsearch_lib;
    pub mod ewutcom_lib;
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

    return_ok_color : [u8; 3],
    return_err_color : [u8; 3],

    folders_color : [u8; 3],
    files_color : [u8; 3],

    cli_title : String,

    start_up_type : String,
}


fn main(){
    // load ThemeConfigs
    let ewut_config = _event_load_configs();
    let [inp_r, inp_g, inp_b] = ewut_config.input_str_color;
    let mut input_command : String = String::new();


    ewutrm_lib::_DIRECTFUNC_change_title(ewut_config.cli_title); // auto-title
    //ewutrm_lib::_LIBFUNC_print_ascii_to_term(); // first time ascii write
    ewutrm_lib::_LIBFUNC_getstartup();
    
    loop { // loop for the inputs
        input_command.clear();
        print!("{}", ewut_config.input_str.truecolor(inp_r, inp_g, inp_b));
        stdout().flush().unwrap(); // flush memory for print
        stdin().read_line(&mut input_command).expect("Error!");
        input_command = input_command.trim().to_string();

        match ewutsearch_lib::_SEARCH_commandsearch(&input_command){
            Ok(okturn) => ewutrm_lib::_LIBFUNC_print("return_ok_color", okturn),
            Err(errturn) => ewutrm_lib::_LIBFUNC_print("return_err_color", errturn),
        }
        
    }

}


fn _event_load_configs() -> EWUT_config{
    let config_data = fs::read_to_string("data\\conf\\EWUT.toml").unwrap();
    let rtn = toml::from_str::<EWUT_config>(&config_data).unwrap();
    rtn
}
