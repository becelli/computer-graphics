#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



#[tauri::command]
fn increment_counter(counter: u64) -> u64 {
    counter * 2
}

// #[tauri::command]
// fn decrement_counter(counter: i32) -> i32 {
//     counter / 2 
// }



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![increment_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
