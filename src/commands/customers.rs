use std::sync::mpsc::Sender;

use serde_json;

command!(list(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    println!("{}", msg);
    output.send(msg)
        .expect("Failed to queue message.");
});

command!(offer(data, output) {
    let msg = ::fchat::message::PrivateMessageBuilder::new()
        .recipient(data["character"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});