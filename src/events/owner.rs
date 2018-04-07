use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use serde_json;
use diesel::pg::PgConnection;

event!(accept_owner_invite(data, output, db) {
    match data["sender"].as_str().unwrap() {
        "Luna Frisch" => {
            let mut response = ::fchat::message::InviteResponseBuilder::new()
                .channel(data["name"].as_str().unwrap())
                .build();

            output.send(response)
                .expect("Failed to queue InviteResponseBuilder.");
        },
        "Zakura Ikinaru" => {
            let mut response = ::fchat::message::InviteResponseBuilder::new()
                .channel(data["name"].as_str().unwrap())
                .build();

            output.send(response)
                .expect("Failed to queue InviteResponseBuilder.");
        },
        _ => {},
    }
});