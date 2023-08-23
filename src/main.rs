mod keycode;
mod play_sound;
mod start;

use std::env;
use config::{Config, File};

const ASCII_ART: &str =
    r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;

fn main() {
    let mut settings = Config::default();

    // Add a configuration file to load data from
    settings.merge(File::with_name("config.toml")).expect("Failed to load config file");

    let soundpack_name: String = settings.get("soundpack_name").unwrap();
    let volume: u16 = settings.get("volume").unwrap();

    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_folder = exe_path.parent().expect("Failed to get executable folder");

    let soundpack_folder = exe_folder.join("packs").join(soundpack_name);

    println!("{}", ASCII_ART);
    
    start::rustyvibes::start_rustyvibes(soundpack_folder.to_string_lossy().to_string(), volume);
}