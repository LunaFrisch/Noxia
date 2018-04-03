use std::sync::mpsc::Sender;

use serde_json;

command!(clockin(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(clockout(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(confirm(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(deny(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});