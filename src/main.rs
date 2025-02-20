use clap::Command;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct TimerData {
    total_spent: u64,
    current_start: Option<u64>,
    paused_elapsed: u64,
}

impl TimerData {
    fn new() -> Self {
        Self {
            total_spent: 0,
            current_start: None,
            paused_elapsed: 0,
        }
    }
}

fn get_timer_path() -> PathBuf {
    if let Ok(path) = env::var("DEVTIMER_PATH") {
        return PathBuf::from(path);
    }
    #[cfg(target_os = "windows")]
    let default_path =
        dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("C:/Users/Public/devtimer"));
    #[cfg(not(target_os = "windows"))]
    let default_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/tmp/devtimer"));

    let path = default_path.join("timer.json");
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create config directory");
    }
    path
}

fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{} seconds", seconds)
    } else if seconds < 3600 {
        format!("{} minutes", seconds / 60)
    } else {
        format!(
            "{} hours and {} minutes",
            seconds / 3600,
            (seconds % 3600) / 60
        )
    }
}

fn load_timer() -> TimerData {
    let timer_path = get_timer_path();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(timer_path)
        .expect("Failed to open or create timer file");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| TimerData::new())
}

fn save_timer(timer: &TimerData) {
    let timer_path = get_timer_path();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(timer_path)
        .expect("Failed to write to timer file");
    file.write_all(serde_json::to_string(timer).unwrap().as_bytes())
        .unwrap();
}

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let matches = Command::new("devtimer")
        .version("1.0")
        .about("Track time spent coding on a project")
        .subcommand(Command::new("start").about("Starts the timer"))
        .subcommand(Command::new("break").about("Pauses the timer and shows current session time"))
        .subcommand(Command::new("back").about("Resumes the timer"))
        .subcommand(Command::new("stop").about("Stops the timer and adds session time"))
        .subcommand(Command::new("spent").about("Shows total time spent coding"))
        .get_matches();

    let mut timer = load_timer();
    let now = get_now();

    match matches.subcommand_name() {
        Some("start") => {
            if timer.current_start.is_some() || timer.paused_elapsed > 0 {
                println!(
                    "A session is already in progress. Use 'stop' to finish the current session."
                );
            } else {
                timer.current_start = Some(now);
                println!("Timer started.");
            }
        }
        Some("break") => {
            if let Some(start) = timer.current_start {
                let elapsed = now.saturating_sub(start);
                timer.paused_elapsed += elapsed;
                timer.current_start = None;
                println!(
                    "Paused. Current session time: {}",
                    format_duration(timer.paused_elapsed)
                );
            } else {
                println!("Timer is not running.");
            }
        }
        Some("back") => {
            if timer.current_start.is_none() && timer.paused_elapsed > 0 {
                timer.current_start = Some(now);
                println!("Resumed.");
            } else if timer.current_start.is_some() {
                println!("Timer is already running.");
            } else {
                println!("No paused session to resume. Use 'start' to begin a session.");
            }
        }
        Some("stop") => {
            let session_time = if let Some(start) = timer.current_start {
                now.saturating_sub(start) + timer.paused_elapsed
            } else if timer.paused_elapsed > 0 {
                timer.paused_elapsed
            } else {
                0
            };
            if session_time > 0 {
                timer.total_spent += session_time;
                println!("Stopped. Session time: {}", format_duration(session_time));
                timer.current_start = None;
                timer.paused_elapsed = 0;
                save_timer(&timer);
            } else {
                println!("Timer is not running.");
            }
        }
        Some("spent") => {
            println!("Total time spent: {}", format_duration(timer.total_spent));
        }
        _ => {
            println!("Invalid command. Use 'devtimer --help' for usage info.");
        }
    }
    save_timer(&timer);
}
