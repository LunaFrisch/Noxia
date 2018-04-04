use std::sync::mpsc::Sender;

use serde_json;

command!(profile(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(feedback(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(gelbooru(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(rule_thirty_four(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(e_six_twenty_one(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(yandere(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(urban(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});

command!(lmgtfy(data, output) {
    let msg = ::fchat::message::MessageBuilder::new()
        .channel(data["channel"].as_str().unwrap())
        .push(r"Sorry, but the bot is currently in maintenance mode, while [user]Luna Frisch[/user] finishes tweaking the completely overhauled source.")
        .build();

    output.send(msg)
        .expect("Failed to queue message.");
});