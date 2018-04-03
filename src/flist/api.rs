use serde_json;

pub struct ApiTicket {
    account: String,
    password: String,
    no_characters: String,
    no_friends: String,
    no_bookmarks: String,
}

impl ApiTicket {
    pub fn new() -> Self {
        ApiTicket {
            account: "".to_string(),
            password: "".to_string(),
            no_characters: "true".to_string(),
            no_friends: "true".to_string(),
            no_bookmarks: "true".to_string(),
        }
    }

    pub fn account<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.account = s.to_string();
        self
    }

    pub fn password<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.password = s.to_string();
        self
    }

    pub fn acquire(&self) -> String {
        use reqwest;

        let payload = [("account", self.account.as_str()), ("password", self.password.as_str()), ("no_characters", &self.no_characters.as_str()), ("no_friends", &self.no_friends.as_str()), ("no_bookmarks", self.no_bookmarks.as_str())];
        let r_client = reqwest::Client::new();
        let r_result = r_client.post("https://www.f-list.net/json/getApiTicket.php")
            .form(&payload)
            .send();

        let s_value: serde_json::Value = serde_json::from_str(r_result.unwrap().text().unwrap().as_str()).unwrap();
        s_value["ticket"].as_str().unwrap().to_string()
    }
}