#![warn(clippy::all, clippy::pedantic)]

use std::env;
use std::fs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use rdev::{grab, Event, EventType, Key};
use xcap::Monitor;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let screen_dir: String = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;

    path.push(&screen_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |event| callback(event, &screen_dir)) {
        println!("error: {error:?}");
    }

    Ok(())
}

fn callback(event: Event, screen_dir: &str) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::KeyP) => {
            make_screen(screen_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screen_dir: &str) {
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        match monitor.capture_image() {
            Ok(image) => {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let monitor_id = monitor.id();
                let file_path = format!("{screen_dir}/screenshot_{monitor_id:?}_{ts}.png");

                match image.save(&file_path) {
                    Ok(_) => println!("Скриншот успешно сохранен: {file_path}"),
                    Err(error) => println!("Не удалось сохранить файл: {error:?}")
                }
            },
            Err(error) => {
                println!("Не удалось сделать скриншот: {error:?}");
            }
        }
    }
}