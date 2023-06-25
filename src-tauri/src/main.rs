// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use postgrest::Postgrest;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn all_reviews() -> String {
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co");

    let response = client
        .from("reviews")
        .eq("jailed", "yes")
        .select("*")
        .execute()
        .await;

    let body = response
        .expect("Unable to unwrap reviews")
        .text()
        .await
        .unwrap();

    body
}

#[tauri::command]
fn accept_review() {
    println!("Accept review");
}

#[tauri::command]
fn reject_review() {
    println!("Reject review");
}

#[tauri::command]
async fn all_estimates() -> String {
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co");
    let response = client
        .from("estimates")
        .eq("completed", "no")
        .select("*")
        .execute()
        .await;

    let body = response
        .expect("Unable to unwrap estimates")
        .text()
        .await
        .unwrap();

    body
}

#[tauri::command]
fn complet_estimate() {
    println!("Complete estimate");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            all_reviews,
            accept_review,
            reject_review,
            all_estimates,
            complet_estimate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
