use std::{io::{self, Write, stdout}, process};
use hardware_query::{SystemOverview};
use crate::ewutrm_lib;
use std::fs;
use std::env;
use std::path::Path;

pub fn _COM_help() -> String{
    return r#"

──────────────────────────────
    WINDOWS COMMAND HELP
──────────────────────────────

[X] Network Commands
  ─────────────────────────
  arp       → Displays or modifies the IP-to-Physical address table
              Arguments: -a | -d | -s
  ipconfig  → Displays IP configuration
              Arguments: /all | /release | /renew
  ping      → Tests network connectivity
              Arguments: <host> | -n <count>

[A] File & Directory Commands
  ─────────────────────────
  cd        → Displays or changes current directory
              Arguments: <path>
  dir       → Lists files and directories
              Arguments: /w | /p | /s
  mkdir     → Creates a new directory
              Arguments: <path>
  rd        → Removes a directory
              Arguments: <path> | /s
  del       → Deletes files
              Arguments: <file> | /p
  ren       → Renames file or directory
              Arguments: <old> <new>
  move      → Moves files
              Arguments: <source> <dest>
  xcopy     → Copies files and directories
              Arguments: <source> <dest> | /s | /e | /y

[S] System Commands
  ─────────────────────────
  shutdown  → Shuts down or restarts PC
              Arguments: /s | /r | /t <sec>
  tasklist  → Displays running processes
              Arguments: /s <computer> | /fi <filter>
  taskkill  → Terminates tasks
              Arguments: /PID <id> | /IM <name> | /F
  set       → Displays or sets environment variables
              Arguments: <var>=<val> | <var>

[P] Utilities
  ─────────────────────────
  echo      → Displays messages or toggles echo
              Arguments: on | off | <text>
  cls       → Clears the terminal screen
  exit      → Exits the command prompt
  break     → Sets or clears CTRL+C checking
              Arguments: on | off
  chkdsk    → Checks disk for errors
              Arguments: /f | /r
  gpupdate  → Refreshes Group Policy settings
              Arguments: /force
  powershell→ Launches PowerShell
              Arguments: -Command <cmd>




    "#.to_string();
}


pub fn _COM_exit() -> String{
    process::exit(0);
}

pub fn _COM_clear_cli() -> String{
    match process::Command::new("cmd").args(&["/C", "cls"]).status() {
        Ok(_) => return String::from("OK!"),
        Err(_) => return String::from("ERR!")
    }
    
}


pub fn _COM_print_to_screen(input : &str, linetf : bool) -> (){
    if (linetf == true) {
        println!("{}", input);
    } else if (linetf == false){
        print!("{}", input);
        stdout().flush().unwrap();
    } else {
        println!("{}", input);
    }
}


pub fn _COM_cd(cd : String) -> String {
    let path = Path::new(&cd);

    if !path.exists() {
        return format!("ERR: '{}' diye bir klasör yok", cd);
    }

    print!("cc : {:?}", path);

    match env::set_current_dir(path) {
        Ok(_) => format!("OK: {}", cd),
        Err(e) => format!("ERR: {}", e),
    }
}

pub fn _COM_list() -> String{
    let path = std::env::current_dir().unwrap();
    let entries = fs::read_dir(path).expect("Error! Path Reading Error.");
    let mut folders = Vec::new();
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let name = entry.file_name().into_string().unwrap();

        if metadata.is_dir() {
            folders.push(name);
        } else {
            files.push(name);
        }
    }

    for folder in folders {
        ewutrm_lib::_LIBFUNC_print("folders_color", folder);
    }

    for file in files {
        ewutrm_lib::_LIBFUNC_print("files_color", file);
    }

    return String::new();
    

}