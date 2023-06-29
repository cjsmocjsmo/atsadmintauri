use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EstInInfo {
    pub name: String,
    pub addr: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub reqservdate: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstOutInfo {
    pub id: String,
    pub acctid: String,
    pub estid: String,
    pub name: String,
    pub addr: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub reqservdate: String,
    pub comment: String,
    pub completed: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RevInInfo {
    pub name: String,
    pub email: String,
    pub stars: String,
    pub review: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevOutInfo {
    pub id: String,
    pub acctid: String,
    pub revid: String,
    // pub name: String,
    pub email: String,
    pub stars: String,
    pub review: String,
    pub reject: String,
    pub accept: String,
    pub jailed: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub acct_name: String,
    pub acct_id: String,
    pub creation_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstData {
    pub estid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevData {
    pub revid: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logout {
    pub token: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Obj {
    pub admin: Vec<AdminData>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AdminData {
    pub name: String,
    pub password: String,
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomClaim {
    pub email: String
}