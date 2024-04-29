// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use rdev::{grab, Event, EventType, Key};
use sysinfo::System;
use tauri::Manager;

fn close_process_by_name(process_name: &str) {
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

fn main() {
    let callback = |event: Event| -> Option<Event> {
        if let EventType::KeyPress(Key::F6) = event.event_type {
            println!("Consuming and cancelling CapsLock");
            let process_name = "WeChat.exe";
            // close_process_by_name(process_name);
            None // CapsLock is now effectively disabled
        } else {
            Some(event)
        }
    };

    tauri::async_runtime::spawn(async move {
        println!("在任意界面输入F5，结束 DN 进程");
        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error)
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // window
            //     .set_ignore_cursor_events(true)
            //     .expect("error setting ignore cursor events");
            // window.set_always_on_top(true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
