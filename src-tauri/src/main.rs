// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod functions;
pub mod atstypes;


#[tauri::command]
async fn sign_in(email: String, password: String, bar: String) -> bool {
    let creds = functions::get_admin(email, password, bar).await;
    
    creds
}

#[tauri::command]
async fn all_reviews() -> Vec<atstypes::RevOutInfo> {
    let reviews = functions::get_all_reviews().await;

    reviews
}

#[tauri::command]
async fn accept_review(revid: String) -> String {
    let accept_rev = functions::accept_review(revid).await;

    accept_rev
}

#[tauri::command]
async fn reject_review(revid: String) -> String{
    let reject_rev = functions::reject_review(revid).await;

    reject_rev
}

#[tauri::command]
async fn all_estimates() -> atstypes::EstOutInfo {
    let estimates = functions::all_estimates().await;

    estimates
}

#[tauri::command]
async fn complete_estimate(estid: String) -> String {
    let completed = functions::complete_estimate(estid).await;

    completed
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sign_in,
            all_reviews,
            accept_review,
            reject_review,
            all_estimates,
            complete_estimate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
