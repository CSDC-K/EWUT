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