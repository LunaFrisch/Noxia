use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use diesel::pg::PgConnection;

use serde_json;

command!(set_role(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(check_favor(data, output, db) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(blacklist(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(check_feedback(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(say(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(set_worker(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(remove_worker(data, output, db) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});