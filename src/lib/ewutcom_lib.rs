use std::path::Path;
use std::{env, fs};

pub fn _COM_help() -> String {
    r#"
 ╔══════════════════════════════════╗
 ║      AVAILABLE COMMANDS          ║
 ╚══════════════════════════════════╝
 [Filesystem]
   cd <path>    Change directory
   ls           List files and folders
 
 [System]
   clear        Clear terminal history
   exit         Close application
   title <text> Change window title
   
 [Utils]
   print <text> Echo text to screen
   ascii        Show banner
    "#.to_string()
}

pub fn _COM_exit() -> String {
    "__EXIT_SIGNAL__".to_string()
}

pub fn _COM_clear_cli() -> String {
    "__CLEAR_SIGNAL__".to_string()
}

pub fn _COM_print_to_screen(input: &str, _linetf: bool) -> String {
    input.to_string()
}

pub fn _COM_cd(cd: String) -> String {
    let path = Path::new(&cd);
    if !path.exists() {
        return format!("❌ Error: Path '{}' not found.", cd);
    }
    match env::set_current_dir(path) {
        Ok(_) => format!("📂 Directory changed to: {}", cd),
        Err(e) => format!("❌ Error changing directory: {}", e),
    }
}

pub fn _COM_list() -> String {
    let path = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
    
    let mut output = String::new();
    output.push_str(&format!("Directory content of: {:?}\n\n", path));

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            let meta = entry.metadata();
            
            // Modern icons
            let prefix = if meta.map(|m| m.is_dir()).unwrap_or(false) {
                "📁 "
            } else {
                "📄 "
            };
            
            output.push_str(&format!("{}{}\n", prefix, name));
        }
    } else {
        output.push_str("❌ Error reading directory contents.");
    }

    output
}