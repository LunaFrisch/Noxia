use std::sync::mpsc::Sender;

use serde_json;

event!(accept_owner_invite(data, output) {
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