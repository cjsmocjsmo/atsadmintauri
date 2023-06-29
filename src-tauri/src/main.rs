// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod functions;
pub mod atstypes;

use postgrest::Postgrest;

// #[tauri::command]
// fn sign_in(email: &str, password: &str) -> String {
//     format!("{}_{}", email, password)
// }


// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// #[tauri::command]
// async fn sign_pdate(email: String, password: String) -> String{
//     let admin = functions::get_admin(email, password);

//     admin.await
// }


#[tauri::command]
async fn sign_in(email: String, password: String, bar: String) -> bool {
    println!("this is data: {:?}", bar);
    let creds = functions::get_admin(email, password, bar).await;
    

    creds
    
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

// #[tauri::command]
// fn accept_review() {
//     println!("Accept review");
// }

// #[tauri::command]
// fn reject_review() {
//     println!("Reject review");
// }

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

// #[tauri::command]
// fn complet_estimate() {
//     println!("Complete estimate");
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // greet, 
            // sign_pdate,
            sign_in,
            all_reviews,
            // accept_review,
            // reject_review,
            all_estimates,
            // complet_estimate

        ])
        // .invoke_handler(tauri::generate_handler![sign_in])
        // .invoke_handler(tauri::generate_handler![all_reviews])
        // .invoke_handler(tauri::generate_handler![accept_review])
        // .invoke_handler(tauri::generate_handler![reject_review])
        // .invoke_handler(tauri::generate_handler![all_estimates])
        // .invoke_handler(tauri::generate_handler![complet_estimate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
