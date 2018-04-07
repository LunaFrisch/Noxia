use ::schema::profiles;
use ::schema::packages;
use ::schema::roles;
use ::schema::purchases;

#[derive(Queryable, Identifiable, AsChangeset)]
#[primary_key(userid)]
#[table_name="profiles"]
pub struct Profile {
    pub userid: i32,
    pub name: String,
    pub role: String,
    pub favor: i32,
    pub totalclients: i32,
    pub totalvisits: i32,
}

#[derive(Insertable)]
#[table_name="profiles"]
pub struct NewProfile<'a> {
    pub userid: &'a i32,
    pub name: &'a str,
    pub role: &'a str,
    pub favor: &'a i32,
    pub totalclients: &'a i32,
    pub totalvisits: &'a i32,
}

#[derive(Queryable, Identifiable, Clone)]
#[primary_key(userid)]
#[table_name="packages"]
pub struct UserPackages {
    pub userid: i32,
    pub packagelist: String,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewUserPackages<'a> {
    pub userid: &'a i32,
    pub packagelist: &'a str,
}

#[derive(Queryable)]
//#[primary_key(rolename)]
//#[table_name="roles"]
pub struct Role {
    pub rolename: String,
    pub tip: bool,
    pub clockin: bool,
    pub clockout: bool,
    pub confirm: bool,
    pub deny: bool,
    pub list: bool,
    pub offer: bool,
    pub feedback: bool,
    pub gelbooru: bool,
    pub rule34: bool,
    pub e621: bool,
    pub yandere: bool,
    pub urban: bool,
    pub lmgtfy: bool,
    pub checkfavor: bool,
    pub setrole: bool,
    pub setworker: bool,
    pub removeworker: bool,
    pub blacklist: bool,
    pub say: bool,
    pub checkfeedback: bool,
    pub setpackage: bool,
    pub profile: bool,
}

#[derive(Queryable, Identifiable)]
#[primary_key(userid)]
#[table_name="purchases"]
pub struct UserPurchases {
    pub userid: i32,
    pub gelbooru: bool,
    pub rule34: bool,
    pub e621: bool,
    pub yandere: bool,
    pub lmgtfy: bool,
}

#[derive(Insertable)]
#[table_name="purchases"]
pub struct NewUserPurchases<'a> {
    pub userid: &'a i32,
    pub gelbooru: &'a bool,
    pub rule34: &'a bool,
    pub e621: &'a bool,
    pub yandere: &'a bool,
    pub lmgtfy: &'a bool,
}