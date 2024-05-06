// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::time::Duration;
use rdev::{grab, Event, EventType, Key};
use sysinfo::System;
use tauri::{Manager, command};
use tts::Tts;

fn key_down_to_string(event_type: EventType) -> Option<&'static str> {
    let key_str = match event_type {
        EventType::KeyPress(Key::F1) => Some("F1"),
        EventType::KeyPress(Key::F2) => Some("F2"),
        EventType::KeyPress(Key::F3) => Some("F3"),
        EventType::KeyPress(Key::F4) => Some("F4"),
        EventType::KeyPress(Key::F5) => Some("F5"),
        EventType::KeyPress(Key::F6 ) => Some("F6"),
        EventType::KeyPress(Key::F7) => Some("F7"),
        EventType::KeyPress(Key::F8) => Some("F8"),
        EventType::KeyPress(Key::F9) => Some("F9"),
        EventType::KeyPress(Key::F10) => Some("F10"),
        EventType::KeyPress(Key::F11) => Some("F11"),
        EventType::KeyPress(Key::F12) => Some("F12"),
        _ => None,
    };
    key_str
}

fn key_up_to_string(event_type: EventType) -> Option<&'static str> {
    let key_str = match event_type {
        EventType::KeyRelease(Key::F1) => Some("F1"),
        EventType::KeyRelease(Key::F2) => Some("F2"),
        EventType::KeyRelease(Key::F3) => Some("F3"),
        EventType::KeyRelease(Key::F4) => Some("F4"),
        EventType::KeyRelease(Key::F5) => Some("F5"),
        EventType::KeyRelease(Key::F6 ) => Some("F6"),
        EventType::KeyRelease(Key::F7) => Some("F7"),
        EventType::KeyRelease(Key::F8) => Some("F8"),
        EventType::KeyRelease(Key::F9) => Some("F9"),
        EventType::KeyRelease(Key::F10) => Some("F10"),
        EventType::KeyRelease(Key::F11) => Some("F11"),
        EventType::KeyRelease(Key::F12) => Some("F12"),
        _ => None,
    };
    key_str
}

#[command]
fn key_down(key: &str, window: tauri::Window) {
    window.emit("key-down", key).unwrap();
}

#[command]
fn key_up(key: &str, window: tauri::Window) {
    window.emit("key-up", key).unwrap();
}

#[command]
fn close_process_by_name(process_name: &str) {
    println!("结束进程 {}", process_name);
    let s = System::new_all(); // 创建一个System对象，包含所有系统信息
    for (_pid, process) in s.processes() {
        if process.name() == process_name {
            process.kill();
            println!("you killed {}", process_name);
            return;
        }
    }
    println!("not found process {}", process_name);
}

impl Future for Tts {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let tts = self.get_mut();
        match tts.on_utterance_end(cx) {
            Poll::Ready(Ok(())) => {
                tts.shutdown();
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                tts.shutdown();
                Poll::Ready(Err(e))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[command]
async fn speak_text(text: &str) -> Result<()> {
    // speaker::speak(text);
    let mut tts = Tts::default().expect("Failed to initialize TTS");
    let _ = tts.speak(text, false).await;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();
            tauri::async_runtime::spawn(async move {
                if let Err(error) = grab(move|event: Event| -> Option<Event>  {
                    if let Some(key) = key_down_to_string(event.event_type) {
                        println!("key down: {:?}", key);
                        key_down(key, window.clone());
                    } else if let Some(key) = key_up_to_string(event.event_type) {
                        println!("key up: {:?}", key);
                        key_up(key, window.clone());
                    }
                     Some(event)
                })  {
                    println!("error: {:?}", error);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_process_by_name,
            speak_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
