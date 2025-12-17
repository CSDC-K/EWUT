use std::collections::HashMap;
use crate::ewutcom_lib;
use crate::ewutrm_lib;

pub fn _SEARCH_commandsearch(command_input: &str) -> Result<String, String> {
    let mut commands: HashMap<&str, fn() -> String> = HashMap::new();

    commands.insert("help", _CALL_help);
    commands.insert("exit", _CALL_exit);
    commands.insert("clear", _CALL_clear_cli);
    commands.insert("ascii", _CALLDIRECT_asciiart);
    commands.insert("ls", _CALL_list);

    // Using "start_with" for commands with arguments
    if let Some(rest) = command_input.strip_prefix("title ") {
        Ok(_CALLDIRECT_change_title(rest))
    } else if let Some(rest) = command_input.strip_prefix("print ") {
        Ok(_CALL_print_to_screen(rest, false))
    } else if let Some(rest) = command_input.strip_prefix("cd ") {
        Ok(_CALL_cd(rest))
    } 
    // Exact match
    else if let Some(com) = commands.get(command_input) {
        Ok(com())
    } 
    // Not found
    else {
        Err(format!("Unknown command: '{}'. Type 'help' for info.", command_input))
    }
}

fn _CALL_help() -> String { ewutcom_lib::_COM_help() }
fn _CALL_exit() -> String { ewutcom_lib::_COM_exit() }
fn _CALL_clear_cli() -> String { ewutcom_lib::_COM_clear_cli() }
fn _CALLDIRECT_asciiart() -> String { ewutrm_lib::_LIBFUNC_print_ascii_to_term() }
fn _CALLDIRECT_change_title(input: &str) -> String { ewutrm_lib::_DIRECTFUNC_change_title(input.to_string()) }
fn _CALL_print_to_screen(input: &str, linetf: bool) -> String { ewutcom_lib::_COM_print_to_screen(input, linetf) }
fn _CALL_list() -> String { ewutcom_lib::_COM_list() }
fn _CALL_cd(input: &str) -> String { ewutcom_lib::_COM_cd(input.into()) }