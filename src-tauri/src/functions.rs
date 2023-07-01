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

pub async fn get_all_reviews() -> Vec<atstypes::RevOutInfo> {
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);

    let response = client
        .from("reviews")
        .eq("jailed", "Yes")
        .select("*")
        .execute()
        .await;

    let body = response
        .expect("Unable to unwrap reviews")
        .text()
        .await
        .unwrap();

    println!("body: {}", body.clone());

    let reviews: Vec<atstypes::RevOutInfo> = serde_json::from_str(body.as_str()).unwrap();

    reviews
}

pub async fn accept_review(revid: String) -> String {
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);

    let _response = client
        .from("reviews")
        .eq("revid", revid)
        .update("{\"jailed\": \"No\"}")
        .update("{\"accept\": \"Yes\"}")
        .update("{\"reject\": \"No\"}")
        .execute()
        .await
        .unwrap();

    String::from("review has been updated")
}

pub async fn reject_review(revid: String) -> String {
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);

    let _response = client
        .from("reviews")
        .eq("revid", revid)
        .update("{\"jailed\": \"No\"}")
        .update("{\"reject\": \"Yes\"}")
        .update("{\"accept\": \"No\"}")
        .execute()
        .await
        .unwrap();

    String::from("review has been updated")
}

pub async fn all_estimates() -> Vec<atstypes::EstOutInfo> {
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);

    let response = client
        .from("estimates")
        .eq("completed", "No")
        .select("*")
        .execute()
        .await;

    let body = response
        .expect("Unable to unwrap estimates")
        .text()
        .await
        .unwrap();

    println!("est body: {}", body.clone());

    let estimates: Vec<atstypes::EstOutInfo> = serde_json::from_str(body.as_str()).unwrap();

    estimates
}

pub async fn complete_estimate(estid: String) -> String{
    let api_key = dotenv::var("ATS_APIKEY").unwrap();
    let client = Postgrest::new("https://yluhibvgdlzknijsssbn.supabase.co/rest/v1")
        .insert_header("apikey", api_key);

    let _response = client
        .from("estimates")
        .eq("estid", estid)
        .update("{\"completed\": \"Yes\"}")
        .execute()
        .await
        .unwrap();

    String::from("estimates have been updated")
}