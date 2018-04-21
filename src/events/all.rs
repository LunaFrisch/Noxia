use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use serde_json;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use ::schema::profiles;
use ::schema::profiles::dsl::*;

event!(add_new_user_on_join(data, output, db) {
    use diesel;
    use diesel::dsl::max;

    use ::models::Profile as Profile;
    use ::models::NewProfile as NewProfile;

    let character = data["character"]["identity"].as_str().unwrap();
    let profile_search_result = profiles.filter(profiles::dsl::name.eq(character))
        .limit(1)
        .load::<Profile>(&*db.lock().unwrap())
        .expect("Error loading profile from database.");

    if profile_search_result.len() == 0 {
        let current_uid_max: Option<i32> = profiles.select(max(userid)).first(&*db.lock().unwrap()).unwrap_or(None);
        let mut new_id: i32;

        let new_profile = match current_uid_max {
            Some(data) => {
                new_id = data+1;
                NewProfile {
                    userid: &new_id,
                    name: character,
                    role: "customer",
                    favor: &0,
                    totalclients: &0,
                    totalvisits: &0,
                }
            },
            None => {
                NewProfile {
                    userid: &0,
                    name: character,
                    role: "customer",
                    favor: &0,
                    totalclients: &0,
                    totalvisits: &0,
                }
            },
        };

        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(&*db.lock().unwrap())
            .expect("Error inserting new profile.");
    }
});