use std::fs;
use std::env;
use serde::{Deserialize, Serialize};
use hardware_query::{SystemOverview, HardwarePresets};
use ratatui::style::Color;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub theme_name: String,
    pub window_opacity: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorConfig {
    pub primary_color: [u8; 3],
    pub secondary_color: [u8; 3],
    pub text_color: [u8; 3],
    pub success_color: [u8; 3],
    pub error_color: [u8; 3],
    pub input_color: [u8; 3],
    pub background_color: [u8; 3],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettingsConfig {
    pub start_up_type: String,
    pub term_ascii: String,
    pub prompt_symbol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EwutConfig {
    pub theme: ThemeConfig,
    pub colors: ColorConfig,
    pub settings: SettingsConfig,
}

// Helper to convert [u8; 3] to Ratatui Color
pub fn to_rgb(arr: [u8; 3]) -> Color {
    Color::Rgb(arr[0], arr[1], arr[2])
}

pub fn _event_load_configs() -> EwutConfig {
    let confdir = env::var("USERPROFILE").unwrap_or(".".to_string()) + "\\AppData\\Roaming\\EWUT\\";
    
    match fs::read_to_string(confdir + "data\\conf\\EWUT.toml") {
        Ok(data) => toml::from_str::<EwutConfig>(&data).unwrap_or_else(|_| _default_config()),
        Err(_) => _default_config(),
    }
}

fn _default_config() -> EwutConfig {
    EwutConfig {
        theme: ThemeConfig { theme_name: "Default".into(), window_opacity: 255 },
        colors: ColorConfig {
            primary_color: [0, 255, 255],
            secondary_color: [255, 0, 255],
            text_color: [255, 255, 255],
            success_color: [0, 255, 0],
            error_color: [255, 0, 0],
            input_color: [255, 255, 0],
            background_color: [0, 0, 0],
        },
        settings: SettingsConfig {
            start_up_type: "ascii".into(),
            term_ascii: "EWUT TERMINAL".into(),
            prompt_symbol: ">".into(),
        },
    }
}

pub fn _LIBFUNC_print_ascii_to_term() -> String {
    let conf = _event_load_configs();
    format!("\n{}\n", conf.settings.term_ascii)
}

pub fn _DIRECTFUNC_change_title(title_arg: String) -> String {
    let _ = std::process::Command::new("cmd").args(&["/C", "title", title_arg.as_str()]).status();
    String::from("System Window Title Updated.")
}

pub fn _LIBFUNC_pc_health() -> String {
    match SystemOverview::quick() {
        Ok(overview) => format!("{:#?}", overview),
        Err(_) => "Could not fetch system information.".to_string(),
    }
}

fn make_progress_bar(score: u8) -> String {
    let filled_len = (score as f32 / 10.0).round() as usize;
    let empty_len = 10usize.saturating_sub(filled_len);
    let filled = "█".repeat(filled_len);
    let empty = "░".repeat(empty_len);
    format!("{}{}", filled, empty) 
}

pub fn _LIBFUNC_pc_score() -> String {
    let mut output = String::new();
    
    if let (Ok(ai), Ok(gaming), Ok(dev)) = (
        HardwarePresets::ai_assessment(),
        HardwarePresets::gaming_assessment(),
        HardwarePresets::developer_assessment()
    ) {
        output.push_str("\n   HARDWARE PERFORMANCE METRICS\n   ────────────────────────────\n");
        output.push_str(&format!("🤖 AI Capability: {} ({})\n", make_progress_bar(ai.ai_score as u8), ai.ai_score));
        output.push_str(&format!("🎮 Gaming Power:  {} ({})\n", make_progress_bar(gaming.gaming_score as u8), gaming.gaming_score));
        output.push_str(&format!("🛠️ Dev Workflow:  {} ({})\n", make_progress_bar(dev.dev_score as u8), dev.dev_score));
    } else {
        output.push_str("Hardware analysis failed.\n");
    }
    
    output
}

pub fn _LIBFUNC_getstartup() -> String {
    let conf = _event_load_configs();

    if conf.settings.start_up_type == "ascii" {
        _LIBFUNC_print_ascii_to_term()
    } else if conf.settings.start_up_type == "pc_health_info" {
        _LIBFUNC_pc_health()
    } else if conf.settings.start_up_type == "pc_score" {
        _LIBFUNC_pc_score()
    } else {
        "EWUT Terminal System Ready.".to_string()
    }
}