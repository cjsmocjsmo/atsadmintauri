use postgrest::Postgrest;
use dotenv;
use crate::atstypes;

pub async fn get_admin(email: String, password: String, bar: String) -> bool {
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);
    

    let response = client
        .from("admin")
        .eq("password", password.clone())
        .eq("email", email.clone())
        .select("*")
        .single()
        .execute()
        .await;

    let body = response
        .expect("fuck")
        .text()
        .await
        .unwrap();

    let admin: atstypes::Login = serde_json::from_str(body.as_str()).unwrap();
    let mut signin = true;
    println!("admin.token {}", admin.token.clone());
    println!("bar: {}", bar.clone());
    // if !&admin.token.to_string().eq(&bar) {
    //     signin = false;
    //     println!("token is false");
    //     return signin;
    // }
    if !admin.email.to_string().eq(&email) {
        signin = false;
        println!("email is false");
        return signin;
    }
    if !admin.password.to_string().eq(&password) {
        signin = false;
        println!("password if false");
        return signin;
    }


    signin
}