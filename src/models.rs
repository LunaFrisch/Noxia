#[derive(Queryable, Identifiable)]
#[primary_key(UserId)]
#[table_name="Profiles"]
pub struct Profile {
    pub UserId: i32,
    pub Name: String,
    pub Role: String,
    pub Favor: i32,
    pub TotalClients: i32,
    pub TotalVisits: i32,
}

#[derive(Insertable)]
#[table_name="Profiles"]
pub struct NewProfile<'a> {
    pub UserId: &'a i32,
    pub Name: &'a str,
    pub Role: &'a str,
    pub Favor: &'a i32,
    pub TotalClients: &'a i32,
    pub TotalVisits: &'a i32,
}

#[derive(Queryable, Identifiable)]
#[primary_key(UserId)]
#[table_name="Packages"]
pub struct UserPackages {
    pub UserId: i32,
    pub PackageList: String,
}

#[derive(Insertable)]
#[table_name="Packages"]
pub struct NewUserPackages<'a> {
    pub UserId: &'a i32,
    pub Packages: &'a str,
}

#[derive(Queryable)]
#[primary_key(RoleName)]
#[table_name="Roles"]
pub struct Role {
    pub RoleName: String,
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
#[primary_key(UserId)]
#[table_name="Purchases"]
pub struct UserPurchases {
    pub UserId: i32,
    pub gelbooru: bool,
    pub rule34: bool,
    pub e621: bool,
    pub yandere: bool,
    pub lmgtfy: bool,
}

#[derive(Insertable)]
#[table_name="Purchases"]
pub struct NewUserPurchases<'a> {
    pub UserId: &'a str,
    pub gelbooru: &'a bool,
    pub rule34: &'a bool,
    pub e621: &'a bool,
    pub yandere: &'a bool,
    pub lmgtfy: &'a bool,
}