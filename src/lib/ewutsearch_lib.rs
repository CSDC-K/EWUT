/// FUNCTION CALLER LIBRARY
/// event history : 
/// main (command_input) -> search_lib (command_search) -> com_lib (command_run) -> return
/// LIB : SEARCH
/// VER : 0.1

use std::collections::HashMap;
use crate::ewutcom_lib;
use crate::ewutrm_lib;


pub fn _SEARCH_commandsearch(command_input : &str) -> Result<String, String>{
    let mut commands : HashMap<&str, fn() -> String> = HashMap::new(); // creation of commandhash list

    // insert fn to hash.
    commands.insert("/help", _CALL_help);
    commands.insert("/exit", _CALL_exit);
    commands.insert("/clear", _CALL_clear_cli);
    commands.insert("/0", _CALLDIRECT_asciiart);

    if let Some(rest) = command_input.strip_prefix("/title "){
        let comreturn = _CALLDIRECT_change_title(rest);
        Ok(comreturn) // debug return
    }

    else if let Some(com) = commands.get(command_input){
        let comreturn = com();
        Ok(comreturn) // debug return
    } else {
        Err(format!("NO COMMAND FOUND"))
    }

}


// command caller functions


fn _CALL_help() -> String{
    ewutcom_lib::_COM_help()
}

fn _CALL_exit() -> String{
    ewutcom_lib::_COM_exit()
}

fn _CALL_clear_cli() -> String{
    ewutcom_lib::_COM_clear_cli()
}


fn _CALLDIRECT_asciiart() -> String{
    ewutrm_lib::_LIBFUNC_print_ascii_to_term();
    String::from("")
}

fn _CALLDIRECT_change_title(input : &str) -> String{
    ewutrm_lib::_DIRECTFUNC_change_title(input.to_string())
}