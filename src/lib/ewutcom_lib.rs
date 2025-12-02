use std::process;

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

