/// FUNCTION CALLER LIBRARY
/// event history : 
/// main (command_input) -> search_lib (command_search) -> com_lib (command_run) -> return
/// LIB : SEARCH
/// VER : 0.1

use std::collections::HashMap;


pub fn _SEARCH_commandsearch(command_input : &str) -> Result<String, String>{
    let mut commands : HashMap<&str, fn() -> String> = HashMap::new(); // creation of commandhash list

    // insert fn to hash.
    commands.insert("/help", _CALL_help);
    commands.insert("/exit", _CALL_exit);

    if let Some(com) = commands.get(command_input){
        let comreturn = com();
        Ok(comreturn)
    } else {
        Err(format!("NO COMMAND FOUND"))
    }

}


// command caller functions


fn _CALL_help() -> String{
    return String::from("help is called.");
}

fn _CALL_exit() -> String{
    return String::from("exit is called.");
}
