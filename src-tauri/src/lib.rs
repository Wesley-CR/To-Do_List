// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[tauri::command]
fn add_task(task: String) {
    let mut tasks = TASKS.lock().unwrap();
    println!("Adding task: {}", task);
    tasks.push(task);
}

#[tauri::command]
fn get_tasks() -> Vec<String> {
    let tasks = TASKS.lock().unwrap();
    tasks.clone()
}

#[tauri::command]
fn check_task(index: i32){
    let mut tasks = TASKS.lock().unwrap();
    //eliminate the element at the index
    tasks.remove(index as usize);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![add_task, get_tasks, check_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}