#![feature(core_intrinsics)]

extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate tungstenite;
extern crate url;
extern crate threadpool;
extern crate regex;
extern crate dotenv;
#[macro_use] extern crate diesel;

use std::env;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod models;
mod schema;
mod utility;
mod flist;
#[macro_use] mod fchat;
mod events;
mod commands;

fn main() {
    dotenv().ok();

    let _ = ::fchat::client::ThreadedClient::new()
        .account(env::var("ACCOUNT").unwrap().as_str())
        .password(env::var("PASSWORD").unwrap().as_str())
        .character(env::var("CHARACTER").unwrap().as_str())
        .server(env::var("SERVER").unwrap().as_str())
        .prefix(env::var("PREFIX").unwrap().as_str())
        .database(PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str()).expect(&format!("Error connecting to database.")))
        .register_event(::events::owner::accept_owner_invite, ::fchat::event::OnEvent::ChannelInvite)
        .register_event(::events::all::add_new_user_on_join, ::fchat::event::OnEvent::ChannelJoin)
        .register_command(::commands::owner::set_role, ::fchat::command::OnEvent::ChannelMessage, "set_role")
        .register_command(::commands::owner::check_favor, ::fchat::command::OnEvent::PrivateMessage, "check_favor")
        .register_command(::commands::currency::tip, ::fchat::command::OnEvent::ChannelMessage, "tip")
        .register_command(::commands::currency::favor, ::fchat::command::OnEvent::ChannelMessage, "favor")
        .register_command(::commands::workers::clockin, ::fchat::command::OnEvent::PrivateMessage, "clockin")
        .register_command(::commands::workers::clockout, ::fchat::command::OnEvent::PrivateMessage, "clockout")
        .register_command(::commands::workers::confirm, ::fchat::command::OnEvent::PrivateMessage, "confirm")
        .register_command(::commands::workers::deny, ::fchat::command::OnEvent::PrivateMessage, "deny")
        .register_command(::commands::customers::list, ::fchat::command::OnEvent::PrivateMessage, "list")
        .register_command(::commands::customers::offer, ::fchat::command::OnEvent::PrivateMessage, "offer")
        .run();
}
